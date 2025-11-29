use crate::dev::nvme_define::NVME_IDENTIFY_CNS_CODES::*;
use crate::dev::nvme_define::*;
use crate::dev::nvme_device::*;
use std::{io, mem::size_of};

impl NVME_COMMAND {
    pub fn opcode(&mut self, opc: u32) -> &mut Self {
        self.CDW0.set_OPC(opc);
        self
    }
    pub fn nsid(&mut self, nsid: u32) -> &mut Self {
        self.NSID = nsid;
        self
    }
    pub fn data(&mut self) -> &mut Self {
        self
    }
    pub fn cdw10(&mut self, value: u32) -> &mut Self {
        self.u.GENERAL.CDW10 = value;
        self
    }
    pub fn cdw11(&mut self, value: u32) -> &mut Self {
        self.u.GENERAL.CDW11 = value;
        self
    }
    pub fn cdw12(&mut self, value: u32) -> &mut Self {
        self.u.GENERAL.CDW12 = value;
        self
    }
    pub fn cdw13(&mut self, value: u32) -> &mut Self {
        self.u.GENERAL.CDW13 = value;
        self
    }
    pub fn cdw14(&mut self, value: u32) -> &mut Self {
        self.u.GENERAL.CDW14 = value;
        self
    }
    pub fn cdw15(&mut self, value: u32) -> &mut Self {
        self.u.GENERAL.CDW15 = value;
        self
    }
    pub fn identify(&mut self, cns: u8) -> &mut Self {
        unsafe { self.u.IDENTIFY.CDW10.set_CNS(cns) };
        self
    }
    pub fn abort(&mut self) -> &mut Self {
        self
    }
    pub fn getfeatures(&mut self) -> &mut Self {
        self
    }
    pub fn setfeatures(&mut self) -> &mut Self {
        self
    }
    pub fn getlogpage(&mut self) -> &mut Self {
        self
    }
    pub fn formatnvm(&mut self) -> &mut Self {
        self
    }
    pub fn sanitize(&mut self) -> &mut Self {
        self
    }
}

impl InboxDriver {
    pub fn nvme_send_vsc2_passthrough_command(
        &self,
        sub_opcode: u32,
        direction: u8,
        p_param_buf: &mut [u8],
        p_data_buf: &mut [u8],
        p_completion_dw0: Option<&mut u32>,
        nsid: u32,
    ) -> io::Result<NVME_COMMAND_STATUS> {
        let mut default_completion_dw0 = 0;
        let completion_dw0 = p_completion_dw0.unwrap_or(&mut default_completion_dw0);

        let mut nc = NVME_COMMAND::default();
        nc.opcode(NvmeVscOpcode::Write as u32)
            .nsid(nsid)
            .cdw10((p_param_buf.len() / size_of::<u32>()) as u32)
            .cdw12(sub_opcode);

        let result = self.nvme_send_passthrough_command(
            NvmeOpcodeType::WRITE as u8,
            &nc,
            p_param_buf,
            completion_dw0,
        );
        let ncs = match result {
            Ok(ncs) => ncs,
            Err(e) => return Err(e),
        };
        if direction == 0
            || ncs.SCT() != NVME_STATUS_TYPES::NVME_STATUS_TYPE_GENERIC_COMMAND as u16
            || ncs.SC() != NVME_STATUS_GENERIC_COMMAND_CODES::NVME_STATUS_SUCCESS_COMPLETION as u16
        {
            return result;
        }

        // Data phase
        nc.opcode(NvmeVscOpcode::None as u32 | direction as u32)
            .cdw10((p_data_buf.len() / size_of::<u32>()) as u32)
            .cdw12(sub_opcode)
            .cdw14(1);

        self.nvme_send_passthrough_command(
            NvmeOpcodeType::NOBUFFER as u8 | direction,
            &nc,
            p_data_buf,
            completion_dw0,
        )
    }

    pub fn nvme_send_vsc_admin_passthrough_command(
        &self,
        p_nc_admin: &NVME_COMMAND,
        p_data_buf: Option<&mut [u8]>,
        p_completion_dw0: Option<&mut u32>,
    ) -> io::Result<NVME_COMMAND_STATUS> {
        let mut direction = (p_nc_admin.CDW0.OPC() as u8) & 3;
        if p_data_buf.is_none() {
            direction = 0;
        }
        let sub_opcode = match direction {
            0 => VS_STD_NVME_CMD_TYPE_NON_DATA, // Adjust based on actual enum or constant
            1 => VS_STD_NVME_CMD_TYPE_WRITE,
            2 => VS_STD_NVME_CMD_TYPE_READ,
            _ => return Err(io::Error::new(io::ErrorKind::Other, "Not Supported")),
        };

        let mut param_buffer = [0u8; NVME_DATA_BUFFER_SIZE];
        let command_bytes = unsafe {
            std::slice::from_raw_parts(
                p_nc_admin as *const NVME_COMMAND as *const u8,
                size_of::<NVME_COMMAND>(),
            )
        };
        param_buffer[..command_bytes.len()].copy_from_slice(command_bytes);

        self.nvme_send_vsc2_passthrough_command(
            sub_opcode,
            direction,
            &mut param_buffer,
            p_data_buf.unwrap_or(&mut []),
            p_completion_dw0,
            0, // Default NSID, adjust if necessary
        )
    }

    pub fn nvme_identify_ns_list(&self, nsid: u32, all: bool) -> io::Result<Vec<u32>> {
        let mut buffer = vec![0u8; 4096];
        let mut nc = NVME_COMMAND::default();
        let mut dw0: u32 = 0;
        let cns = if all {
            NVME_IDENTIFY_CNS_CODES::NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_LIST as u8
        } else {
            NVME_IDENTIFY_CNS_CODES::NVME_IDENTIFY_CNS_ACTIVE_NAMESPACES as u8
        };

        nc.opcode(NVME_ADMIN_COMMANDS::NVME_ADMIN_COMMAND_IDENTIFY as u32)
            .nsid(nsid)
            .identify(cns);
        let ncs =
            self.nvme_send_vsc_admin_passthrough_command(&nc, Some(&mut buffer), Some(&mut dw0))?;

        if ncs.SCT() == NVME_STATUS_TYPES::NVME_STATUS_TYPE_GENERIC_COMMAND as u16
            && ncs.SC() == NVME_STATUS_GENERIC_COMMAND_CODES::NVME_STATUS_SUCCESS_COMPLETION as u16
        {
            let _ns_list: Vec<u32> = buffer
                .chunks_exact(4)
                .map(|chunk| u32::from_le_bytes(chunk.try_into().expect("Chunk size mismatch")))
                .filter(|&value| value != 0)
                .collect();
            return Ok(_ns_list);
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "Not Supported"));
        }
    }

    pub fn nvme_identify_controller(&self) -> io::Result<NVME_IDENTIFY_CONTROLLER_DATA> {
        let result = self.nvme_identify_query(NVME_IDENTIFY_CNS_CONTROLLER as u32, 0);
        match result {
            Ok(data_bytes) => {
                Ok(unsafe { *(data_bytes.as_ptr() as *const NVME_IDENTIFY_CONTROLLER_DATA) })
            }
            Err(err) => Err(err),
        }
    }

    pub fn nvme_identify_namespace(&self, nsid: u32) -> io::Result<NVME_IDENTIFY_NAMESPACE_DATA> {
        let result = self.nvme_identify_query(NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE as u32, nsid);
        match result {
            Ok(data_bytes) => {
                Ok(unsafe { *(data_bytes.as_ptr() as *const NVME_IDENTIFY_NAMESPACE_DATA) })
            }
            Err(err) => Err(err),
        }
    }

    pub fn nvme_getfeature(&self, fid: u32, sel: u32) -> io::Result<u32> {
        let mut cdw10 = NVME_CDW10_GET_FEATURES::default();
        cdw10.set_FID(fid);
        cdw10.set_SEL(sel);
        if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
            Ok(value)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
        }
    }

    pub fn nvme_setfeature(&self, fid: u32, value: u32) -> io::Result<u32> {
        let mut cdw10 = NVME_CDW10_SET_FEATURES::default();
        cdw10.set_FID(fid);
        cdw10.set_SV(0);
        let mut cdw11 = value;
        if fid == 0x6 {
            let mut wce = NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE::new();
            wce.set_WCE(value);
            cdw11 = wce.into();
        }
        if let Ok(value) = self.nvme_set_features(cdw10.into(), cdw11) {
            Ok(value)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
        }
    }
}

// Example Enum Definitions (actual values and types may vary)
#[repr(u8)]
#[derive(Debug)]
pub enum NvmeOpcodeType {
    NOBUFFER,
    WRITE,
    READ,
    READWRITE,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum NvmeVscOpcode {
    None = 0xf0,
    Write = 0xf1,
    Read = 0xf2,
}

impl Default for NvmeVscOpcode {
    fn default() -> Self {
        NvmeVscOpcode::None
    }
}

const NVME_DATA_BUFFER_SIZE: usize = 4096; // Example size, adjust as necessary
const VS_STD_NVME_CMD_TYPE_READ: u32 = 0x83061400;
const VS_STD_NVME_CMD_TYPE_WRITE: u32 = 0x83061401;
const VS_STD_NVME_CMD_TYPE_NON_DATA: u32 = 0x83061402;
