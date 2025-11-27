use endian_codec::EncodeBE;
use log::warn;
use windows_sys::{
    Win32::Foundation::*, Win32::Storage::FileSystem::*, Win32::Storage::IscsiDisc::*,
    Win32::System::Ioctl::*, Win32::System::IO::*,
};

use std::{
    ffi::c_void,
    fmt,
    io::{self, Read, Write},
    mem::{size_of_val, zeroed},
    ptr::{null, null_mut},
};

use super::scsi::*;
use crate::SECTOR_SIZE;

pub fn last_error() -> u32 {
    unsafe { GetLastError() }
}

pub fn open(path: &str, rw: char) -> isize {
    let filename = std::ffi::CString::new(path).unwrap();
    let handle = unsafe {
        CreateFileA(
            filename.as_ptr() as *const u8,
            if rw == 'w' {
                GENERIC_WRITE | GENERIC_READ
            } else {
                GENERIC_READ
            },
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            null_mut(),
            if rw == 'w' && !path.contains("\\\\.\\") {
                CREATE_ALWAYS
            } else {
                OPEN_EXISTING
            },
            FILE_FLAG_NO_BUFFERING | FILE_FLAG_WRITE_THROUGH,
            0,
        )
    };
    handle
}

pub fn ioctl(
    handle: isize,
    control_code: u32,
    in_buffer: Option<(*const c_void, usize)>,
    out_buffer: Option<(*mut c_void, usize)>,
) -> io::Result<usize> {
    let mut bytes_returned = 0u32;
    let (in_buffer, in_buffer_size) = in_buffer.unwrap_or((null(), 0));
    let (out_buffer, out_buffer_size) = out_buffer.unwrap_or((null_mut(), 0));
    let ok = unsafe {
        DeviceIoControl(
            handle,
            control_code,
            in_buffer,
            in_buffer_size as u32,
            out_buffer,
            out_buffer_size as u32,
            &mut bytes_returned,
            null_mut(),
        )
    };
    if ok == 0 {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Error code: {:#08x}", last_error()),
        ))
    } else {
        Ok(bytes_returned as usize)
    }
}

fn geometry(drive: &HANDLE) -> usize {
    let mut geo: DISK_GEOMETRY_EX = unsafe { zeroed() };
    if let Ok(_r) = {
        ioctl(
            *drive,
            IOCTL_DISK_GET_DRIVE_GEOMETRY_EX,
            Some((null_mut(), 0)),
            Some((&mut geo as *mut _ as *mut c_void, size_of_val(&geo))),
        )
    } {
        geo.DiskSize as usize
    } else {
        0 as usize
    }
}

fn getfilesize(drive: &HANDLE) -> usize {
    let mut bytes_returned = 0;
    let r = unsafe { GetFileSizeEx(*drive, &mut bytes_returned) };
    if r == 0 {
        geometry(drive)
    } else {
        bytes_returned as usize
    }
}

pub struct Disk {
    path: String,
    rw: char,
    pub handle: HANDLE,
    pub size: usize,
    pub lba_shift: u8,
    write_offset: u64,
    sptdwb: SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER,
    pub fua: Option<bool>,
}

impl fmt::Debug for Disk {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Disk path: \"{}\", rw: '{}', handle: {}, size: {}, fua: {:?}",
            self.path, self.rw, self.handle, self.size, self.fua,
        )
    }
}

impl Disk {
    pub fn open(path: String, rw: char, fua: Option<bool>) -> Option<Disk> {
        let handle = open(&path, rw);
        if handle == INVALID_HANDLE_VALUE {
            warn!("Can't open file!! '{}'", path);
            None
        } else {
            Some(Disk {
                path,
                rw,
                handle,
                size: getfilesize(&handle),
                lba_shift: 9,
                write_offset: 0,
                sptdwb: SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER::new(SCSI_IOCTL_DATA_OUT),
                fua,
            })
        }
    }

    /// The size of the drive in bytes.
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn scsi_open(&mut self, path: String) {
        unsafe { CloseHandle(self.handle) };
        self.handle = INVALID_HANDLE_VALUE;

        let handle = open(&path, self.rw);
        if handle == INVALID_HANDLE_VALUE {
            warn!("Can't open file!! '{}'", path);
        } else {
            self.handle = handle;
        }
    }

    pub fn get_scsi_address(&self) -> io::Result<u8> {
        let mut scsi_addr: SCSI_ADDRESS = unsafe { zeroed() };
        if let Ok(_r) = ioctl(
            self.handle,
            IOCTL_SCSI_GET_ADDRESS,
            Some((null_mut(), 0)),
            Some((
                &mut scsi_addr as *mut _ as *mut c_void,
                size_of_val(&scsi_addr),
            )),
        ) {
            println!(
                "get_scsi_address: port {} bus {}",
                scsi_addr.PortNumber, scsi_addr.PathId
            );
            Ok(scsi_addr.PathId)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error code: {:#08x}", last_error()),
            ))
        }
    }

    pub fn get_cache_information(&self) {
        let mut cache_info: DISK_CACHE_INFORMATION = unsafe { zeroed() };
        if let Ok(_r) = ioctl(
            self.handle,
            IOCTL_DISK_GET_CACHE_INFORMATION,
            Some((null_mut(), 0)),
            Some((
                &mut cache_info as *mut _ as *mut c_void,
                size_of_val(&cache_info),
            )),
        ) {
            println!("get_cache_information: {:?}", cache_info.WriteCacheEnabled);
        }
    }

    pub fn storage_query_property(&self) {
        let mut spq: STORAGE_PROPERTY_QUERY = unsafe { zeroed() };
        spq.PropertyId = StorageDeviceWriteCacheProperty;
        let mut cache_info: STORAGE_WRITE_CACHE_PROPERTY = unsafe { zeroed() };
        if let Ok(_r) = ioctl(
            self.handle,
            IOCTL_STORAGE_QUERY_PROPERTY,
            Some((&spq as *const _ as *const c_void, size_of_val(&spq))),
            Some((
                &mut cache_info as *mut _ as *mut c_void,
                size_of_val(&cache_info),
            )),
        ) {
            println!("storage_query_property: {}", cache_info.WriteCacheEnabled);
        }
    }

    pub fn scsi_pass_through_direct(&mut self) -> io::Result<usize> {
        ioctl(
            self.handle,
            IOCTL_SCSI_PASS_THROUGH_DIRECT,
            Some((
                &self.sptdwb as *const _ as *const c_void,
                size_of_val(&self.sptdwb),
            )),
            Some((
                &mut self.sptdwb as *mut _ as *mut c_void,
                size_of_val(&self.sptdwb),
            )),
        )
    }

    pub fn security_recv(&mut self, protocol: u8, com_id: u16, buf: &[u8]) -> io::Result<usize> {
        let cdb = ScsiSecCdb12::new(
            ScsiOpcode::SCSI_OPCODE_SECURITY_RECV,
            protocol,
            com_id,
            buf.len() as u32,
        );
        self.sptdwb.set_buffer(SCSI_IOCTL_DATA_IN, buf);
        cdb.encode_as_be_bytes(&mut self.sptdwb.sptd.Cdb);
        self.sptdwb.sptd.CdbLength = 12;

        self.scsi_pass_through_direct()
    }

    pub fn security_send(&mut self, protocol: u8, com_id: u16, buf: &[u8]) -> io::Result<usize> {
        let cdb = ScsiSecCdb12::new(
            ScsiOpcode::SCSI_OPCODE_SECURITY_SEND,
            protocol,
            com_id,
            buf.len() as u32,
        );
        self.sptdwb.set_buffer(SCSI_IOCTL_DATA_OUT, buf);
        cdb.encode_as_be_bytes(&mut self.sptdwb.sptd.Cdb);
        self.sptdwb.sptd.CdbLength = 12;

        self.scsi_pass_through_direct()
    }

    pub fn discovery0(&mut self) -> io::Result<usize> {
        let buff = Box::new(vec![0x0u8; 4096]);
        let res = self.security_recv(0x01, 0x0001, &buff);
        println!("discovery0 {:?}", &buff[..512]);
        res
    }

    pub fn storage_set_property(&self) {
        let mut sps: STORAGE_PROPERTY_SET = unsafe { zeroed() };
        sps.PropertyId = StorageDeviceWriteCacheProperty;
        sps.SetType = PropertyStandardSet;
        let mut cache_info: STORAGE_WRITE_CACHE_PROPERTY = unsafe { zeroed() };

        if let Ok(_r) = ioctl(
            self.handle,
            IOCTL_STORAGE_SET_PROPERTY,
            Some((&sps as *const _ as *const c_void, size_of_val(&sps))),
            Some((
                &mut cache_info as *mut _ as *mut c_void,
                size_of_val(&cache_info),
            )),
        ) {
            println!("storage_set_property: {}", cache_info.WriteCacheEnabled);
        }
    }

    pub fn scsi_read(&mut self, offset: u64, buf: &[u8]) -> io::Result<usize> {
        if buf.len() <= 0 {
            return Ok(0);
        }
        let mut len = buf.len() - 1;
        len += SECTOR_SIZE - (len % SECTOR_SIZE);
        let lba = offset >> self.lba_shift;
        let nlb = (len as u32 >> self.lba_shift) - 1;
        let cdb = ScsiRwCdb16::new(ScsiOpcode::SCSI_OPCODE_READ_16, lba, nlb, 0);
        self.sptdwb.set_buffer(SCSI_IOCTL_DATA_IN, buf);
        cdb.encode_as_be_bytes(&mut self.sptdwb.sptd.Cdb);
        self.sptdwb.sptd.CdbLength = 16;

        self.scsi_pass_through_direct()
    }

    #[allow(unused_assignments)]
    pub fn scsi_write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() <= 0 {
            return Ok(0);
        }
        let mut len = buf.len() - 1;
        len += SECTOR_SIZE - (len % SECTOR_SIZE);
        let lba = self.write_offset >> self.lba_shift;
        let nlb = (len as u32 >> self.lba_shift) - 1;
        let mut flag = 0;
        if let Some(fua) = self.fua {
            flag = (fua as u8) << 3;
        }
        let cdb = ScsiRwCdb16::new(ScsiOpcode::SCSI_OPCODE_WRITE_16, lba, nlb, flag as u8);
        self.sptdwb.set_buffer(SCSI_IOCTL_DATA_OUT, buf);
        cdb.encode_as_be_bytes(&mut self.sptdwb.sptd.Cdb);
        self.sptdwb.sptd.CdbLength = 16;

        let res = self.scsi_pass_through_direct();
        match res {
            Err(_err) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error code: {:#08x}", last_error()),
            )),
            Ok(_wb) => {
                self.write_offset += self.sptdwb.sptd.DataTransferLength as u64;
                Ok(self.sptdwb.sptd.DataTransferLength as usize)
            }
        }
    }
}

impl Read for Disk {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0u32;
        let res = unsafe {
            ReadFile(
                self.handle,
                buf.as_mut_ptr() as *mut u8,
                buf.len() as u32,
                &mut bytes_read,
                null_mut(),
            )
        };
        if res == 0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error code: {:#08x}", last_error()),
            ))
        } else {
            Ok(bytes_read as usize)
        }
    }
}

impl Write for Disk {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() <= 0 {
            return Ok(0);
        }
        let mut len = buf.len() - 1;
        len += SECTOR_SIZE - (len % SECTOR_SIZE);
        let mut bytes_write = 0u32;
        let res = unsafe {
            WriteFile(
                self.handle,
                buf.as_ptr() as *const u8,
                len as u32,
                &mut bytes_write,
                null_mut(),
            )
        };
        if res == 0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error code: {:#08x}", last_error()),
            ))
        } else {
            Ok(bytes_write as usize)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

unsafe impl Send for Disk {}
unsafe impl Sync for Disk {}

pub fn get_physical_drv_number_from_logical_drv(drive_name: String) -> i32 {
    let mut disk_number = -1;
    let path = format!("\\\\.\\{drive_name}");
    let h_device = open(&path, 'r');

    if h_device != INVALID_HANDLE_VALUE {
        let mut st_volume_data: VOLUME_DISK_EXTENTS = unsafe { zeroed() };
        if let Ok(_ret) = ioctl(
            h_device,
            IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS,
            Some((null_mut(), 0)),
            Some((
                &mut st_volume_data as *mut _ as *mut c_void,
                size_of_val(&st_volume_data),
            )),
        ) {
            disk_number = st_volume_data.Extents[0].DiskNumber as i32;
        }
        unsafe { CloseHandle(h_device) };
    }
    disk_number
}
