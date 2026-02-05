use endian_codec::{DecodeBE, EncodeBE, PackedSize};
use memoffset::offset_of;
use std::{
    ffi::c_void,
    fmt,
    mem::size_of,
    ops::{Index, IndexMut},
    ptr::null_mut,
};
use windows_sys::Win32::Storage::IscsiDisc::SCSI_PASS_THROUGH_DIRECT;

// Shouldn't have more than 255 bytes
pub type SenseBuffer = [u8; 32];

// Alignment must match that of the device https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddscsi/ni-ntddscsi-ioctl_scsi_pass_through_direct#remarks
// Since we don't know it we use the max alignment: double DWORD https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddstor/ns-ntddstor-_storage_adapter_descriptor
#[repr(C, align(64))]
pub struct ScsiDataBuffer(Vec<u8>);

#[repr(C)]
#[allow(non_camel_case_types, non_snake_case, unused_assignments)]
pub struct SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER {
    pub sptd: SCSI_PASS_THROUGH_DIRECT,
    pub Filler: u32,
    pub ucSenseBuf: SenseBuffer,
}

impl fmt::Debug for SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl fmt::Display for SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "ScsiPassThrough: ScsiStatus {}, DataTransferLength {}, CDB: opcode {}, flag {}",
            self.sptd.ScsiStatus, self.sptd.DataTransferLength, self.sptd.Cdb[0], self.sptd.Cdb[1],
        )
    }
}

impl SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER {
    pub fn new(dir: u32) -> Self {
        Self {
            sptd: SCSI_PASS_THROUGH_DIRECT {
                Length: size_of::<SCSI_PASS_THROUGH_DIRECT>() as u16,
                ScsiStatus: 0,
                PathId: 0,
                TargetId: 0,
                Lun: 0,
                Cdb: [0; 16],
                CdbLength: 0,
                DataIn: dir as u8,
                DataBuffer: null_mut(),
                DataTransferLength: 0,
                SenseInfoOffset: offset_of!(Self, ucSenseBuf) as u32,
                SenseInfoLength: size_of::<SenseBuffer>() as u8,
                TimeOutValue: 10,
            },
            Filler: 0,
            ucSenseBuf: Default::default(),
        }
    }

    pub fn set_buffer(&mut self, dir: u32, data_src: &[u8]) {
        self.sptd.DataIn = dir as u8;
        self.sptd.DataBuffer = data_src.as_ptr() as *mut c_void;
        self.sptd.DataTransferLength = data_src.len() as u32;
    }
}

impl AsRef<[u8]> for ScsiDataBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Index<usize> for ScsiDataBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for ScsiDataBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum ScsiOpcode {
    SCSI_OPCODE_READ_10 = 0x28,
    SCSI_OPCODE_READ_16 = 0x88,
    SCSI_OPCODE_WRITE_10 = 0x2a,
    SCSI_OPCODE_WRITE_16 = 0x8a,
    SCSI_OPCODE_READ_CAPACITY_10 = 0x25,
    SCSI_OPCODE_SERVICE_ACTION_IN = 0x9e,
    SCSI_SERVICE_ACTION_READ_CAPACITY_16 = 0x10,
    SCSI_OPCODE_TEST_UNIT_READY = 0x00,
    SCSI_OPCODE_SECURITY_RECV = 0xa2,
    SCSI_OPCODE_SECURITY_SEND = 0xb5,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum ScsiCdbFlag {
    SCSI_FL_FUA_NV = 0x02,
    SCSI_FL_FUA = 0x08,
    SCSI_FL_DPO = 0x10,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PackedSize, EncodeBE, DecodeBE)]
pub struct ScsiRwCdb16 {
    pub opcode: u8,
    pub flags: u8,
    pub lba: u64,
    pub len: u32,
    pub group: u8,
    pub control: u8,
}

impl ScsiRwCdb16 {
    pub fn new(opcode: ScsiOpcode, lba: u64, len: u32, flags: u8) -> Self {
        Self {
            opcode: opcode as u8,
            lba,
            len,
            flags,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PackedSize, EncodeBE, DecodeBE)]
pub struct ScsiSecCdb12 {
    pub opcode: u8,
    pub protocol: u8,
    pub com_id: u16,
    pub reserved: u16,
    pub len: u32,
    pub reserved2: u8,
    pub control: u8,
}

impl ScsiSecCdb12 {
    pub fn new(opcode: ScsiOpcode, protocol: u8, com_id: u16, len: u32) -> Self {
        Self {
            opcode: opcode as u8,
            protocol,
            com_id,
            len,
            ..Default::default()
        }
    }
}
