use crate::dev::nvme_define::*;

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
}

pub fn print_nvme_identify_namespace_data(info: &NVME_IDENTIFY_NAMESPACE_DATA) {
    println!("Namespace Size: {}", info.NSZE);
    println!("Namespace Capacity: {}", info.NCAP);
}

pub fn print_nvme_ns_list(ns_list: &Vec<u32>) {
    println!("Namespace List:");
    for ns in ns_list {
        println!("  Namespace ID: {}", ns);
    }
}

pub fn print_nvme_get_feature(fid: u32, value: u32) {
    match fid {
        0x1 => {
            let info = NVME_CDW11_FEATURE_ARBITRATION::from(value);
            println!("Arbitration (FID: {:02x})", fid);
            println!("  Arbitration Burst (AB): {}", info.AB());
            println!("  Low Priority Weight (LPW): {}", info.LPW());
            println!("  Medium Priority Weight (MPW): {}", info.MPW());
            println!("  High Priority Weight (HPW): {}", info.HPW());
        }
        0x2 => {
            let info = NVME_CDW11_FEATURE_POWER_MANAGEMENT::from(value);
            println!("Power Management (FID: {:02x})", fid);
            println!("  Power State : {:X}", info.PS());
        }
        0x3 => {
            let info = NVME_CDW11_FEATURE_LBA_RANGE_TYPE::from(value);
            println!("LBA Range Type (FID: {:02x})", fid);
            println!("  Type 1 Supported: {}", info.NUM());
        }
        0x4 => {
            let info = NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD::from(value);
            println!("Temperature Threshold (FID: {:02x})", fid);
            println!("  Temperature TMPTH: {}", info.TMPTH());
            println!("  Temperature THSEL: {}", info.THSEL());
            println!("  Temperature TMPSEL: {}", info.TMPSEL());
        }
        0x5 => {
            let info = NVME_CDW11_FEATURE_ERROR_RECOVERY::from(value);
            println!("Error Recovery (FID: {:02x})", fid);
            println!("  Time Limited Error Recovery (TLER): {}", info.TLER());
            println!("  Time Limited Error Recovery (TLER): {}", info.DULBE());
        }
        0x6 => {
            let info = NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE::from(value);
            println!("Volatile Write Cache (FID: {:02x})", fid);
            println!("  Write Cache Enabled: {}", info.WCE());
        }
        0x7 => {
            let info = NVME_CDW11_FEATURE_NUMBER_OF_QUEUES::from(value);
            println!("Number of Queues (FID: {:02x})", fid);
            println!("  Number of Submission Queues: {}", info.NSQ());
            println!("  Number of Completion Queues: {}", info.NCQ());
        }
        0x8 => {
            let info = NVME_CDW11_FEATURE_INTERRUPT_COALESCING::from(value);
            println!("Interrupt Coalescing (FID: {:02x})", fid);
            println!("  Aggregate Time Limit: {}", info.TIME());
            println!("  Aggregation Threshold: {}", info.THR());
        }
        0x9 => {
            let info = NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG::from(value);
            println!("Interrupt Vector Configuration (FID: {:02x})", fid);
            println!("  Coalescing Disable: {}", info.CD());
            println!("  Interrupt Vector: {}", info.IV());
        }
        0xA => {
            let info = NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL::from(value);
            println!("Write Atomicity (FID: {:02x})", fid);
            println!("  Write Atomicity: {}", info.DN());
        }
        0xB => {
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
        }
        0xC => {
            let info = NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION::from(value);
            println!("Autonomous Power State Transition (FID: {:02x})", fid);
            println!("  APSTE: {}", info.APSTE());
        }
        0x10 => {
            let info = NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD::from(value);
            println!("Autonomous Power State Transition (FID: {:02x})", fid);
            println!("  APSTE: {}", info.TMPTH());
        }
        _ => {}
    }
}

pub fn print_nvme_set_feature(fid: u32, value: u32) {
    print_nvme_get_feature(fid, value);
}
