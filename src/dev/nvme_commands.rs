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

    pub fn nvme_identify_ns_list(&self, nsid: u32, all: bool) -> io::Result<NVME_COMMAND_STATUS> {
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

        let ns_list: Vec<u32> = buffer
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().expect("Chunk size mismatch")))
            .filter(|&value| value != 0)
            .collect();

        for ns in &ns_list {
            println!("{:?}", ns);
        }
        Ok(ncs)
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
        match fid {
            0x1 => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_ARBITRATION::from(value);
                    println!("Arbitration (FID: {:02x})", fid);
                    println!("  Arbitration Burst (AB): {}", info.AB());
                    println!("  Low Priority Weight (LPW): {}", info.LPW());
                    println!("  Medium Priority Weight (MPW): {}", info.MPW());
                    println!("  High Priority Weight (HPW): {}", info.HPW());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x2 => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_POWER_MANAGEMENT::from(value);
                    println!("Power Management (FID: {:02x})", fid);
                    println!("  Power State : {:X}", info.PS());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x3 => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_LBA_RANGE_TYPE::from(value);
                    println!("LBA Range Type (FID: {:02x})", fid);
                    println!("  Type 1 Supported: {}", info.NUM());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x4 => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD::from(value);
                    println!("Temperature Threshold (FID: {:02x})", fid);
                    println!("  Temperature TMPTH: {}", info.TMPTH());
                    println!("  Temperature THSEL: {}", info.THSEL());
                    println!("  Temperature TMPSEL: {}", info.TMPSEL());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x5 => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_ERROR_RECOVERY::from(value);
                    println!("Error Recovery (FID: {:02x})", fid);
                    println!("  Time Limited Error Recovery (TLER): {}", info.TLER());
                    println!("  Time Limited Error Recovery (TLER): {}", info.DULBE());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x6 => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE::from(value);
                    println!("Volatile Write Cache (FID: {:02x})", fid);
                    println!("  Write Cache Enabled: {}", info.WCE());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x7 => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_NUMBER_OF_QUEUES::from(value);
                    println!("Number of Queues (FID: {:02x})", fid);
                    println!("  Number of Submission Queues: {}", info.NSQ());
                    println!("  Number of Completion Queues: {}", info.NCQ());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x8 => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_INTERRUPT_COALESCING::from(value);
                    println!("Interrupt Coalescing (FID: {:02x})", fid);
                    println!("  Aggregate Time Limit: {}", info.TIME());
                    println!("  Aggregation Threshold: {}", info.THR());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x9 => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG::from(value);
                    println!("Interrupt Vector Configuration (FID: {:02x})", fid);
                    println!("  Coalescing Disable: {}", info.CD());
                    println!("  Interrupt Vector: {}", info.IV());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0xA => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL::from(value);
                    println!("Write Atomicity (FID: {:02x})", fid);
                    println!("  Write Atomicity: {}", info.DN());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0xB => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG::from(value);
                    println!("Asynchronous Event Configuration (FID: {:02x})", fid);
                    println!("  Critical Warnings: {}", info.CriticalWarnings());
                    println!("  Namespace Attributes: {}", info.NsAttributeNotices());
                    println!("  Firmware Activation: {}", info.FwActivationNotices());
                    println!("  Telemetry Log: {}", info.TelemetryLogNotices());
                    println!("  ANA Change: {}", info.ANAChangeNotices());
                    println!("  Predictable Log: {}", info.PredictableLogChangeNotices());
                    println!("  LBA Status Information: {}", info.LBAStatusNotices());
                    println!("  Endurance Event Log: {}", info.EnduranceEventNotices());
                    println!("  Zone Descriptor: {}", info.ZoneDescriptorNotices());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0xC => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION::from(value);
                    println!("Autonomous Power State Transition (FID: {:02x})", fid);
                    println!("  APSTE: {}", info.APSTE());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x10 => {
                if let Ok(value) = self.nvme_getfeature_query(cdw10.into(), 0) {
                    let info = NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD::from(value);
                    println!("Autonomous Power State Transition (FID: {:02x})", fid);
                    println!("  APSTE: {}", info.TMPTH());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            _ => Err(io::Error::new(io::ErrorKind::Other, "Not Supported")),
        }
    }

    pub fn nvme_setfeature(&self, fid: u32, value: u32) -> io::Result<u32> {
        let mut cdw10 = NVME_CDW10_SET_FEATURES::default();
        cdw10.set_FID(fid);
        cdw10.set_SV(0);
        match fid {
            0x1 => {
                if let Ok(value) = self.nvme_set_features(cdw10.into(), value) {
                    let info = NVME_CDW11_FEATURE_ARBITRATION::from(value);
                    println!("Arbitration (FID: {:02x})", fid);
                    println!("  Arbitration Burst (AB): {}", info.AB());
                    println!("  Low Priority Weight (LPW): {}", info.LPW());
                    println!("  Medium Priority Weight (MPW): {}", info.MPW());
                    println!("  High Priority Weight (HPW): {}", info.HPW());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x2 => {
                if let Ok(value) = self.nvme_set_features(cdw10.into(), value) {
                    let info = NVME_CDW11_FEATURE_POWER_MANAGEMENT::from(value);
                    println!("Power Management (FID: {:02x})", fid);
                    println!("  Power State : {:X}", info.PS());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x3 => {
                if let Ok(value) = self.nvme_set_features(cdw10.into(), value) {
                    let info = NVME_CDW11_FEATURE_LBA_RANGE_TYPE::from(value);
                    println!("LBA Range Type (FID: {:02x})", fid);
                    println!("  Type 1 Supported: {}", info.NUM());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x4 => {
                if let Ok(value) = self.nvme_set_features(cdw10.into(), value) {
                    let info = NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD::from(value);
                    println!("Temperature Threshold (FID: {:02x})", fid);
                    println!("  Temperature TMPTH: {}", info.TMPTH());
                    println!("  Temperature THSEL: {}", info.THSEL());
                    println!("  Temperature TMPSEL: {}", info.TMPSEL());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x5 => {
                if let Ok(value) = self.nvme_set_features(cdw10.into(), value) {
                    let info = NVME_CDW11_FEATURE_ERROR_RECOVERY::from(value);
                    println!("Error Recovery (FID: {:02x})", fid);
                    println!("  Time Limited Error Recovery (TLER): {}", info.TLER());
                    println!("  Time Limited Error Recovery (TLER): {}", info.DULBE());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x6 => {
                let mut cdw11 = NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE::new();
                cdw11.set_WCE(value);
                if let Ok(value) = self.nvme_set_features(cdw10.into(), cdw11.into()) {
                    let info = NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE::from(value);
                    println!("Volatile Write Cache (FID: {:02x})", fid);
                    println!("  Write Cache Enabled: {}", info.WCE());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x7 => {
                if let Ok(value) = self.nvme_set_features(cdw10.into(), value) {
                    let info = NVME_CDW11_FEATURE_NUMBER_OF_QUEUES::from(value);
                    println!("Number of Queues (FID: {:02x})", fid);
                    println!("  Number of Submission Queues: {}", info.NSQ());
                    println!("  Number of Completion Queues: {}", info.NCQ());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x8 => {
                if let Ok(value) = self.nvme_set_features(cdw10.into(), value) {
                    let info = NVME_CDW11_FEATURE_INTERRUPT_COALESCING::from(value);
                    println!("Interrupt Coalescing (FID: {:02x})", fid);
                    println!("  Aggregate Time Limit: {}", info.TIME());
                    println!("  Aggregation Threshold: {}", info.THR());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x9 => {
                if let Ok(value) = self.nvme_set_features(cdw10.into(), value) {
                    let info = NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG::from(value);
                    println!("Interrupt Vector Configuration (FID: {:02x})", fid);
                    println!("  Coalescing Disable: {}", info.CD());
                    println!("  Interrupt Vector: {}", info.IV());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0xA => {
                if let Ok(value) = self.nvme_set_features(cdw10.into(), value) {
                    let info = NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL::from(value);
                    println!("Write Atomicity (FID: {:02x})", fid);
                    println!("  Write Atomicity: {}", info.DN());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0xB => {
                if let Ok(value) = self.nvme_set_features(cdw10.into(), value) {
                    let info = NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG::from(value);
                    println!("Asynchronous Event Configuration (FID: {:02x})", fid);
                    println!("  Critical Warnings: {}", info.CriticalWarnings());
                    println!("  Namespace Attributes: {}", info.NsAttributeNotices());
                    println!("  Firmware Activation: {}", info.FwActivationNotices());
                    println!("  Telemetry Log: {}", info.TelemetryLogNotices());
                    println!("  ANA Change: {}", info.ANAChangeNotices());
                    println!("  Predictable Log: {}", info.PredictableLogChangeNotices());
                    println!("  LBA Status Information: {}", info.LBAStatusNotices());
                    println!("  Endurance Event Log: {}", info.EnduranceEventNotices());
                    println!("  Zone Descriptor: {}", info.ZoneDescriptorNotices());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0xC => {
                if let Ok(value) = self.nvme_set_features(cdw10.into(), value) {
                    let info = NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION::from(value);
                    println!("Autonomous Power State Transition (FID: {:02x})", fid);
                    println!("  APSTE: {}", info.APSTE());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            0x10 => {
                if let Ok(value) = self.nvme_set_features(cdw10.into(), value) {
                    let info = NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD::from(value);
                    println!("Autonomous Power State Transition (FID: {:02x})", fid);
                    println!("  APSTE: {}", info.TMPTH());
                    Ok(value)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Not Supported"))
                }
            }
            _ => Err(io::Error::new(io::ErrorKind::Other, "Not Supported")),
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

pub fn print_nvme_identify_controller_data(data: &NVME_IDENTIFY_CONTROLLER_DATA) {
    println!("{:<12} : 0x{:04X}", "vid", data.VID);
    println!("{:<12} : 0x{:04X}", "ssvid", data.SSVID);
    println!("{:<12} : {}", "sn", String::from_utf8_lossy(&data.SN));
    println!("{:<12} : {}", "mn", String::from_utf8_lossy(&data.MN));
    println!("{:<12} : {}", "fr", String::from_utf8_lossy(&data.FR));
    println!("{:<12} : {}", "rab", data.RAB);
    println!("{:<12} : {:?}", "ieee", &data.IEEE);
    println!("{:<12} : {:?}", "cmic", data.CMIC);
    println!("{:<12} : {}", "mdts", data.MDTS);
    println!("{:<12} : {}", "cntlid", data.CNTLID);
    println!("{:<12} : 0x{:08X}", "ver", data.VER);
    println!("{:<12} : {}", "rtd3r", data.RTD3R);
    println!("{:<12} : {}", "rtd3e", data.RTD3E);
    println!("{:<12} : {:?}", "oaes", data.OAES);
    println!("{:<12} : {:?}", "ctratt", data.CTRATT);
    println!("{:<12} : {:?}", "rrls", data.RRLS);
    println!("{:<12} : {}", "cntltype", data.CNTRLTYPE);
    println!("{:<12} : {:?}", "fguid", &data.FGUID);
    println!("{:<12} : {}", "crdt1", data.CRDT1);
    println!("{:<12} : {}", "crdt2", data.CRDT2);
    println!("{:<12} : {}", "crdt3", data.CRDT3);
    println!("{:<12} : {:?}", "oacs", data.OACS);
    println!("{:<12} : {}", "acl", data.ACL);
    println!("{:<12} : {}", "aerl", data.AERL);
    println!("{:<12} : {:?}", "frmw", data.FRMW);
    println!("{:<12} : {:?}", "lpa", data.LPA);
    println!("{:<12} : {}", "elpe", data.ELPE);
    println!("{:<12} : {}", "npss", data.NPSS);
    println!("{:<12} : {:?}", "avscp", data.AVSCC);
    println!("{:<12} : {:?}", "apsta", data.APSTA);
    println!("{:<12} : {}", "wctemp", data.WCTEMP);
    println!("{:<12} : {}", "cctemp", data.CCTEMP);
    println!("{:<12} : {}", "mtfa", data.MTFA);
    println!("{:<12} : {}", "hmpre", data.HMPRE);
    println!("{:<12} : {}", "hmmin", data.HMMIN);
    println!("{:<12} : {:?}", "tnvmcap", &data.TNVMCAP);
    println!("{:<12} : {:?}", "unvmcap", &data.UNVMCAP);
    println!("{:<12} : {:?}", "rpmbs", data.RPMBS);
    println!("{:<12} : {}", "edstt", data.EDSTT);
    println!("{:<12} : {}", "dsto", data.DSTO);
    println!("{:<12} : {}", "fwug", data.FWUG);
    println!("{:<12} : {}", "kas", data.KAS);
    println!("{:<12} : {:?}", "hctma", data.HCTMA);
    println!("{:<12} : {}", "mntmt", data.MNTMT);
    println!("{:<12} : {}", "mxtmt", data.MXTMT);
    println!("{:<12} : {:?}", "sanicap", data.SANICAP);
    println!("{:<12} : {}", "hmminds", data.HMMINDS);
    println!("{:<12} : {}", "hmmaxd", data.HMMAXD);
    println!("{:<12} : {}", "nsetidmax", data.NSETIDMAX);
    println!("{:<12} : {}", "endgidmax", data.ENDGIDMAX);
    println!("{:<12} : {}", "anatt", data.ANATT);
    println!("{:<12} : {:?}", "anacap", data.ANACAP);
    println!("{:<12} : {}", "anagrpmax", data.ANAGRPMAX);
    println!("{:<12} : {}", "nanagrpid", data.NANAGRPID);
    println!("{:<12} : {}", "pels", data.PELS);
    println!("{:<12} : {:?}", "sqes", data.SQES);
    println!("{:<12} : {:?}", "cqes", data.CQES);
    println!("{:<12} : {}", "maxcmd", data.MAXCMD);
    println!("{:<12} : {}", "nn", data.NN);
    println!("{:<12} : {:?}", "oncs", data.ONCS);
    println!("{:<12} : {:?}", "fuses", data.FUSES);
    println!("{:<12} : {:?}", "fna", data.FNA);
    println!("{:<12} : {:?}", "vwc", data.VWC);
    println!("{:<12} : {}", "awun", data.AWUN);
    println!("{:<12} : {}", "awupf", data.AWUPF);
    println!("{:<12} : {:?}", "nvscss", data.NVSCC);
    println!("{:<12} : {:?}", "nwpc", data.NWPC);
    println!("{:<12} : {}", "acwu", data.ACWU);
    println!("{:<12} : {:?}", "sgls", data.SGLS);
    println!("{:<12} : {}", "mnan", data.MNAN);
    println!(
        "{:<12} : {}",
        "subnqn",
        String::from_utf8_lossy(&data.SUBNQN)
    );
    // Power State Descriptors are not printed here for brevity.
    // Vendor Specific fields are also not printed here for brevity.
}

pub fn print_nvme_identify_namespace_data(info: &NVME_IDENTIFY_NAMESPACE_DATA) {
    println!("Namespace Size: {}", info.NSZE);
    println!("Namespace Capacity: {}", info.NCAP);
    // Add more fields as necessary
}
