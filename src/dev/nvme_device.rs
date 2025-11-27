use crate::dev::disk::open;
use crate::dev::nvme_define::{NVME_FEATURES::*, NVME_IDENTIFY_CNS_CODES::*, NVME_LOG_PAGES::*, *};
use std::mem::offset_of;
use std::{ffi::c_void, io, mem::size_of, ptr::null_mut};
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Ioctl::*;
use windows_sys::Win32::System::IO::DeviceIoControl;

trait StorageProtocolCommand {
    fn new(&mut self) -> &mut Self;
    fn nvme_command(&mut self, command: &NVME_COMMAND) -> &mut Self;
    fn set_data_in(&mut self, direction: u8, data: &[u8]) -> &mut Self;
    fn get_data(&mut self, data: &mut [u8]) -> &mut Self;
}

impl StorageProtocolCommand for STORAGE_PROTOCOL_COMMAND {
    fn new(&mut self) -> &mut Self {
        self.Version = STORAGE_PROTOCOL_STRUCTURE_VERSION;
        self.Length = size_of::<STORAGE_PROTOCOL_COMMAND>() as u32;
        self.ProtocolType = ProtocolTypeNvme as i32;
        self.Flags = STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST;
        self.CommandLength = STORAGE_PROTOCOL_COMMAND_LENGTH_NVME;
        self.TimeOutValue = 30;
        self
    }
    fn nvme_command(&mut self, command: &NVME_COMMAND) -> &mut Self {
        let command_offset = offset_of!(STORAGE_PROTOCOL_COMMAND, Command);
        let command_size = size_of::<NVME_COMMAND>();
        let buffer = self as *mut _ as *mut u8;
        unsafe {
            std::ptr::copy_nonoverlapping(
                command as *const _ as *const u8,
                buffer.add(command_offset),
                command_size,
            );
        };
        self.ErrorInfoOffset = (command_offset + command_size) as u32;
        self.CommandSpecific = STORAGE_PROTOCOL_SPECIFIC_NVME_ADMIN_COMMAND;
        self
    }
    fn set_data_in(&mut self, direction: u8, data: &[u8]) -> &mut Self {
        match direction {
            1 => self.DataToDeviceTransferLength = data.len() as u32,
            2 => self.DataFromDeviceTransferLength = data.len() as u32,
            _ => {}
        }
        self.DataToDeviceBufferOffset = self.ErrorInfoOffset + self.ErrorInfoLength;
        self.DataFromDeviceBufferOffset =
            self.DataToDeviceBufferOffset + self.DataToDeviceTransferLength;
        if direction == 1 && !data.is_empty() {
            let data_offset = self.DataToDeviceBufferOffset as usize;
            let buffer = self as *mut _ as *mut u8;
            let buffer_slice =
                unsafe { std::slice::from_raw_parts_mut(buffer, data_offset + data.len()) };
            buffer_slice[data_offset..data_offset + data.len()].copy_from_slice(data);
        }
        self
    }
    fn get_data(&mut self, data: &mut [u8]) -> &mut Self {
        if !data.is_empty() {
            let data_len = self.DataFromDeviceTransferLength as usize;
            let data_offset = self.DataFromDeviceBufferOffset as usize;
            let buffer = unsafe {
                std::slice::from_raw_parts_mut(self as *mut _ as *mut u8, data_offset + data_len)
            };
            data.copy_from_slice(&buffer[data_offset..data_offset + data_len]);
        }
        self
    }
}

trait StorageProtocolSpecificData {
    fn new(data_type: i32, data_value: u32, sub_value: u32, length: usize) -> Self;
    fn is_valid(&self, length: usize) -> bool;
    fn get_data(&self) -> &[u8];
}

impl StorageProtocolSpecificData for STORAGE_PROTOCOL_SPECIFIC_DATA {
    fn new(data_type: i32, data_value: u32, sub_value: u32, length: usize) -> Self {
        let offset = if length > 0 {
            size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32
        } else {
            0
        };
        STORAGE_PROTOCOL_SPECIFIC_DATA {
            ProtocolType: ProtocolTypeNvme as i32,
            ProtocolDataOffset: offset,
            DataType: data_type as u32,
            ProtocolDataRequestValue: data_value,
            ProtocolDataRequestSubValue: sub_value,
            ProtocolDataRequestSubValue2: 0,
            ProtocolDataRequestSubValue3: 0,
            ProtocolDataRequestSubValue4: 0,
            FixedProtocolReturnData: 0,
            ProtocolDataLength: length as u32,
        }
    }
    fn is_valid(&self, length: usize) -> bool {
        self.ProtocolDataOffset >= size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() as u32
            && self.ProtocolDataLength >= length as u32
    }
    fn get_data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const _ as *const u8).add(self.ProtocolDataOffset as usize),
                self.ProtocolDataLength as usize,
            )
        }
    }
}

impl StorageProtocolSpecificData for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    fn new(data_type: i32, data_value: u32, sub_value: u32, length: usize) -> Self {
        STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
            ProtocolType: ProtocolTypeNvme as i32,
            ProtocolDataOffset: size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA_EXT>() as u32,
            DataType: data_type as u32,
            ProtocolDataValue: data_value,
            ProtocolDataSubValue: sub_value,
            ProtocolDataSubValue2: 0,
            ProtocolDataSubValue3: 0,
            ProtocolDataSubValue4: 0,
            ProtocolDataSubValue5: 0,
            FixedProtocolReturnData: 0,
            ProtocolDataLength: length as u32,
            Reserved: [0; 5],
        }
    }
    fn is_valid(&self, length: usize) -> bool {
        self.ProtocolDataOffset >= size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA_EXT>() as u32
            && self.ProtocolDataLength >= length as u32
    }
    fn get_data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const _ as *const u8).add(self.ProtocolDataOffset as usize),
                self.ProtocolDataLength as usize,
            )
        }
    }
}

// To use FIELD_OFFSET macro equivalent in Rust:
// let offset = field_offset::<SomeType, SomeFieldType>(0 as *const SomeType, |s| &s.some_field);
#[derive(Debug, Clone)]
pub struct InboxDriver {
    handle: HANDLE,
}

impl InboxDriver {
    pub fn open(device_path: &str) -> io::Result<Self> {
        let handle = open(device_path, 'w');
        if handle == INVALID_HANDLE_VALUE {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self { handle })
        }
    }

    pub fn get_handle(&self) -> HANDLE {
        self.handle
    }

    pub fn nvme_send_passthrough_command(
        &self,
        direction: u8,
        nvme_command: &NVME_COMMAND,
        data_buffer: &mut [u8],
        return_dw0: &mut u32,
    ) -> io::Result<NVME_COMMAND_STATUS> {
        let command_offset = offset_of!(STORAGE_PROTOCOL_COMMAND, Command);
        let buffer_size = command_offset + size_of::<NVME_COMMAND>() + data_buffer.len();
        let mut buffer = vec![0; buffer_size];
        let protocol_command =
            unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROTOCOL_COMMAND) };
        protocol_command
            .new()
            .nvme_command(nvme_command)
            .set_data_in(direction, data_buffer);

        let mut returned_length = 0;
        let result = unsafe {
            DeviceIoControl(
                self.handle,
                IOCTL_STORAGE_PROTOCOL_COMMAND,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                &mut returned_length,
                null_mut(),
            )
        };
        if result == 0 {
            return Err(io::Error::last_os_error());
        }

        if direction == 2 {
            protocol_command.get_data(data_buffer);
        }
        *return_dw0 = protocol_command.FixedProtocolReturnData;
        let ncs = NVME_COMMAND_STATUS::from(protocol_command.ErrorCode as u16);
        Ok(ncs)
    }

    pub fn nvme_send_query_command(
        &self,
        property_id: i32,
        protocol_data: &mut STORAGE_PROTOCOL_SPECIFIC_DATA,
    ) -> io::Result<&[u8]> {
        let data_length = protocol_data.ProtocolDataLength as usize;
        let data_offset = offset_of!(STORAGE_PROPERTY_QUERY, AdditionalParameters);
        let query_size = data_offset + size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>() + data_length;
        let mut buffer = vec![0u8; query_size];
        let property_query = unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROPERTY_QUERY) };
        property_query.PropertyId = property_id;
        property_query.QueryType = PropertyStandardQuery;

        let protocol_specific_data_ptr =
            unsafe { buffer.as_mut_ptr().add(data_offset) as *mut STORAGE_PROTOCOL_SPECIFIC_DATA };
        unsafe {
            std::ptr::copy_nonoverlapping(protocol_data, protocol_specific_data_ptr, 1);
        }
        let mut returned_length = 0;
        if unsafe {
            DeviceIoControl(
                self.get_handle(),
                IOCTL_STORAGE_QUERY_PROPERTY,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                &mut returned_length,
                null_mut(),
            )
        } == 0
        {
            return Err(io::Error::last_os_error());
        }

        let data_descriptor =
            unsafe { &*(buffer.as_ptr() as *const STORAGE_PROTOCOL_DATA_DESCRIPTOR) };
        if data_descriptor.Version != size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32
            || data_descriptor.Size != size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Data descriptor header not valid",
            ));
        }

        let protocol_specific_data = &data_descriptor.ProtocolSpecificData;
        unsafe {
            std::ptr::copy_nonoverlapping(protocol_specific_data_ptr, protocol_data, 1);
        }
        Ok(protocol_specific_data.get_data())
    }

    pub fn nvme_identify_query(&self, cns: u32, nsid: u32) -> io::Result<&[u8]> {
        let mut protocol_specific_data = STORAGE_PROTOCOL_SPECIFIC_DATA::new(
            NVMeDataTypeIdentify,
            cns,
            nsid,
            NVME_IDENTIFY_SIZE,
        );
        self.nvme_send_query_command(
            StorageAdapterProtocolSpecificProperty,
            &mut protocol_specific_data,
        )
    }

    pub fn nvme_logpage_query(&self, lid: u32, cdw11: u32) -> io::Result<&[u8]> {
        let mut protocol_specific_data =
            STORAGE_PROTOCOL_SPECIFIC_DATA::new(NVMeDataTypeLogPage, lid, cdw11, NVME_MAX_LOG_SIZE);
        self.nvme_send_query_command(
            StorageDeviceProtocolSpecificProperty,
            &mut protocol_specific_data,
        )
    }

    pub fn nvme_getfeature_query(&self, fid: u32, cdw11: u32) -> io::Result<u32> {
        let mut protocol_specific_data =
            STORAGE_PROTOCOL_SPECIFIC_DATA::new(NVMeDataTypeFeature, fid, cdw11, 0);
        self.nvme_send_query_command(
            StorageDeviceProtocolSpecificProperty,
            &mut protocol_specific_data,
        )
        .map(|_| protocol_specific_data.FixedProtocolReturnData)
    }

    pub fn nvme_send_set_command(
        &self,
        property_id: i32,
        protocol_data: &STORAGE_PROTOCOL_SPECIFIC_DATA_EXT,
    ) -> io::Result<&[u8]> {
        let data_length = protocol_data.ProtocolDataLength as usize;
        let data_offset = offset_of!(STORAGE_PROPERTY_SET, AdditionalParameters);
        let set_size = data_offset + size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA_EXT>() + data_length;
        let mut buffer = vec![0u8; set_size];
        let property_set = unsafe { &mut *(buffer.as_mut_ptr() as *mut STORAGE_PROPERTY_SET) };
        property_set.PropertyId = property_id;
        property_set.SetType = PropertyStandardSet;

        let protocol_specific_data_ptr = unsafe {
            buffer.as_mut_ptr().add(data_offset) as *mut STORAGE_PROTOCOL_SPECIFIC_DATA_EXT
        };
        unsafe {
            std::ptr::copy_nonoverlapping(protocol_data, protocol_specific_data_ptr, 1);
        }
        let mut returned_length = 0;
        if unsafe {
            DeviceIoControl(
                self.get_handle(),
                IOCTL_STORAGE_SET_PROPERTY,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                &mut returned_length,
                null_mut(),
            )
        } == 0
        {
            return Err(io::Error::last_os_error());
        }

        let data_descriptor =
            unsafe { &*(buffer.as_ptr() as *const STORAGE_PROTOCOL_DATA_DESCRIPTOR) };
        if data_descriptor.Version != size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32
            || data_descriptor.Size != size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>() as u32
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Data descriptor header not valid",
            ));
        }

        Ok(data_descriptor.ProtocolSpecificData.get_data())
    }

    pub fn nvme_set_features(&self, fid: u32, cdw11: u32) -> io::Result<u32> {
        let protocol_specific_data = STORAGE_PROTOCOL_SPECIFIC_DATA_EXT::new(
            NVMeDataTypeFeature,
            fid,
            cdw11,
            NVME_MAX_LOG_SIZE,
        );
        self.nvme_send_set_command(
            StorageAdapterProtocolSpecificProperty,
            &protocol_specific_data,
        )
        .map(|_| protocol_specific_data.FixedProtocolReturnData)
    }
}

impl Drop for InboxDriver {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle) };
    }
}
