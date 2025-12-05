#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use modular_bitfield::prelude::*;

pub const NVME_IDENTIFY_SIZE: usize = 4096;

//
// 3.1.1  Offset 00h: CAP (Controller Capabilities)
//
#[derive(Debug, Clone, Copy)]
pub enum NVME_AMS_OPTION {
    NVME_AMS_ROUND_ROBIN = 0,
    NVME_AMS_WEIGHTED_ROUND_ROBIN_URGENT = 1,
}

#[bitfield]
#[repr(u64)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CONTROLLER_CAPABILITIES {
    pub MQES: B16, // RO - Maximum Queue Entries Supported (MQES)
    pub CQR: bool, // RO - Contiguous Queues Required (CQR)

    // Bit 17, 18 - AMS; RO - Arbitration Mechanism Supported (AMS)
    pub AMS_WeightedRoundRobinWithUrgent: bool, // Bit 17: Weighted Round Robin with Urgent;
    pub AMS_VendorSpecific: bool,               // Bit 18: Vendor Specific.

    pub Reserved0: B5, // RO - bit 19 ~ 23
    pub TO: B8,        // RO - Timeout (TO)
    pub DSTRD: B4,     // RO - Doorbell Stride (DSTRD)
    pub NSSRS: bool,   // RO - NVM Subsystem Reset Supported (NSSRS)

    // Bit 37 ~ 44 - CSS; RO - Command Sets Supported (CSS)
    pub CSS_NVM: bool,        // Bit 37: NVM command set
    pub CSS_Reserved0: bool,  // Bit 38: Reserved
    pub CSS_Reserved1: bool,  // Bit 39: Reserved
    pub CSS_Reserved2: bool,  // Bit 40: Reserved
    pub CSS_Reserved3: bool,  // Bit 41: Reserved
    pub CSS_Reserved4: bool,  // Bit 42: Reserved
    pub CSS_MultipleIo: bool, // Bit 43: One or more IO command sets
    pub CSS_AdminOnly: bool,  // Bit 44: Only Admin command set (no IO command set)

    pub Reserved2: B3, // RO - bit 45 ~ 47
    pub MPSMIN: B4,    // RO - Memory Page Size Minimum (MPSMIN)
    pub MPSMAX: B4,    // RO - Memory Page Size Maximum (MPSMAX)
    pub Reserved3: B8, // RO - bit 56 ~ 63
}

//
// 3.1.2  Offset 08h: VS (Version)
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_VERSION {
    //LSB
    pub TER: B8, // Tertiary Version Number (TER)
    pub MNR: B8, // Minor Version Number (MNR)
    pub MJR: B16, // Major Version Number (MJR)
                 //MSB
}

//
// 3.1.5  Offset 14h: CC (Controller Configuration)
//
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NVME_CC_SHN_SHUTDOWN_NOTIFICATIONS {
    NVME_CC_SHN_NO_NOTIFICATION = 0,
    NVME_CC_SHN_NORMAL_SHUTDOWN = 1,
    NVME_CC_SHN_ABRUPT_SHUTDOWN = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NVME_CSS_COMMAND_SETS {
    NVME_CSS_NVM_COMMAND_SET = 0,
    NVME_CSS_ALL_SUPPORTED_IO_COMMAND_SET = 6,
    NVME_CSS_ADMIN_COMMAND_SET_ONLY = 7,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CONTROLLER_CONFIGURATION {
    pub EN: B1,        // RW - Enable (EN)
    pub Reserved0: B3, // RO
    pub CSS: B3,       // RW - I/O  Command Set Selected (CSS)
    pub MPS: B4,       // RW - Memory Page Size (MPS)
    pub AMS: B3,       // RW - Arbitration Mechanism Selected (AMS)
    pub SHN: B2,       // RW - Shutdown Notification (SHN)
    pub IOSQES: B4,    // RW - I/O  Submission Queue Entry Size (IOSQES)
    pub IOCQES: B4,    // RW - I/O  Completion Queue Entry Size (IOCQES)
    pub Reserved1: B8, // RO
}

//
// 3.1.6  Offset 1Ch: CSTS (Controller Status)
//
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NVME_CSTS_SHST_SHUTDOWN_STATUS {
    NVME_CSTS_SHST_NO_SHUTDOWN = 0,
    NVME_CSTS_SHST_SHUTDOWN_IN_PROCESS = 1,
    NVME_CSTS_SHST_SHUTDOWN_COMPLETED = 2,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CONTROLLER_STATUS {
    pub RDY: bool,      // RO - Ready (RDY)
    pub CFS: bool,      // RO - Controller Fatal Status (CFS)
    pub SHST: B2,       // RO - Shutdown Status (SHST)
    pub NSSRO: bool,    // RW1C - NVM Subsystem Reset Occurred (NSSRO)
    pub PP: bool,       // RO - Processing Paused (PP)
    pub Reserved0: B26, // RO
}

//
// 3.1.7  Offset 20h: NSSR (NVM Subsystem Reset)
//
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_NVM_SUBSYSTEM_RESET {
    pub NSSRC: u32, // RW - NVM Subsystem Reset Control (NSSRC)
}

//
// 3.1.8  Offset 24h: AQA (Admin Queue Attributes)
//
// #[derive(Clone, Copy)]
// pub union NVME_ADMIN_QUEUE_ATTRIBUTES {
//     pub bits: u32,
//     pub fields: NVME_ADMIN_QUEUE_ATTRIBUTES_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_ADMIN_QUEUE_ATTRIBUTES {
    pub ASQS: B12,     // RW - Admin  Submission Queue Size (ASQS)
    pub Reserved0: B4, // RO
    pub ACQS: B12,     // RW - Admin  Completion Queue Size (ACQS)
    pub Reserved1: B4, // RO
}
//
// 3.1.9  Offset 28h: ASQ (Admin Submission Queue Base Address)
//
#[bitfield]
#[repr(u64)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS {
    pub Reserved0: B12, // RO
    pub ASQB: B52,      // RW - Admin Submission Queue Base (ASQB)
}

//
// 3.1.10  Offset 30h: ACQ (Admin Completion Queue Base Address)
//
#[bitfield]
#[repr(u64)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS {
    pub Reserved0: B12, // RO
    pub ACQB: B52,      // RW - Admin Completion Queue Base (ACQB)
}
//
// 3.1.11 Offset 38h: CMBLOC (Controller Memory Buffer Location)
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CONTROLLER_MEMORY_BUFFER_LOCATION {
    pub BIR: B3,      // RO - Base Indicator Register (BIR)
    pub Reserved: B9, // RO
    pub OFST: B20,    // RO - Offset (OFST)
}

//
// 3.1.12 Offset 3Ch: CMBSZ (Controller Memory Buffer Size)
//
pub enum NVME_CMBSZ_SIZE_UNITS {
    NVME_CMBSZ_SIZE_UNITS_4KB = 0,
    NVME_CMBSZ_SIZE_UNITS_64KB = 1,
    NVME_CMBSZ_SIZE_UNITS_1MB = 2,
    NVME_CMBSZ_SIZE_UNITS_16MB = 3,
    NVME_CMBSZ_SIZE_UNITS_256MB = 4,
    NVME_CMBSZ_SIZE_UNITS_4GB = 5,
    NVME_CMBSZ_SIZE_UNITS_64GB = 6,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CONTROLLER_MEMORY_BUFFER_SIZE {
    pub SQS: B1,      // RO - Submission Queue Support (SQS)
    pub CQS: B1,      // RO - Completion Queue Support (CQS)
    pub LISTS: B1,    // RO - PRP SGL List Support (LISTS)
    pub RDS: B1,      // RO - Read Data Support (RDS)
    pub WDS: B1,      // RO - Write Data Support (WDS)
    pub Reserved: B3, // RO
    pub SZU: B4,      // RO - Size Units (SZU)
    pub SZ: B20,      // RO - Size (SZ)
}

//
// 3.1.13  Offset (1000h + ((2y) * (4 << CAP.DSTRD))): SQyTDBL (Submission Queue y Tail Doorbell)
//
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_SUBMISSION_QUEUE_TAIL_DOORBELL {
    pub SQT: u16,       // RW - Submission Queue Tail (SQT)
    pub Reserved0: u16, // RO
}

//
// 3.1.14  Offset  (1000h + ((2y + 1) * (4 << CAP.DSTRD))): CQyHDBL (Completion Queue y Head Doorbell)
//
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMPLETION_QUEUE_HEAD_DOORBELL {
    pub CQH: u16,       // RW - Completion Queue Head (CQH)
    pub Reserved0: u16, // RO
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_CONTROLLER_REGISTERS {
    pub CAP: NVME_CONTROLLER_CAPABILITIES, // Controller Capabilities; 8 bytes
    pub VS: NVME_VERSION,                  // Version
    pub INTMS: u32,                        // Interrupt Mask Set
    pub INTMC: u32,                        // Interrupt Mask Clear
    pub CC: NVME_CONTROLLER_CONFIGURATION, // Controller Configuration
    pub Reserved0: u32,
    pub CSTS: NVME_CONTROLLER_STATUS,     // Controller Status
    pub NSSR: NVME_NVM_SUBSYSTEM_RESET,   // NVM Subsystem Reset (Optional)
    pub AQA: NVME_ADMIN_QUEUE_ATTRIBUTES, // Admin Queue Attributes
    pub ASQ: NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS, // Admin Submission Queue Base Address; 8 bytes
    pub ACQ: NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS, // Admin Completion Queue Base Address; 8 bytes
    pub CMBLOC: NVME_CONTROLLER_MEMORY_BUFFER_LOCATION, // Controller Memory Buffer Location (Optional)
    pub CMBSZ: NVME_CONTROLLER_MEMORY_BUFFER_SIZE,      // Controller Memory Buffer Size (Optional)
    pub Reserved2: [u32; 944],                          // 40h ~ EFFh
    pub Reserved3: [u32; 64],                           // F00h ~ FFFh, Command Set Specific
    pub Doorbells: [u32; 0], // Start of the first Doorbell register. (Admin SQ Tail Doorbell)
}

impl Default for NVME_CONTROLLER_REGISTERS {
    fn default() -> Self {
        NVME_CONTROLLER_REGISTERS {
            CAP: NVME_CONTROLLER_CAPABILITIES::default(),
            VS: NVME_VERSION::default(),
            INTMS: 0,
            INTMC: 0,
            CC: NVME_CONTROLLER_CONFIGURATION::default(),
            Reserved0: 0,
            CSTS: NVME_CONTROLLER_STATUS::default(),
            NSSR: NVME_NVM_SUBSYSTEM_RESET::default(),
            AQA: NVME_ADMIN_QUEUE_ATTRIBUTES::default(),
            ASQ: NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS::default(),
            ACQ: NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS::default(),
            CMBLOC: NVME_CONTROLLER_MEMORY_BUFFER_LOCATION::default(),
            CMBSZ: NVME_CONTROLLER_MEMORY_BUFFER_SIZE::default(),
            Reserved2: [0; 944],
            Reserved3: [0; 64],
            Doorbells: [0; 0],
        }
    }
}

//
// Command completion status
// The "Phase Tag" field and "Status Field" are separated in spec. We define them in the same data structure to ease the memory access from software.
//
#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_STATUS {
    pub P: B1,   // Phase Tag (P)
    pub SC: B8,  // Status Code (SC)
    pub SCT: B3, // Status Code Type (SCT)
    pub Reserved: B2,
    pub M: B1,   // More (M)
    pub DNR: B1, // Do Not Retry (DNR)
}

//
// Command completion entry
//
#[repr(C)]
pub struct NVME_COMPLETION_ENTRY {
    pub DW0: u32,
    pub DW1: u32,
    pub DW2: NVME_COMPLETION_ENTRY_DW2,
    pub DW3: NVME_COMPLETION_ENTRY_DW3,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMPLETION_ENTRY_DW2 {
    pub SQHD: u16, // SQ Head Pointer (SQHD)
    pub SQID: u16, // SQ Identifier (SQID)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMPLETION_ENTRY_DW3 {
    pub CID: u16, // Command Identifier (CID)
    pub Status: NVME_COMMAND_STATUS,
}

//
// Completion entry DW0 for NVME_ADMIN_COMMAND_ASYNC_EVENT_REQUEST
//
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NVME_ASYNC_EVENT_TYPES {
    NVME_ASYNC_EVENT_TYPE_ERROR_STATUS = 0,
    NVME_ASYNC_EVENT_TYPE_HEALTH_STATUS = 1,
    NVME_ASYNC_EVENT_TYPE_NOTICE = 2,
    NVME_ASYNC_EVENT_TYPE_IO_COMMAND_SET_STATUS = 6,
    NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC = 7,
}

//
// Error Status: NVME_ASYNC_EVENT_TYPE_ERROR_STATUS
//
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NVME_ASYNC_EVENT_ERROR_STATUS_CODES {
    NVME_ASYNC_ERROR_INVALID_SUBMISSION_QUEUE = 0,
    NVME_ASYNC_ERROR_INVALID_DOORBELL_WRITE_VALUE = 1,
    NVME_ASYNC_ERROR_DIAG_FAILURE = 2,
    NVME_ASYNC_ERROR_PERSISTENT_INTERNAL_DEVICE_ERROR = 3,
    NVME_ASYNC_ERROR_TRANSIENT_INTERNAL_DEVICE_ERROR = 4,
    NVME_ASYNC_ERROR_FIRMWARE_IMAGE_LOAD_ERROR = 5,
}

//
// SMART/Health Status: NVME_ASYNC_EVENT_TYPE_HEALTH_STATUS
//
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NVME_ASYNC_EVENT_HEALTH_STATUS_CODES {
    NVME_ASYNC_HEALTH_NVM_SUBSYSTEM_RELIABILITY = 0,
    NVME_ASYNC_HEALTH_TEMPERATURE_THRESHOLD = 1,
    NVME_ASYNC_HEALTH_SPARE_BELOW_THRESHOLD = 2,
}

// Notice Status: NVME_ASYNC_EVENT_TYPE_NOTICE
//
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NVME_ASYNC_EVENT_NOTICE_CODES {
    NVME_ASYNC_NOTICE_NAMESPACE_ATTRIBUTE_CHANGED = 0,
    NVME_ASYNC_NOTICE_FIRMWARE_ACTIVATION_STARTING = 1,
    NVME_ASYNC_NOTICE_TELEMETRY_LOG_CHANGED = 2,
    NVME_ASYNC_NOTICE_ASYMMETRIC_ACCESS_CHANGE = 3,
    NVME_ASYNC_NOTICE_PREDICTABLE_LATENCY_EVENT_AGGREGATE_LOG_CHANGE = 4,
    NVME_ASYNC_NOTICE_LBA_STATUS_INFORMATION_ALERT = 5,
    NVME_ASYNC_NOTICE_ENDURANCE_GROUP_EVENT_AGGREGATE_LOG_CHANGE = 6,
    NVME_ASYNC_NOTICE_ZONE_DESCRIPTOR_CHANGED = 0xEF,
}

pub enum NVME_ASYNC_EVENT_IO_COMMAND_SET_STATUS_CODES {
    NVME_ASYNC_IO_CMD_SET_RESERVATION_LOG_PAGE_AVAILABLE = 0,
    NVME_ASYNC_IO_CMD_SANITIZE_OPERATION_COMPLETED = 1,
    NVME_ASYNC_IO_CMD_SANITIZE_OPERATION_COMPLETED_WITH_UNEXPECTED_DEALLOCATION = 2,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST {
    pub AsyncEventType: B3,
    pub Reserved0: B5,
    pub AsyncEventInfo: B8,
    pub LogPage: B8,
    pub Reserved1: B8,
}

pub enum NVME_STATUS_TYPES {
    NVME_STATUS_TYPE_GENERIC_COMMAND = 0,
    NVME_STATUS_TYPE_COMMAND_SPECIFIC = 1,
    NVME_STATUS_TYPE_MEDIA_ERROR = 2,
    NVME_STATUS_TYPE_VENDOR_SPECIFIC = 7,
}

//
//  Status Code (SC) of NVME_STATUS_TYPE_GENERIC_COMMAND
//
pub enum NVME_STATUS_GENERIC_COMMAND_CODES {
    NVME_STATUS_SUCCESS_COMPLETION = 0x00,
    NVME_STATUS_INVALID_COMMAND_OPCODE = 0x01,
    NVME_STATUS_INVALID_FIELD_IN_COMMAND = 0x02,
    NVME_STATUS_COMMAND_ID_CONFLICT = 0x03,
    NVME_STATUS_DATA_TRANSFER_ERROR = 0x04,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_POWER_LOSS_NOTIFICATION = 0x05,
    NVME_STATUS_INTERNAL_DEVICE_ERROR = 0x06,
    NVME_STATUS_COMMAND_ABORT_REQUESTED = 0x07,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_SQ_DELETION = 0x08,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_FAILED_FUSED_COMMAND = 0x09,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_FAILED_MISSING_COMMAND = 0x0A,
    NVME_STATUS_INVALID_NAMESPACE_OR_FORMAT = 0x0B,
    NVME_STATUS_COMMAND_SEQUENCE_ERROR = 0x0C,
    NVME_STATUS_INVALID_SGL_LAST_SEGMENT_DESCR = 0x0D,
    NVME_STATUS_INVALID_NUMBER_OF_SGL_DESCR = 0x0E,
    NVME_STATUS_DATA_SGL_LENGTH_INVALID = 0x0F,
    NVME_STATUS_METADATA_SGL_LENGTH_INVALID = 0x10,
    NVME_STATUS_SGL_DESCR_TYPE_INVALID = 0x11,
    NVME_STATUS_INVALID_USE_OF_CONTROLLER_MEMORY_BUFFER = 0x12,
    NVME_STATUS_PRP_OFFSET_INVALID = 0x13,
    NVME_STATUS_ATOMIC_WRITE_UNIT_EXCEEDED = 0x14,
    NVME_STATUS_OPERATION_DENIED = 0x15,
    NVME_STATUS_SGL_OFFSET_INVALID = 0x16,
    NVME_STATUS_RESERVED = 0x17,
    NVME_STATUS_HOST_IDENTIFIER_INCONSISTENT_FORMAT = 0x18,
    NVME_STATUS_KEEP_ALIVE_TIMEOUT_EXPIRED = 0x19,
    NVME_STATUS_KEEP_ALIVE_TIMEOUT_INVALID = 0x1A,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_PREEMPT_ABORT = 0x1B,
    NVME_STATUS_SANITIZE_FAILED = 0x1C,
    NVME_STATUS_SANITIZE_IN_PROGRESS = 0x1D,
    NVME_STATUS_SGL_DATA_BLOCK_GRANULARITY_INVALID = 0x1E,

    NVME_STATUS_DIRECTIVE_TYPE_INVALID = 0x70,
    NVME_STATUS_DIRECTIVE_ID_INVALID = 0x71,

    NVME_STATUS_NVM_LBA_OUT_OF_RANGE = 0x80,
    NVME_STATUS_NVM_CAPACITY_EXCEEDED = 0x81,
    NVME_STATUS_NVM_NAMESPACE_NOT_READY = 0x82,
    NVME_STATUS_NVM_RESERVATION_CONFLICT = 0x83,
    NVME_STATUS_FORMAT_IN_PROGRESS = 0x84,
}

//
//  Status Code (SC) of NVME_STATUS_TYPE_COMMAND_SPECIFIC
//
pub enum NVME_STATUS_COMMAND_SPECIFIC_CODES {
    NVME_STATUS_COMPLETION_QUEUE_INVALID = 0x00, // Create I/O Submission Queue
    NVME_STATUS_INVALID_QUEUE_IDENTIFIER = 0x01, // Create I/O Submission Queue, Create I/O Completion Queue, Delete I/O Completion Queue, Delete I/O Submission Queue
    NVME_STATUS_MAX_QUEUE_SIZE_EXCEEDED = 0x02, // Create I/O Submission Queue, Create I/O Completion Queue
    NVME_STATUS_ABORT_COMMAND_LIMIT_EXCEEDED = 0x03, // Abort
    NVME_STATUS_ASYNC_EVENT_REQUEST_LIMIT_EXCEEDED = 0x05, // Asynchronous Event Request
    NVME_STATUS_INVALID_FIRMWARE_SLOT = 0x06,   // Firmware Commit
    NVME_STATUS_INVALID_FIRMWARE_IMAGE = 0x07,  // Firmware Commit
    NVME_STATUS_INVALID_INTERRUPT_VECTOR = 0x08, // Create I/O Completion Queue
    NVME_STATUS_INVALID_LOG_PAGE = 0x09,        // Get Log Page
    NVME_STATUS_INVALID_FORMAT = 0x0A,          // Format NVM
    NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_CONVENTIONAL_RESET = 0x0B, // Firmware Commit
    NVME_STATUS_INVALID_QUEUE_DELETION = 0x0C,  // Delete I/O Completion Queue
    NVME_STATUS_FEATURE_ID_NOT_SAVEABLE = 0x0D, // Set Features
    NVME_STATUS_FEATURE_NOT_CHANGEABLE = 0x0E,  // Set Features
    NVME_STATUS_FEATURE_NOT_NAMESPACE_SPECIFIC = 0x0F, // Set Features
    NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_NVM_SUBSYSTEM_RESET = 0x10, // Firmware Commit
    NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_RESET = 0x11, // Firmware Commit
    NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_MAX_TIME_VIOLATION = 0x12, // Firmware Commit
    NVME_STATUS_FIRMWARE_ACTIVATION_PROHIBITED = 0x13, // Firmware Commit
    NVME_STATUS_OVERLAPPING_RANGE = 0x14, // Firmware Commit, Firmware Image Download, Set Features

    NVME_STATUS_NAMESPACE_INSUFFICIENT_CAPACITY = 0x15, // Namespace Management
    NVME_STATUS_NAMESPACE_IDENTIFIER_UNAVAILABLE = 0x16, // Namespace Management
    NVME_STATUS_NAMESPACE_ALREADY_ATTACHED = 0x18,      // Namespace Attachment
    NVME_STATUS_NAMESPACE_IS_PRIVATE = 0x19,            // Namespace Attachment
    NVME_STATUS_NAMESPACE_NOT_ATTACHED = 0x1A,          // Namespace Attachment
    NVME_STATUS_NAMESPACE_THIN_PROVISIONING_NOT_SUPPORTED = 0x1B, // Namespace Management
    NVME_STATUS_CONTROLLER_LIST_INVALID = 0x1C,         // Namespace Attachment

    NVME_STATUS_DEVICE_SELF_TEST_IN_PROGRESS = 0x1D, // Device Self-test

    NVME_STATUS_BOOT_PARTITION_WRITE_PROHIBITED = 0x1E, // Firmware Commit

    NVME_STATUS_INVALID_CONTROLLER_IDENTIFIER = 0x1F, // Virtualization Management
    NVME_STATUS_INVALID_SECONDARY_CONTROLLER_STATE = 0x20, // Virtualization Management
    NVME_STATUS_INVALID_NUMBER_OF_CONTROLLER_RESOURCES = 0x21, // Virtualization Management
    NVME_STATUS_INVALID_RESOURCE_IDENTIFIER = 0x22,   // Virtualization Management

    NVME_STATUS_SANITIZE_PROHIBITED_ON_PERSISTENT_MEMORY = 0x23, // Sanitize

    NVME_STATUS_INVALID_ANA_GROUP_IDENTIFIER = 0x24, // Namespace Management
    NVME_STATUS_ANA_ATTACH_FAILED = 0x25,            // Namespace Attachment

    NVME_IO_COMMAND_SET_NOT_SUPPORTED = 0x29, // Namespace Attachment/Management
    NVME_IO_COMMAND_SET_NOT_ENABLED = 0x2A,   // Namespace Attachment
    NVME_IO_COMMAND_SET_COMBINATION_REJECTED = 0x2B, // Set Features
    NVME_IO_COMMAND_SET_INVALID = 0x2C,       // Identify

    NVME_STATUS_STREAM_RESOURCE_ALLOCATION_FAILED = 0x7F, // Streams Directive

    NVME_STATUS_NVM_CONFLICTING_ATTRIBUTES = 0x80, // Dataset Management, Read, Write
    NVME_STATUS_NVM_INVALID_PROTECTION_INFORMATION = 0x81, // Compare, Read, Write, Write Zeroes
    NVME_STATUS_NVM_ATTEMPTED_WRITE_TO_READ_ONLY_RANGE = 0x82, // Dataset Management, Write, Write Uncorrectable, Write Zeroes
    NVME_STATUS_NVM_COMMAND_SIZE_LIMIT_EXCEEDED = 0x83,        // Dataset Management

    NVME_STATUS_ZONE_BOUNDARY_ERROR = 0xB8, // Compare, Read, Verify, Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append
    NVME_STATUS_ZONE_FULL = 0xB9, // Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append
    NVME_STATUS_ZONE_READ_ONLY = 0xBA, // Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append
    NVME_STATUS_ZONE_OFFLINE = 0xBB, // Compare, Read, Verify, Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append
    NVME_STATUS_ZONE_INVALID_WRITE = 0xBC, // Write, Write Uncorrectable, Write Zeroes, Copy
    NVME_STATUS_ZONE_TOO_MANY_ACTIVE = 0xBD, // Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append, Zone Management Send
    NVME_STATUS_ZONE_TOO_MANY_OPEN = 0xBE, // Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append, Zone Management Send
    NVME_STATUS_ZONE_INVALID_STATE_TRANSITION = 0xBF, // Zone Management Send
}

//
//  Status Code (SC) of NVME_STATUS_TYPE_MEDIA_ERROR
//
pub enum NVME_STATUS_MEDIA_ERROR_CODES {
    NVME_STATUS_NVM_WRITE_FAULT = 0x80,
    NVME_STATUS_NVM_UNRECOVERED_READ_ERROR = 0x81,
    NVME_STATUS_NVM_END_TO_END_GUARD_CHECK_ERROR = 0x82,
    NVME_STATUS_NVM_END_TO_END_APPLICATION_TAG_CHECK_ERROR = 0x83,
    NVME_STATUS_NVM_END_TO_END_REFERENCE_TAG_CHECK_ERROR = 0x84,
    NVME_STATUS_NVM_COMPARE_FAILURE = 0x85,
    NVME_STATUS_NVM_ACCESS_DENIED = 0x86,
    NVME_STATUS_NVM_DEALLOCATED_OR_UNWRITTEN_LOGICAL_BLOCK = 0x87,
}

//
// Admin Command Set
//
pub enum NVME_ADMIN_COMMANDS {
    NVME_ADMIN_COMMAND_DELETE_IO_SQ = 0x00,
    NVME_ADMIN_COMMAND_CREATE_IO_SQ = 0x01,
    NVME_ADMIN_COMMAND_GET_LOG_PAGE = 0x02,
    NVME_ADMIN_COMMAND_DELETE_IO_CQ = 0x04,
    NVME_ADMIN_COMMAND_CREATE_IO_CQ = 0x05,
    NVME_ADMIN_COMMAND_IDENTIFY = 0x06,
    NVME_ADMIN_COMMAND_ABORT = 0x08,
    NVME_ADMIN_COMMAND_SET_FEATURES = 0x09,
    NVME_ADMIN_COMMAND_GET_FEATURES = 0x0A,
    NVME_ADMIN_COMMAND_ASYNC_EVENT_REQUEST = 0x0C,
    NVME_ADMIN_COMMAND_NAMESPACE_MANAGEMENT = 0x0D,

    NVME_ADMIN_COMMAND_FIRMWARE_COMMIT = 0x10, // "Firmware Activate" command has been renamed to "Firmware Commit" command in spec v1.2
    NVME_ADMIN_COMMAND_FIRMWARE_IMAGE_DOWNLOAD = 0x11,
    NVME_ADMIN_COMMAND_DEVICE_SELF_TEST = 0x14,
    NVME_ADMIN_COMMAND_NAMESPACE_ATTACHMENT = 0x15,

    NVME_ADMIN_COMMAND_DIRECTIVE_SEND = 0x19,
    NVME_ADMIN_COMMAND_DIRECTIVE_RECEIVE = 0x1A,
    NVME_ADMIN_COMMAND_VIRTUALIZATION_MANAGEMENT = 0x1C,
    NVME_ADMIN_COMMAND_NVME_MI_SEND = 0x1D,
    NVME_ADMIN_COMMAND_NVME_MI_RECEIVE = 0x1E,

    NVME_ADMIN_COMMAND_DOORBELL_BUFFER_CONFIG = 0x7C,

    NVME_ADMIN_COMMAND_FORMAT_NVM = 0x80,
    NVME_ADMIN_COMMAND_SECURITY_SEND = 0x81,
    NVME_ADMIN_COMMAND_SECURITY_RECEIVE = 0x82,
    NVME_ADMIN_COMMAND_SANITIZE = 0x84,
    NVME_ADMIN_COMMAND_GET_LBA_STATUS = 0x86,
}

//
// Features for Get/Set Features command
//
pub enum NVME_FEATURES {
    NVME_FEATURE_ARBITRATION = 0x01,
    NVME_FEATURE_POWER_MANAGEMENT = 0x02,
    NVME_FEATURE_LBA_RANGE_TYPE = 0x03,
    NVME_FEATURE_TEMPERATURE_THRESHOLD = 0x04,
    NVME_FEATURE_ERROR_RECOVERY = 0x05,
    NVME_FEATURE_VOLATILE_WRITE_CACHE = 0x06,
    NVME_FEATURE_NUMBER_OF_QUEUES = 0x07,
    NVME_FEATURE_INTERRUPT_COALESCING = 0x08,
    NVME_FEATURE_INTERRUPT_VECTOR_CONFIG = 0x09,
    NVME_FEATURE_WRITE_ATOMICITY = 0x0A,
    NVME_FEATURE_ASYNC_EVENT_CONFIG = 0x0B,
    NVME_FEATURE_AUTONOMOUS_POWER_STATE_TRANSITION = 0x0C,
    NVME_FEATURE_HOST_MEMORY_BUFFER = 0x0D,
    NVME_FEATURE_TIMESTAMP = 0x0E,
    NVME_FEATURE_KEEP_ALIVE = 0x0F,
    NVME_FEATURE_HOST_CONTROLLED_THERMAL_MANAGEMENT = 0x10,
    NVME_FEATURE_NONOPERATIONAL_POWER_STATE = 0x11,
    NVME_FEATURE_READ_RECOVERY_LEVEL_CONFIG = 0x12,
    NVME_FEATURE_PREDICTABLE_LATENCY_MODE_CONFIG = 0x13,
    NVME_FEATURE_PREDICTABLE_LATENCY_MODE_WINDOW = 0x14,
    NVME_FEATURE_LBA_STATUS_INFORMATION_REPORT_INTERVAL = 0x15,
    NVME_FEATURE_HOST_BEHAVIOR_SUPPORT = 0x16,
    NVME_FEATURE_SANITIZE_CONFIG = 0x17,
    NVME_FEATURE_ENDURANCE_GROUP_EVENT_CONFIG = 0x18,
    NVME_FEATURE_IO_COMMAND_SET_PROFILE = 0x19,

    NVME_FEATURE_ENHANCED_CONTROLLER_METADATA = 0x7D,
    NVME_FEATURE_CONTROLLER_METADATA = 0x7E,
    NVME_FEATURE_NAMESPACE_METADATA = 0x7F,

    NVME_FEATURE_NVM_SOFTWARE_PROGRESS_MARKER = 0x80,
    NVME_FEATURE_NVM_HOST_IDENTIFIER = 0x81,
    NVME_FEATURE_NVM_RESERVATION_NOTIFICATION_MASK = 0x82,
    NVME_FEATURE_NVM_RESERVATION_PERSISTANCE = 0x83,
    NVME_FEATURE_NVM_NAMESPACE_WRITE_PROTECTION_CONFIG = 0x84,

    NVME_FEATURE_ERROR_INJECTION = 0xC0, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_CLEAR_FW_UPDATE_HISTORY = 0xC1, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_READONLY_WRITETHROUGH_MODE = 0xC2, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS = 0xC3, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_ENABLE_IEEE1667_SILO = 0xC4, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_PLP_HEALTH_MONITOR = 0xC5,   // This is from OCP NVMe Cloud SSD spec.
}

//
// Abort command: parameter
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_ABORT {
    pub SQID: B8, // Submission Queue Identifier (SQID)
    pub CID: B16, // Command Identifier (CID)
    reserved: B8,
}

//
// Identify Command of Controller or Namespace Structure (CNS)
//
pub enum NVME_IDENTIFY_CNS_CODES {
    NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE = 0x0,
    NVME_IDENTIFY_CNS_CONTROLLER = 0x1,
    NVME_IDENTIFY_CNS_ACTIVE_NAMESPACES = 0x2, // A list of up to 1024 active namespace IDs is returned to the host containing active namespaces with a namespace identifier greater than the value specified in the Namespace Identifier (CDW1.NSID) field.
    NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE = 0x3,
    NVME_IDENTIFY_CNS_NVM_SET = 0x4,

    NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE_IO_COMMAND_SET = 0x5,
    NVME_IDENTIFY_CNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET = 0x6,
    NVME_IDENTIFY_CNS_ACTIVE_NAMESPACE_LIST_IO_COMMAND_SET = 0x7,

    NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_LIST = 0x10,
    NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE = 0x11,
    NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NSID = 0x12,
    NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NVM_SUBSYSTEM = 0x13,
    NVME_IDENTIFY_CNS_PRIMARY_CONTROLLER_CAPABILITIES = 0x14,
    NVME_IDENTIFY_CNS_SECONDARY_CONTROLLER_LIST = 0x15,
    NVME_IDENTIFY_CNS_NAMESPACE_GRANULARITY_LIST = 0x16,
    NVME_IDENTIFY_CNS_UUID_LIST = 0x17,
    NVME_IDENTIFY_CNS_DOMAIN_LIST = 0x18,
    NVME_IDENTIFY_CNS_ENDURANCE_GROUP_LIST = 0x19,

    NVME_IDENTIFY_CNS_ALLOCATED_NAMSPACE_LIST_IO_COMMAND_SET = 0x1A,
    NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_IO_COMMAND_SET = 0x1B,
    NVME_IDENTIFY_CNS_IO_COMMAND_SET = 0x1C,
}

//
// Identify Command Set Identifiers (CSI)
//
pub enum NVME_COMMAND_SET_IDENTIFIERS {
    NVME_COMMAND_SET_NVM = 0x0,
    NVME_COMMAND_SET_KEY_VALUE = 0x1,
    NVME_COMMAND_SET_ZONED_NAMESPACE = 0x2,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_IDENTIFY {
    pub CNS: B8, // Controller or Namespace Structure (CNS, Defined in NVME_IDENTIFY_CNS_CODES)
    pub Reserved: B8,
    pub CNTID: B16, // Controller Identifier (CNTID)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_IDENTIFY {
    pub NVM: NVME_CDW11_IDENTIFY_STRUCT,
    pub CNS: NVME_CDW11_IDENTIFY_STRUCT2,
    pub AsUlong: u32,
}

impl Default for NVME_CDW11_IDENTIFY {
    fn default() -> Self {
        NVME_CDW11_IDENTIFY { AsUlong: 0 }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_IDENTIFY_STRUCT {
    pub NVMSETID: u16, // NVM Set Identifier
    pub Reserved: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_IDENTIFY_STRUCT2 {
    pub CNSID: u32, // CNS Specific Identifier (NVM Set ID/Domain ID/Endurance Group ID)
    pub Reserved2: u8,
    pub CSI: u8, // Command Set Identifier (CSI, Defined in NVME_COMMAND_SET_IDENTIFIERS)
}

//
// Output of NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE (0x0)
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_LBA_FORMAT {
    pub MS: u16,   // bit 0:15     Metadata Size (MS)
    pub LBADS: u8, // bit 16:23    LBA  Data  Size (LBADS)

    pub RP: B2,        // bit 24:25    Relative Performance (RP)
    pub Reserved0: B6, // bit 26:31
}

//
//
#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVM_RESERVATION_CAPABILITIES {
    pub PersistThroughPowerLoss: B1,
    pub WriteExclusiveReservation: B1,
    pub ExclusiveAccessReservation: B1,
    pub WriteExclusiveRegistrantsOnlyReservation: B1,
    pub ExclusiveAccessRegistrantsOnlyReservation: B1,
    pub WriteExclusiveAllRegistrantsReservation: B1,
    pub ExclusiveAccessAllRegistrantsReservation: B1,
    pub Reserved: B1,
}

//
//
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA {
    pub NSZE: u64,                              // byte 0:7        M - Namespace Size (NSZE)
    pub NCAP: u64,                              // byte 8:15       M - Namespace Capacity (NCAP)
    pub NUSE: u64,                              // byte 16:23      M - Namespace Utilization (NUSE)
    pub NSFEAT: NamespaceFeatures,              // byte 24         M - Namespace Features (NSFEAT)
    pub NLBAF: u8,                              // byte 25         M - Number of LBA Formats (NLBAF)
    pub FLBAS: FormattedLbaSize,                // byte 26         M - Formatted LBA Size (FLBAS)
    pub MC: MetadataCapabilities,               // byte 27         M - Metadata Capabilities (MC)
    pub DPC: DataProtectionCapabilities, // byte 28         M - End-to-end Data Protection Capabilities (DPC)
    pub DPS: DataProtectionTypeSettings, // byte 29         M - End-to-end Data Protection Type Settings (DPS)
    pub NMIC: NamespaceMultiPathIoCapabilities, // byte 30         O - Namespace Multi-path I/O and Namespace Sharing Capabilities (NMIC)
    pub RESCAP: NvmReservationCapabilities, // byte 31         O - Reservation Capabilities (RESCAP)
    pub FPI: FormatProgressIndicator,       // byte 32         O - Format Progress Indicator (FPI)
    pub DLFEAT: DeallocatedLogicalBlockFeatures, // byte 33
    pub NAWUN: u16,  // byte 34:35      O - Namespace Atomic Write Unit Normal (NAWUN)
    pub NAWUPF: u16, // byte 36:37      O - Namespace Atomic Write Unit Power Fail (NAWUPF)
    pub NACWU: u16,  // byte 38:39      O - Namespace Atomic Compare & Write Unit (NACWU)
    pub NABSN: u16,  // byte 40:41      O - Namespace Atomic Boundary Size Normal (NABSN)
    pub NABO: u16,   // byte 42:43      O - Namespace Atomic Boundary Offset (NABO)
    pub NABSPF: u16, // byte 44:45      O - Namespace Atomic Boundary Size Power Fail (NABSPF)
    pub NOIOB: u16,  // byte 46:47      O - Namespace Optimal IO Boundary (NOIOB)
    pub NVMCAP: [u8; 16], // byte 48:63      O - NVM Capacity (NVMCAP)
    pub NPWG: u16,   // byte 64:65      O - Namespace Preferred Write Granularity (NPWG)
    pub NPWA: u16,   // byte 66:67      O - Namespace Preferred Write Alignment (NPWA)
    pub NPDG: u16,   // byte 68:69      O - Namespace Preferred Deallocate Granularity (NPDG)
    pub NPDA: u16,   // byte 70:71      O - Namespace Preferred Deallocate Alignment (NPDA)
    pub NOWS: u16,   // byte 72:73      O - Namespace Optimal Write Size (NOWS)
    pub MSSRL: u16,  // byte 74:75      O - Maximum Single Source Range Length(MSSRL)
    pub MCL: u32,    // byte 76:79      O - Maximum Copy Length(MCL)
    pub MSRC: u8,    // byte 80         O - Maximum Source Range Count(MSRC)
    pub Reserved2: [u8; 11], // byte 81:91
    pub ANAGRPID: u32, // byte 92:95      O - ANA Group Identifier (ANAGRPID)
    pub Reserved3: [u8; 3], // byte 96:98
    pub NSATTR: NamespaceAttributes, // byte 99         O - Namespace Attributes{
    pub NVMSETID: u16, // byte 100:101    O - Associated NVM Set Identifier
    pub ENDGID: u16, // byte 102:103    O - Associated Endurance Group Identier
    pub NGUID: [u8; 16], // byte 104:119    O - Namespace Globally Unique Identifier (NGUID)
    pub EUI64: [u8; 8], // byte 120:127    M - IEEE Extended Unique Identifier (EUI64)
    pub LBAF: [NVME_LBA_FORMAT; 16], // byte 128:191 M - LBA Format 0~15 Support (LBAF0)
    pub Reserved4: [u8; 192], // byte 192:383
    pub VS: [u8; 3712], // byte 384:4095    O - Vendor Specific (VS): This range of bytes is allocated for vendor specific usage.
}

impl Default for NVME_IDENTIFY_NAMESPACE_DATA {
    fn default() -> Self {
        NVME_IDENTIFY_NAMESPACE_DATA {
            NSZE: 0,
            NCAP: 0,
            NUSE: 0,
            NSFEAT: NamespaceFeatures::default(),
            NLBAF: 0,
            FLBAS: FormattedLbaSize::default(),
            MC: MetadataCapabilities::default(),
            DPC: DataProtectionCapabilities::default(),
            DPS: DataProtectionTypeSettings::default(),
            NMIC: NamespaceMultiPathIoCapabilities::default(),
            RESCAP: NvmReservationCapabilities::default(),
            FPI: FormatProgressIndicator::default(),
            DLFEAT: DeallocatedLogicalBlockFeatures::default(),
            NAWUN: 0,
            NAWUPF: 0,
            NACWU: 0,
            NABSN: 0,
            NABO: 0,
            NABSPF: 0,
            NOIOB: 0,
            NVMCAP: [0; 16],
            NPWG: 0,
            NPWA: 0,
            NPDG: 0,
            NPDA: 0,
            NOWS: 0,
            MSSRL: 0,
            MCL: 0,
            MSRC: 0,
            Reserved2: [0; 11],
            ANAGRPID: 0,
            Reserved3: [0; 3],
            NSATTR: NamespaceAttributes::default(),
            NVMSETID: 0,
            ENDGID: 0,
            NGUID: [0; 16],
            EUI64: [0; 8],
            LBAF: [NVME_LBA_FORMAT::default(); 16],
            Reserved4: [0; 192],
            VS: [0; 3712],
        }
    }
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NamespaceFeatures {
    pub ThinProvisioning: B1,
    pub NameSpaceAtomicWriteUnit: B1,
    pub DeallocatedOrUnwrittenError: B1,
    pub SkipReuseUI: B1,
    pub NameSpaceIoOptimization: B1,
    pub Reserved: B3,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FormattedLbaSize {
    pub LbaFormatIndex: B4,
    pub MetadataInExtendedDataLBA: B1,
    Reserved: B3,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MetadataCapabilities {
    pub MetadataInExtendedDataLBA: B1,
    pub MetadataInSeparateBuffer: B1,
    Reserved: B6,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DataProtectionCapabilities {
    pub ProtectionInfoType1: B1,
    pub ProtectionInfoType2: B1,
    pub ProtectionInfoType3: B1,
    pub InfoAtBeginningOfMetadata: B1,
    pub InfoAtEndOfMetadata: B1,
    pub Reserved: B3,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DataProtectionTypeSettings {
    pub ProtectionInfoTypeEnabled: B3,
    pub InfoAtBeginningOfMetadata: B1,
    Reserved: B4,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NamespaceMultiPathIoCapabilities {
    pub SharedNameSpace: B1,
    Reserved: B7,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NvmReservationCapabilities {
    pub PersistThroughPowerLoss: B1,
    pub WriteExclusiveReservation: B1,
    pub ExclusiveAccessReservation: B1,
    pub WriteExclusiveRegistrantsOnlyReservation: B1,
    pub ExclusiveAccessRegistrantsOnlyReservation: B1,
    pub WriteExclusiveAllRegistrantsReservation: B1,
    pub ExclusiveAccessAllRegistrantsReservation: B1,
    pub Reserved: B1,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FormatProgressIndicator {
    pub PercentageRemained: B7,
    Supported: B1,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DeallocatedLogicalBlockFeatures {
    pub ReadBehavior: B3,
    pub WriteZeroes: B1,
    pub GuardFieldWithCRC: B1,
    Reserved: B3,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NamespaceAttributes {
    pub WriteProtected: B1, // Write Protected
    Reserved: B7,           // Reserved
} // byte 99 O - Namespace Attributes

//
// Output of NVME_IDENTIFY_CNS_CONTROLLER (0x01)
//
#[bitfield]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_POWER_STATE_DESC {
    pub MP: B16, // bit 0:15.    Maximum  Power (MP)

    pub Reserved0: B8, // bit 16:23

    pub MPS: B1,       // bit 24: Max Power Scale (MPS)
    pub NOPS: B1,      // bit 25: Non-Operational State (NOPS)
    pub Reserved1: B6, // bit 26:31

    pub ENLAT: B32, // bit 32:63.   Entry Latency (ENLAT)
    pub EXLAT: B32, // bit 64:95.   Exit Latency (EXLAT)

    pub RRT: B5,       // bit 96:100.  Relative Read Throughput (RRT)
    pub Reserved2: B3, // bit 101:103

    pub RRL: B5,       // bit 104:108  Relative Read Latency (RRL)
    pub Reserved3: B3, // bit 109:111

    pub RWT: B5,       // bit 112:116  Relative Write Throughput (RWT)
    pub Reserved4: B3, // bit 117:119

    pub RWL: B5,       // bit 120:124  Relative Write Latency (RWL)
    pub Reserved5: B3, // bit 125:127

    pub IDLP: B16, // bit 128:143  Idle Power (IDLP)

    pub Reserved6: B6, // bit 144:149
    pub IPS: B2,       // bit 150:151  Idle Power Scale (IPS)

    pub Reserved7: B8, // bit 152:159

    pub ACTP: B16, // bit 160:175  Active Power (ACTP)

    pub APW: B3,       // bit 176:178  Active Power Workload (APW)
    pub Reserved8: B3, // bit 179:181
    pub APS: B2,       // bit 182:183  Active Power Scale (APS)

    pub Reserved9: B72, // bit 184:255.
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA {
    pub VID: u16,                         // byte 0:1.    M - PCI Vendor ID (VID)
    pub SSVID: u16,                       // byte 2:3.    M - PCI Subsystem Vendor ID (SSVID)
    pub SN: [u8; 20],                     // byte 4: 23.  M - Serial Number (SN)
    pub MN: [u8; 40],                     // byte 24:63.  M - Model Number (MN)
    pub FR: [u8; 8],                      // byte 64:71.  M - Firmware Revision (FR)
    pub RAB: u8,                          // byte 72.     M - Recommended Arbitration Burst (RAB)
    pub IEEE: [u8; 3], // byte 73:75.  M - IEEE OUI Identifier (IEEE). Controller Vendor code.
    pub CMIC: CMIC, // byte 76.     O - Controller Multi-Path I/O and Namespace Sharing Capabilities (CMIC)
    pub MDTS: u8,   // byte 77.     M - Maximum Data Transfer Size (MDTS)
    pub CNTLID: u16, // byte 78:79.   M - Controller ID (CNTLID)
    pub VER: u32,   // byte 80:83.   M - Version (VER)
    pub RTD3R: u32, // byte 84:87.   M - RTD3 Resume Latency (RTD3R)
    pub RTD3E: u32, // byte 88:91.   M - RTD3 Entry Latency (RTD3E)
    pub OAES: OAES, // byte 92:95.   M - Optional Asynchronous Events Supported (OAES)
    pub CTRATT: CTRATT, // byte 96:99.   M - Controller Attributes (CTRATT)
    pub RRLS: RRLS, // byte 100:101. O - Read Recovery Levels Supported (RRLS)
    pub Reserved0: [u8; 9], // byte 102:110.
    pub CNTRLTYPE: u8, // byte 111.     M - Controller Type
    pub FGUID: [u8; 16], // byte 112:127. O - FRU Globally Unique Identifier (FGUID)
    pub CRDT1: u16, // byte 128:129. O - Command Retry Delay Time 1
    pub CRDT2: u16, // byte 130:131. O - Command Retry Delay Time 1
    pub CRDT3: u16, // byte 132:133. O - Command Retry Delay Time 1
    pub Reserved0_1: [u8; 106], // byte 134:239.
    pub ReservedForManagement: [u8; 16], // byte 240:255.  Refer to the NVMe Management Interface Specification for definition.
    pub OACS: OACS,                      // byte 256:257. M - Optional Admin Command Support (OACS)
    pub ACL: u8,                         // byte 258.    M - Abort Command Limit (ACL)
    pub AERL: u8,                        // byte 259.    M - Asynchronous Event Request Limit (AERL)
    pub FRMW: FRMW,                      // byte 260.    M - Firmware Updates (FRMW)
    pub LPA: LPA,                        // byte 261.    M - Log Page Attributes (LPA)
    pub ELPE: u8,                        // byte 262.    M - Error Log Page Entries (ELPE)
    pub NPSS: u8,                        // byte 263.    M - Number of Power States Support (NPSS)
    pub AVSCC: AVSCC, // byte 264.    M - Admin Vendor Specific Command Configuration (AVSCC)
    pub APSTA: APSTA, // byte 265.     O - Autonomous Power State Transition Attributes (APSTA)
    pub WCTEMP: u16,  // byte 266:267. M - Warning Composite Temperature Threshold (WCTEMP)
    pub CCTEMP: u16,  // byte 268:269. M - Critical Composite Temperature Threshold (CCTEMP)
    pub MTFA: u16,    // byte 270:271. O - Maximum Time for Firmware Activation (MTFA)
    pub HMPRE: u32,   // byte 272:275. O - Host Memory Buffer Preferred Size (HMPRE)
    pub HMMIN: u32,   // byte 276:279. O - Host Memory Buffer Minimum Size (HMMIN)
    pub TNVMCAP: [u8; 16], // byte 280:295. O - Total NVM Capacity (TNVMCAP)
    pub UNVMCAP: [u8; 16], // byte 296:311. O - Unallocated NVM Capacity (UNVMCAP)
    pub RPMBS: RPMBS, // byte 312:315. O - Replay Protected Memory Block Support (RPMBS)
    pub EDSTT: u16,   // byte 316:317. O - Extended Device Self-test Time (EDSTT)
    pub DSTO: u8,     // byte 318.     O - Device Self-test Options (DSTO)
    pub FWUG: u8,     // byte 319.     M - Firmware Update Granularity (FWUG)
    pub KAS: u16,     // byte 320:321  M - Keep Alive Support (KAS)
    pub HCTMA: HCTMA, // byte 322:323  O - Host Controlled Thermal Management Attributes (HCTMA)
    pub MNTMT: u16,   // byte 324:325  O - Minimum Thermal Management Temperature (MNTMT)
    pub MXTMT: u16,   // byte 326:327  O - Maximum Thermal Management Temperature (MXTMT)
    pub SANICAP: SANICAP, // byte 328:331  O - Sanitize Capabilities (SANICAP)
    pub HMMINDS: u32, // byte 332:335  O - Host Memory Buffer Minimum Descriptor Entry Size (HMMINDS)
    pub HMMAXD: u16,  // byte 336:337  O - Host Memory Maxiumum Descriptors Entries (HMMAXD)
    pub NSETIDMAX: u16, // byte 338:339  O - NVM Set Identifier Maximum
    pub ENDGIDMAX: u16, // byte 340:341  O - Endurance Group Identifier Maximum (ENDGIDMAX)
    pub ANATT: u8,    // byte 342      O - ANA Transition Time (ANATT)
    pub ANACAP: ANACAP, // byte 343      O - Asymmetric Namespace Access Capabilities (ANACAP)
    pub ANAGRPMAX: u32, // byte 344:347  O - ANA Group Identifier Maximum (ANAGRPMAX)
    pub NANAGRPID: u32, // byte 348:351  O - Number of ANA Group Identifiers (NANAGRPID)
    pub PELS: u32,    // byte 352:355  O - Persistent Event Log Size (PELS)
    pub Reserved1: [u8; 156], // byte 356:511.
    pub SQES: SQES,   // byte 512.    M - Submission Queue Entry Size (SQES)
    pub CQES: CQES,   // byte 513.    M - Completion Queue Entry Size (CQES)
    pub MAXCMD: u16,  // byte 514:515. M - Maximum Outstanding Commands (MAXCMD)
    pub NN: u32,      // byte 516:519. M - Number of Namespaces (NN)
    pub ONCS: ONCS,   // byte 520:521. M - Optional NVM Command Support (ONCS)
    pub FUSES: FUSES, // byte 522:523. M - Fused Operation Support (FUSES)
    pub FNA: FNA,     // byte 524.     M - Format NVM Attributes (FNA)
    pub VWC: VWC,     // byte 525.     M - Volatile Write Cache (VWC)
    pub AWUN: u16,    // byte 526:527. M - Atomic Write Unit Normal (AWUN)
    pub AWUPF: u16,   // byte 528:529. M - Atomic Write Unit Power Fail (AWUPF)
    pub NVSCC: NVSCC, // byte 530.     M - NVM Vendor Specific Command Configuration (NVSCC)
    pub NWPC: NWPC,   // byte 531.     M - Namespace Write Protection Capabilities (NWPC)
    pub ACWU: u16,    // byte 532:533  O - Atomic Compare & Write Unit (ACWU)
    pub Reserved4: [u8; 2], // byte 534:535.
    pub SGLS: SGLS,   // byte 536:539. O - SGL Support (SGLS)
    pub MNAN: u32,    // byte 540:543. O - Maximum Number of Allowed Namespace (MNAN)
    pub Reserved6: [u8; 224], // byte 544:767.
    pub SUBNQN: [u8; 256], // byte 768:1023. M - NVM Subsystem NVMe Qualified Name (SUBNQN)
    pub Reserved7: [u8; 768], // byte 1024:1791
    pub Reserved8: [u8; 256], // byte 1792:2047. Refer to NVMe over Fabrics Specification
    pub PDS: [NVME_POWER_STATE_DESC; 32], // byte 2048:3071. M - Power State Descriptors
    pub VS: [u8; 1024], // byte 3072 : 4095.
}

impl Default for NVME_IDENTIFY_CONTROLLER_DATA {
    fn default() -> Self {
        NVME_IDENTIFY_CONTROLLER_DATA {
            VID: 0,
            SSVID: 0,
            SN: [0; 20],
            MN: [0; 40],
            FR: [0; 8],
            RAB: 0,
            IEEE: [0; 3],
            CMIC: Default::default(),
            MDTS: 0,
            CNTLID: 0,
            VER: 0,
            RTD3R: 0,
            RTD3E: 0,
            OAES: Default::default(),
            CTRATT: Default::default(),
            RRLS: Default::default(),
            Reserved0: [0; 9],
            CNTRLTYPE: 0,
            FGUID: [0; 16],
            CRDT1: 0,
            CRDT2: 0,
            CRDT3: 0,
            Reserved0_1: [0; 106],
            ReservedForManagement: [0; 16],
            OACS: Default::default(),
            ACL: 0,
            AERL: 0,
            FRMW: Default::default(),
            LPA: Default::default(),
            ELPE: 0,
            NPSS: 0,
            AVSCC: Default::default(),
            APSTA: Default::default(),
            WCTEMP: 0,
            CCTEMP: 0,
            MTFA: 0,
            HMPRE: 0,
            HMMIN: 0,
            TNVMCAP: [0; 16],
            UNVMCAP: [0; 16],
            RPMBS: Default::default(),
            EDSTT: 0,
            DSTO: 0,
            FWUG: 0,
            KAS: 0,
            HCTMA: Default::default(),
            MNTMT: 0,
            MXTMT: 0,
            SANICAP: Default::default(),
            HMMINDS: 0,
            HMMAXD: 0,
            NSETIDMAX: 0,
            ENDGIDMAX: 0,
            ANATT: 0,
            ANACAP: Default::default(),
            ANAGRPMAX: 0,
            NANAGRPID: 0,
            PELS: 0,
            Reserved1: [0; 156], // byte 356:511.
            SQES: Default::default(),
            CQES: Default::default(),
            MAXCMD: 0,
            NN: 0,
            ONCS: Default::default(),
            FUSES: Default::default(),
            FNA: Default::default(),
            VWC: Default::default(),
            AWUN: 0,
            AWUPF: 0,
            NVSCC: Default::default(),
            NWPC: Default::default(),
            ACWU: 0,
            Reserved4: [0; 2],
            SGLS: Default::default(),
            MNAN: 0,
            Reserved6: [0; 224],
            SUBNQN: [0; 256],
            Reserved7: [0; 768],
            Reserved8: [0; 256],
            PDS: Default::default(),
            VS: [0; 1024],
        }
    }
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CMIC {
    pub MultiPCIePorts: B1,
    pub MultiControllers: B1,
    pub SRIOV: B1,
    pub Reserved: B5,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct OAES {
    pub Reserved0: B8,
    pub NamespaceAttributeChanged: B1,
    pub FirmwareActivation: B1,
    pub Reserved1: B1,
    pub AsymmetricAccessChanged: B1,
    pub PredictableLatencyAggregateLogChanged: B1,
    pub LbaStatusChanged: B1,
    pub EnduranceGroupAggregateLogChanged: B1,
    pub Reserved2: B12,
    pub ZoneInformation: B1,
    pub Reserved3: B4,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CTRATT {
    pub HostIdentifier128Bit: B1,
    pub NOPSPMode: B1,
    pub NVMSets: B1,
    pub ReadRecoveryLevels: B1,
    pub EnduranceGroups: B1,
    pub PredictableLatencyMode: B1,
    pub TBKAS: B1,
    pub NamespaceGranularity: B1,
    pub SQAssociations: B1,
    pub UUIDList: B1,
    pub Reserved0: B22,
}

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RRLS {
    pub ReadRecoveryLevel0: B1,
    pub ReadRecoveryLevel1: B1,
    pub ReadRecoveryLevel2: B1,
    pub ReadRecoveryLevel3: B1,
    pub ReadRecoveryLevel4: B1,
    pub ReadRecoveryLevel5: B1,
    pub ReadRecoveryLevel6: B1,
    pub ReadRecoveryLevel7: B1,
    pub ReadRecoveryLevel8: B1,
    pub ReadRecoveryLevel9: B1,
    pub ReadRecoveryLevel10: B1,
    pub ReadRecoveryLevel11: B1,
    pub ReadRecoveryLevel12: B1,
    pub ReadRecoveryLevel13: B1,
    pub ReadRecoveryLevel14: B1,
    pub ReadRecoveryLevel15: B1,
}

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, Default)]
pub struct OACS {
    pub SecurityCommands: B1,
    pub FormatNVM: B1,
    pub FirmwareCommands: B1,
    pub NamespaceCommands: B1,
    pub DeviceSelfTest: B1,
    pub Directives: B1,
    pub NVMeMICommands: B1,
    pub VirtualizationMgmt: B1,
    pub DoorBellBufferConfig: B1,
    pub GetLBAStatus: B1,
    pub Reserved: B6,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FRMW {
    pub Slot1ReadOnly: B1,
    pub SlotCount: B3,
    pub ActivationWithoutReset: B1,
    pub Reserved: B3,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct LPA {
    pub SmartPagePerNamespace: B1,
    pub CommandEffectsLog: B1,
    pub LogPageExtendedData: B1,
    pub TelemetrySupport: B1,
    pub PersistentEventLog: B1,
    pub Reserved0: B1,
    pub TelemetryDataArea4: B1,
    pub Reserved1: B1,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AVSCC {
    pub CommandFormatInSpec: B1,
    pub Reserved: B7,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct APSTA {
    pub Supported: B1,
    pub Reserved: B7,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RPMBS {
    pub RPMBUnitCount: B3,        // Number of RPMB Units
    pub AuthenticationMethod: B3, // Authentication Method
    pub Reserved0: B10,
    pub TotalSize: B8,  // Total Size: in 128KB units.
    pub AccessSize: B8, // Access Size: in 512B units.
}

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, Default)]
pub struct HCTMA {
    pub Supported: B1,
    pub Reserved: B15,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SANICAP {
    pub CryptoErase: B1,
    pub BlockErase: B1,
    pub Overwrite: B1,
    pub Reserved: B26,
    pub NDI: B1,
    pub NODMMAS: B2,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ANACAP {
    pub OptimizedState: B1,
    pub NonOptimizedState: B1,
    pub InaccessibleState: B1,
    pub PersistentLossState: B1,
    pub ChangeState: B1,
    pub Reserved: B1,
    pub StaticANAGRPID: B1,
    pub SupportNonZeroANAGRPID: B1,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SQES {
    pub RequiredEntrySize: B4,
    pub MaxEntrySize: B4,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CQES {
    pub RequiredEntrySize: B4,
    pub MaxEntrySize: B4,
}

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ONCS {
    pub Compare: B1,
    pub WriteUncorrectable: B1,
    pub DatasetManagement: B1,
    pub WriteZeroes: B1,
    pub FeatureField: B1,
    pub Reservations: B1,
    pub Timestamp: B1,
    pub Verify: B1,
    pub Reserved: B8,
}

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FUSES {
    pub CompareAndWrite: B1,
    pub Reserved: B15,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FNA {
    pub FormatApplyToAll: B1,
    pub SecureEraseApplyToAll: B1,
    pub CryptographicEraseSupported: B1,
    pub FormatSupportNSIDAllF: B1,
    pub Reserved: B4,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct VWC {
    pub Present: B1,
    pub FlushBehavior: B2,
    pub Reserved: B5,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVSCC {
    pub CommandFormatInSpec: B1,
    pub Reserved: B7,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NWPC {
    pub WriteProtect: B1,
    pub UntilPowerCycle: B1,
    pub Permanent: B1,
    pub Reserved: B5,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SGLS {
    pub SGLSupported: B2,
    pub KeyedSGLData: B1,
    pub Reserved0: B13,
    pub BitBucketDescrSupported: B1,
    pub ByteAlignedContiguousPhysicalBuffer: B1,
    pub SGLLengthLargerThanDataLength: B1,
    pub MPTRSGLDescriptor: B1,
    pub AddressFieldSGLDataBlock: B1,
    pub TransportSGLData: B1,
    pub Reserved1: B10,
}

//
// Namespace Identfier Type (NIDT)
//
pub enum NVME_IDENTIFIER_TYPE {
    NVME_IDENTIFIER_TYPE_EUI64 = 0x1,
    NVME_IDENTIFIER_TYPE_NGUID = 0x2,
    NVME_IDENTIFIER_TYPE_UUID = 0x3,
    NVME_IDENTIFIER_TYPE_CSI = 0x4,
}

//
// Namespace Identfier Length (NIDL) for a given type defined by NVME_IDENTIFIER_TYPE
//
pub enum NVME_IDENTIFIER_TYPE_LENGTH {
    NVME_IDENTIFIER_TYPE_EUI64_LENGTH = 0x8,
    NVME_IDENTIFIER_TYPE_NGUID_LENGTH = 0x10,
    // NVME_IDENTIFIER_TYPE_UUID_LENGTH = 0x10,
    NVME_IDENTIFIER_TYPE_CSI_LENGTH = 0x1,
}

//
// Output of NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE (0x03)
//
const NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE_SIZE: usize = 0x1000;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_IDENTIFY_NAMESPACE_DESCRIPTOR {
    pub NIDT: u8, // Namespace Identifier Type as defined in NVME_IDENTIFIER_TYPE
    pub NIDL: u8, // Namespace Identifier Length
    pub Reserved: [u8; 2],
    pub NID: [u8; 1], // Namespace Identifier (Based on NVME_IDENTIFIER_TYPE)
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_SET_ATTRIBUTES_ENTRY {
    pub Identifier: u16,
    pub ENDGID: u16,
    pub Reserved1: u32,
    pub Random4KBReadTypical: u32,
    pub OptimalWriteSize: u32,
    pub TotalCapacity: [u8; 16],
    pub UnallocatedCapacity: [u8; 16],
    pub Reserved2: [u8; 80],
}
impl Default for NVME_SET_ATTRIBUTES_ENTRY {
    fn default() -> Self {
        NVME_SET_ATTRIBUTES_ENTRY {
            Identifier: 0,
            ENDGID: 0,
            Reserved1: 0,
            Random4KBReadTypical: 0,
            OptimalWriteSize: 0,
            TotalCapacity: [0; 16],
            UnallocatedCapacity: [0; 16],
            Reserved2: [0; 80],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVM_SET_LIST {
    pub IdentifierCount: u8,
    Reserved: [u8; 127],
    pub Entry: [NVME_SET_ATTRIBUTES_ENTRY; 1],
}

impl Default for NVM_SET_LIST {
    fn default() -> Self {
        NVM_SET_LIST {
            IdentifierCount: 0,
            Reserved: [0; 127],
            Entry: [NVME_SET_ATTRIBUTES_ENTRY::default(); 1],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_LBA_ZONE_FORMAT {
    pub ZoneSize: u64, // bit 0:63 Zone Size (MS)
    pub ZDES: u8,      // bit 64:71 Zone Descriptor Extension Size (ZDES)
    Reserved: [u8; 7],
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET {
    pub ZOC: ZOC,
    pub OZCS: OZCS,
    pub MAR: u32,
    pub MOR: u32,
    pub RRL: u32,
    pub FRL: u32,
    pub Reserved0: [u8; 2796],
    pub LBAEF: [NVME_LBA_ZONE_FORMAT; 16],
    pub Reserved1: [u8; 768],
    pub VS: [u8; 256],
}
impl Default for NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET {
    fn default() -> Self {
        NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET {
            ZOC: ZOC::default(),
            OZCS: OZCS::default(),
            MAR: 0,
            MOR: 0,
            RRL: 0,
            FRL: 0,
            Reserved0: [0; 2796],
            LBAEF: [NVME_LBA_ZONE_FORMAT::default(); 16],
            Reserved1: [0; 768],
            VS: [0; 256],
        }
    }
}

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ZOC {
    pub VariableZoneCapacity: B1,
    pub ZoneExcursions: B1,
    Reserved: B14,
}

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, Default)]
pub struct OZCS {
    pub ReadAcrossZoneBoundaries: B1,
    Reserved: B15,
}
//
// Output of NVME_IDENTIFY_CNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET (0x06) with Command Set Identifier (0x00)
//
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    pub VSL: u8,              // byte 0       O - Verify Size Limit (VZL)
    pub WZSL: u8,             // byte 1       O - Write Zeroes Size Limit (WZSL)
    pub WUSL: u8,             // byte 2       O - Write Uncorrectable Size Limit (WUSL)
    pub DMRL: u8,             // byte 3       O - Dataset Management Ranges Limit (DMRL)
    pub DMRSL: u32,           // byte 4:7     O - Dataset Management Range Size Limit (DMRSL)
    pub DMSL: u64,            // byte 8:15    O - Dataset Management Size Limit (DMSL)
    pub Reserved: [u8; 4080], // byte 16:4095
}

impl Default for NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    fn default() -> Self {
        NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
            VSL: 0,
            WZSL: 0,
            WUSL: 0,
            DMRL: 0,
            DMRSL: 0,
            DMSL: 0,
            Reserved: [0; 4080],
        }
    }
}

//
// Output of NVME_IDENTIFY_CNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET (0x06) with Command Set Identifier (0x02)
//
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    pub ZASL: u8,         // byte 0.          O - Zone Append Size Limit (ZASL)
    Reserved: [u8; 4095], // byte 1:4095
}

impl Default for NVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    fn default() -> Self {
        NVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
            ZASL: 0,
            Reserved: [0; 4095],
        }
    }
}

//
// Output of NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NSID (0x12)/NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NVM_SUBSYSTEM (0x13)
//
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_CONTROLLER_LIST {
    pub NumberOfIdentifiers: u16,
    pub ControllerID: [u16; 2047],
}

impl Default for NVME_CONTROLLER_LIST {
    fn default() -> Self {
        NVME_CONTROLLER_LIST {
            NumberOfIdentifiers: 0,
            ControllerID: [0; 2047],
        }
    }
}

//
// Output of NVME_IDENTIFY_CNS_IO_COMMAND_SET (0x1C)
//
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_IDENTIFY_IO_COMMAND_SET {
    pub IOCommandSetVector: [u64; 512],
}

impl Default for NVME_IDENTIFY_IO_COMMAND_SET {
    fn default() -> Self {
        NVME_IDENTIFY_IO_COMMAND_SET {
            IOCommandSetVector: [0; 512],
        }
    }
}

//
// Data Structure of LBA Range Type entry
//
pub enum NVME_LBA_RANGE_TYPES {
    NVME_LBA_RANGE_TYPE_RESERVED = 0,
    NVME_LBA_RANGE_TYPE_FILESYSTEM = 1,
    NVME_LBA_RANGE_TYPE_RAID = 2,
    NVME_LBA_RANGE_TYPE_CACHE = 3,
    NVME_LBA_RANGE_TYPE_PAGE_SWAP_FILE = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_LBA_RANGE_TYPE_ENTRY {
    pub Type: u8, // Type (Type): Specifies the Type of the LBA range.
    pub Attributes: NVME_LBA_RANGE_TYPE_ATTRIBUTES, // Attributes: Specifies attributes of the LBA range. Each bit defines an attribute.
    pub Reserved0: [u8; 14],
    pub SLBA: u64, // Starting LBA (SLBA): This field specifies the 64-bit address of the first logical block that is part of this LBA range.
    pub NLB: u64, // Number of Logical Blocks (NLB): This field specifies the number of logical blocks that are part of this LBA range. This is a 0s based value.
    pub GUID: [u8; 16], // Unique Identifier (GUID): This field is a global unique identifier that uniquely specifies the type of this LBA range. Well known Types may be defined and are published on the NVM Express website.
    pub Reserved1: [u8; 16],
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_LBA_RANGE_TYPE_ATTRIBUTES {
    pub MayOverwritten: B1,
    pub Hidden: B1,
    Reserved: B6,
}

//
// Vendor defined log pages
//
pub enum NVME_VENDOR_LOG_PAGES {
    NVME_LOG_PAGE_WCS_DEVICE_SMART_ATTRIBUTES = 0xC0, // WCS device SMART Attributes log page
    NVME_LOG_PAGE_WCS_DEVICE_ERROR_RECOVERY = 0xC1,   // WCS device Error Recovery log page
}

//
// SMART Attributes Log Page GUID is defined in spec as byte stream: 0xAFD514C97C6F4F9CA4F2BFEA2810AFC5
// which is converted to GUID format as: {2810AFC5-BFEA-A4F2-9C4F-6F7CC914D5AF}
//
pub const GUID_WCS_DEVICE_SMART_ATTRIBUTES: [u8; 16] = [
    0x28, 0x10, 0xAF, 0xC5, 0xBF, 0xEA, 0xA4, 0xF2, 0x9C, 0x4F, 0x6F, 0x7C, 0xC9, 0x14, 0xD5, 0xAF,
];

//
// Error Recovery Log Page GUID is defined in spec as byte stream: 0x5A1983BA3DFD4DABAE3430FE2131D944
// which is converted to GUID format as: {2131D944-30FE-AE34-AB4D-FD3DBA83195A}
//
pub const GUID_WCS_DEVICE_ERROR_RECOVERY: [u8; 16] = [
    0x21, 0x31, 0xD9, 0x44, 0x30, 0xFE, 0xAE, 0x34, 0xAB, 0x4D, 0xFD, 0x3D, 0xBA, 0x83, 0x19, 0x5A,
];

//
// Notice Status: NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC
//
pub enum NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_CODES {
    NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_RESERVED = 0,
    NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_DEVICE_PANIC = 1,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_RESET_ACTION {
    pub ControllerReset: bool,
    pub NVMeSubsystemReset: bool,
    pub PCIeFLR: bool,
    pub PERST: bool,
    pub PowerCycle: bool,
    pub PCIeConventionalHotReset: bool,
    pub Reserved: B2,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_CAPABILITIES {
    pub PanicAEN: bool,
    pub PanicCFS: bool,
    pub Reserved: B30,
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_WCS_DEVICE_RECOVERY_ACTION {
    NVMeDeviceRecoveryNoAction = 0,          // Requires no action
    NVMeDeviceRecoveryFormatNVM,             // Requires Format NVM
    NVMeDeviceRecoveryVendorSpecificCommand, // Requires Vendor Specific Command
    NVMeDeviceRecoveryVendorAnalysis,        // Requires Vendor Analysis
    NVMeDeviceRecoveryDeviceReplacement,     // Requires Device Replacement
    NVMeDeviceRecoverySanitize,              // Requires Sanitize
    NVMeDeviceRecoveryMax = 15,              // Not an actual action, denotes max action.
}
#[repr(C, packed)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG {
    pub VersionSpecificData: [u8; 494],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: [u8; 16], // GUID_WCS_DEVICE_SMART_ATTRIBUTES
}

impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG {
    fn default() -> Self {
        NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG {
            VersionSpecificData: [0; 494],
            LogPageVersionNumber: 0,
            LogPageGUID: GUID_WCS_DEVICE_SMART_ATTRIBUTES,
        }
    }
}

#[repr(C, packed)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 {
    pub MediaUnitsWritten: [u8; 16],
    pub MediaUnitsRead: [u8; 16],
    pub BadUserNANDBlockCount: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_Count,
    pub BadSystemNANDBlockCount: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_Count,
    pub XORRecoveryCount: u64,
    pub UnrecoverableReadErrorCount: u64,
    pub SoftECCErrorCount: u64,
    pub EndToEndCorrectionCounts: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_CorrectionCounts,
    pub PercentageSystemDataUsed: u8,
    pub RefreshCount: [u8; 7],
    pub UserDataEraseCounts: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_EraseCounts,
    pub ThermalThrottling: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_ThermalThrottling,
    pub Reserved0: [u8; 6],
    pub PCIeCorrectableErrorCount: u64,
    pub IncompleteShutdownCount: u32,
    pub Reserved1: u32,
    pub PercentageFreeBlocks: u8,
    pub Reserved2: [u8; 7],
    pub CapacitorHealth: u16,
    pub Reserved3: [u8; 6],
    pub UnalignedIOCount: u64,
    pub SecurityVersionNumber: u64,
    pub NUSE: u64,
    pub PLPStartCount: [u8; 16],
    pub EnduranceEstimate: [u8; 16],
    pub Reserved4: [u8; 302],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: [u8; 16], // GUID_WCS_DEVICE_SMART_ATTRIBUTES
}

impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 {
    fn default() -> Self {
        NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 {
            MediaUnitsWritten: [0; 16],
            MediaUnitsRead: [0; 16],
            BadUserNANDBlockCount: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_Count::default(),
            BadSystemNANDBlockCount: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_Count::default(),
            XORRecoveryCount: 0,
            UnrecoverableReadErrorCount: 0,
            SoftECCErrorCount: 0,
            EndToEndCorrectionCounts:
                NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_CorrectionCounts::default(),
            PercentageSystemDataUsed: 0,
            RefreshCount: [0; 7],
            UserDataEraseCounts: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_EraseCounts::default(),
            ThermalThrottling: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_ThermalThrottling::default(),
            Reserved0: [0; 6],
            PCIeCorrectableErrorCount: 0,
            IncompleteShutdownCount: 0,
            Reserved1: 0,
            PercentageFreeBlocks: 0,
            Reserved2: [0; 7],
            CapacitorHealth: 0,
            Reserved3: [0; 6],
            UnalignedIOCount: 0,
            SecurityVersionNumber: 0,
            NUSE: 0,
            PLPStartCount: [0; 16],
            EnduranceEstimate: [0; 16],
            Reserved4: [0; 302],
            LogPageVersionNumber: 0,
            LogPageGUID: GUID_WCS_DEVICE_SMART_ATTRIBUTES,
        }
    }
}

#[repr(C, packed)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_Count {
    pub RawCount: [u8; 6],
    pub Normalized: [u8; 2],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_CorrectionCounts {
    pub DetectedCounts: u32,
    pub CorrectedCounts: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_EraseCounts {
    pub MaximumCount: u32,
    pub MinimumCount: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_ThermalThrottling {
    pub EventCount: u8,
    pub Status: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_WCS_DEVICE_ERROR_RECOVERY_LOG {
    pub PanicResetWaitTime: u16,
    pub PanicResetAction: NVME_WCS_DEVICE_RESET_ACTION,
    pub DriveRecoveryAction: u8,
    pub PanicId: u64,
    pub DeviceCapabilities: NVME_WCS_DEVICE_CAPABILITIES,
    pub VendorSpecificRecoveryCode: u8,
    pub Reserved0: [u8; 3],
    pub VendorSpecificCommandCDW12: u32,
    pub VendorSpecificCommandCDW13: u32,
    pub Reserved1: [u8; 466],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: [u8; 16], // GUID_WCS_DEVICE_ERROR_RECOVERY
}
impl Default for NVME_WCS_DEVICE_ERROR_RECOVERY_LOG {
    fn default() -> Self {
        NVME_WCS_DEVICE_ERROR_RECOVERY_LOG {
            PanicResetWaitTime: 0,
            PanicResetAction: NVME_WCS_DEVICE_RESET_ACTION::default(),
            DriveRecoveryAction: NVME_WCS_DEVICE_RECOVERY_ACTION::NVMeDeviceRecoveryNoAction as u8,
            PanicId: 0,
            DeviceCapabilities: NVME_WCS_DEVICE_CAPABILITIES::default(),
            VendorSpecificRecoveryCode: 0,
            Reserved0: [0; 3],
            VendorSpecificCommandCDW12: 0,
            VendorSpecificCommandCDW13: 0,
            Reserved1: [0; 466],
            LogPageVersionNumber: 0,
            LogPageGUID: GUID_WCS_DEVICE_ERROR_RECOVERY,
        }
    }
}

//
// Parameters for NVME_ADMIN_COMMAND_CREATE_IO_CQ
//
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_CREATE_IO_QUEUE {
    pub QID: u16,   // Queue Identifier (QID)
    pub QSIZE: u16, // Queue Size (QSIZE)
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_CREATE_IO_CQ {
    pub PC: bool,   // Physically Contiguous (PC)
    pub IEN: bool,  // Interrupts Enabled (IEN)
    Reserved0: B14, // Reserved
    pub IV: B16,    // Interrupt Vector (IV)
}

//
// Parameters for NVME_ADMIN_COMMAND_CREATE_IO_SQ
//
#[derive(Debug, Clone, Copy)]
pub enum NVME_NVM_QUEUE_PRIORITIES {
    NVME_NVM_QUEUE_PRIORITY_URGENT = 0,
    NVME_NVM_QUEUE_PRIORITY_HIGH = 1,
    NVME_NVM_QUEUE_PRIORITY_MEDIUM = 2,
    NVME_NVM_QUEUE_PRIORITY_LOW = 3,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_CREATE_IO_SQ {
    pub PC: bool,   // Physically Contiguous (PC)
    pub QPRIO: B2,  // Queue Priority (QPRIO)
    Reserved0: B13, // Reserved
    pub CQID: B16,  // Completion Queue Identifier (CQID)
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_FEATURE_VALUE_CODES {
    NVME_FEATURE_VALUE_CURRENT = 0,
    NVME_FEATURE_VALUE_DEFAULT = 1,
    NVME_FEATURE_VALUE_SAVED = 2,
    NVME_FEATURE_VALUE_SUPPORTED_CAPABILITIES = 3,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_GET_FEATURES {
    pub FID: B8, // Feature Identifier (FID)
    pub SEL: B3, // Select (SEL): This field specifies which value of the attributes to return in the provided data.
    pub Reserved0: B21,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_SET_FEATURES {
    pub FID: B8, // Feature Identifier (FID)
    pub Reserved0: B23,
    pub SV: B1, // Save (SV)
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_NUMBER_OF_QUEUES {
    pub NSQ: B16, // Number of IO Submission Queues.
    pub NCQ: B16, // Number of IO Completion Queues.
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_INTERRUPT_COALESCING {
    pub THR: B8,  // Aggregation Threshold (THR)
    pub TIME: B8, // Aggregation Time (TIME)
    pub Reserved0: B16,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG {
    pub IV: B16, // Interrupt Vector (IV)
    pub CD: B1,  // Coalescing Disabled (CD)
    pub Reserved0: B15,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL {
    pub DN: B1, // Disable Normal (DN)
    pub Reserved0: B31,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE {
    pub NOPPME: B1, // Non-Operational Power State Permissive Mode Enable (NOPPME)
    pub Reserved0: B31,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_LBA_RANGE_TYPE {
    pub NUM: B6, // Number of LBA Ranges (NUM)
    pub Reserved0: B26,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_ARBITRATION {
    pub AB: B3, // Arbitration Burst (AB)
    pub Reserved0: B5,
    pub LPW: B8, // Low Priority Weight (LPW)
    pub MPW: B8, // Medium Priority Weight (MPW)
    pub HPW: B8, // High Priority Weight (HPW)
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE {
    pub WCE: B1, // Volatile Write Cache Enable (WCE)
    pub Reserved0: B31,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY {
    pub SAVE: B1, // Save supported
    pub NSS: B1,  // Namespace specific
    pub MOD: B1,  // Changeable
    pub Reserved0: B29,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG {
    pub CriticalWarnings: B8,            // SMART / Health Critical Warnings
    pub NsAttributeNotices: B1,          // Namespace Attributes Notices
    pub FwActivationNotices: B1,         // Firmware Activation Notices
    pub TelemetryLogNotices: B1,         // Telemetry Log Notices
    pub ANAChangeNotices: B1,            // Asymmetric Namespace Access Change Notices
    pub PredictableLogChangeNotices: B1, // Predictable Latency Event Aggregate Log Change Notices
    pub LBAStatusNotices: B1,            // LBA Status Information Notices
    pub EnduranceEventNotices: B1,       // Endurance Group Event Aggregate Log Change Notices
    pub Reserved0: B12,
    pub ZoneDescriptorNotices: B1, // Zone Descriptor Changed Notices
    pub Reserved1: B4,
}

//
// Parameter for NVME_FEATURE_POWER_MANAGEMENT
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_POWER_MANAGEMENT {
    pub PS: B5,         // Power State (PS)
    pub Reserved0: B27, // Reserved
}

// Parameter for NVME_FEATURE_AUTONOMOUS_POWER_STATE_TRANSITION
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION {
    pub APSTE: B1, // Autonomous Power State Transition Enable (APSTE)
    pub Reserved0: B31,
}
//
// Parameter for NVME_FEATURE_AUTONOMOUS_POWER_STATE_TRANSITION
// There is an array of 32 of these (one for each power state) in the data buffer.
//
#[bitfield]
#[repr(u64)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_AUTO_POWER_STATE_TRANSITION_ENTRY {
    pub Reserved0: B3,                  // Bits 0-2 are reserved.
    pub IdleTransitionPowerState: B5, // Bits 3-7 - (ITPS) The non-operational power state for the controller to autonomously transition to after there is a continuous period of idle time in the current power state that exceeds time specified in the ITPT field.
    pub IdleTimePriorToTransition: B24, // Bits 8-31 - (ITPT) The amount of idle time (in ms) that occurs in this power state prior to transitioning to the Idle Transition Power State. A value of 0 disables APST for this power state.
    pub Reserved1: B32,                 // Bits 32-63 are reserved.
}

//
// Parameter for NVME_FEATURE_TEMPERATURE_THRESHOLD
//

//
// Following definitions are used in "THSEL" field.
//
#[derive(Debug, Clone, Copy)]
pub enum NVME_TEMPERATURE_THRESHOLD_TYPES {
    NVME_TEMPERATURE_OVER_THRESHOLD = 0,
    NVME_TEMPERATURE_UNDER_THRESHOLD = 1,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD {
    pub TMPTH: B16, // Temperature Threshold (TMPTH): Indicates the threshold for the temperature of the overall device (controller and NVM included) in units of Kelvin.
    pub TMPSEL: B4, // Threshold Temperature Select (TMPSEL)
    pub THSEL: B2,  // Threshold Type Select (THSEL)
    pub Reserved0: B10, // Reserved
}

//
// Parameter for NVME_FEATURE_ERROR_RECOVERY
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_ERROR_RECOVERY {
    pub TLER: B16,      // Time limited error recovery (TLER)
    pub DULBE: B1,      // Deallocated or unwritten logical block error enable (DULBE)
    pub Reserved0: B15, // Reserved
}
// Parameters for NVME_FEATURE_HOST_MEMORY_BUFFER
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER {
    pub EHM: B1, // Enable Host Memory (EHM) - Enables the host memory buffer.
    pub MR: B1, // Memory Return (MR) - Indicates if the host is returning previously allocated memory to the controller.
    pub Reserved: B30,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER {
    pub HSIZE: u32, // Host Memory Buffer Size (HSIZE) - The size of the host memory buffer in memory page size (CC.MPS) units.
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER {
    pub Reserved: B4,
    pub HMDLLA: B28, // Host Memory Descriptor List Lower Address (HMDLLA) - 16-byte aligned, lower 32 bits of the physical location of the Host Memory Descriptor List.
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER {
    pub HMDLUA: u32, // Host Memory Descriptor List Upper Address (HMDLLA) - Upper 32 bits of the physical location of the Host Memory Descriptor List.
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER {
    pub HMDLEC: u32, // Host Memory Descriptor List Entry Count (HMDLEC) - Number of entries in the Host Memory Descriptor List.
}

//
// This structure is a single entry in the host memory descriptor list.
//
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_HOST_MEMORY_BUFFER_DESCRIPTOR_ENTRY {
    pub BADD: u64, // Buffer Address (BADD) - Physical host memory address aligned to the memory page size (CC.MPS)
    pub BSIZE: u32, // Buffer Size (BSIZE) - The number of contiguous memory page size (CC.MPS) units for this entry.
    Reserved: u32,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE {
    pub IOCSCI: B8, // I/O command Set Profile
    pub Reserved: B24,
}
// Parameters for NVME_FEATURE_ENHANDED_CONTROLLER_METADATA, NVME_FEATURE_CONTROLLER_METADATA, NVME_FEATURE_NAMESPACE_METADATA
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_GET_HOST_METADATA {
    pub GDHM: B1, // Generate Default Host Metadata (GDHM)
    pub Reserved: B31,
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_HOST_METADATA_ELEMENT_ACTIONS {
    NVME_HOST_METADATA_ADD_REPLACE_ENTRY = 0,
    NVME_HOST_METADATA_DELETE_ENTRY_MULTIPLE = 1,
    NVME_HOST_METADATA_ADD_ENTRY_MULTIPLE = 2,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_SET_HOST_METADATA {
    pub Reserved0: B13,
    pub EA: B2, // Element Action (EA), value defined in enum NVME_HOST_METADATA_ELEMENT_ACTIONS
    pub Reserved1: B17,
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_CONTROLLER_METADATA_ELEMENT_TYPES {
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_CONTROLLER_NAME = 0x1,
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_NAME = 0x2,
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_VERSION = 0x3,
    NVME_CONTROLLER_METADATA_PREBOOT_CONTROLLER_NAME = 0x4,
    NVME_CONTROLLER_METADATA_PREBOOT_DRIVER_NAME = 0x5,
    NVME_CONTROLLER_METADATA_PREBOOT_DRIVER_VERSION = 0x6,
    NVME_CONTROLLER_METADATA_SYSTEM_PROCESSOR_MODEL = 0x7,
    NVME_CONTROLLER_METADATA_CHIPSET_DRIVER_NAME = 0x8,
    NVME_CONTROLLER_METADATA_CHIPSET_DRIVER_VERSION = 0x9,
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_NAME_AND_BUILD = 0xA,
    NVME_CONTROLLER_METADATA_SYSTEM_PRODUCT_NAME = 0xB,
    NVME_CONTROLLER_METADATA_FIRMWARE_VERSION = 0xC,
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_FILENAME = 0xD,
    NVME_CONTROLLER_METADATA_DISPLAY_DRIVER_NAME = 0xE,
    NVME_CONTROLLER_METADATA_DISPLAY_DRIVER_VERSION = 0xF,
    NVME_CONTROLLER_METADATA_HOST_DETERMINED_FAILURE_RECORD = 0x10,
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_NAMESPACE_METADATA_ELEMENT_TYPES {
    NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME = 0x1,
    NVME_NAMESPACE_METADATA_PREBOOT_NAMESPACE_NAME = 0x2,
    NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME_QUALIFIER_1 = 0x3,
    NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME_QUALIFIER_2 = 0x4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_HOST_METADATA_ELEMENT_DESCRIPTOR {
    pub FIELD: NVME_HOST_METADATA_ELEMENT_DESCRIPTOR_FIELD, // Element Length (ELEN), element value length in bytes
    pub EVAL: [u8; 0],                                      // Element Value (EVAL), UTF-8 string
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_HOST_METADATA_ELEMENT_DESCRIPTOR_FIELD {
    pub ET: B6, // Element Type (ET), value defined in enum NVME_CONTROLLER_METADATA_ELEMENT_TYPES, NVME_NAMESPACE_METADATA_ELEMENT_TYPES
    pub Reserved0: B2,
    pub ER: B4, // Element Revision (ER)
    pub Reserved1: B4,
    pub ELEN: B16, // Element Length (ELEN), element value length in bytes
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_FEATURE_HOST_METADATA_DATA {
    pub NumberOfMetadataElementDescriptors: u8,
    Reserved0: u8,
    pub MetadataElementDescriptors: [u8; 4094], // Use NVME_HOST_METADATA_ELEMENT_DESCRIPTOR to access this list.
}

impl Default for NVME_FEATURE_HOST_METADATA_DATA {
    fn default() -> Self {
        NVME_FEATURE_HOST_METADATA_DATA {
            NumberOfMetadataElementDescriptors: 0,
            Reserved0: 0,
            MetadataElementDescriptors: [0; 4094],
        }
    }
}

//
// Parameter for NVME_FEATURE_ERROR_INJECTION
// This is from OCP NVMe Cloud SSD spec.
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_ERROR_INJECTION {
    pub NUM: B7,        // Number of Error Injections.
    pub Reserved0: B25, // Reserved
}

//
// DWORD 0 for get feature command (Error Injection) shares the same format with DWORD 11 for set feature command (Error Injection).
//
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_ERROR_INJECTION_ENTRY {
    pub Flags: NVME_ERROR_INJECTION_FLAGS,
    Reserved1: u8,
    pub ErrorInjectionType: u16,
    pub ErrorInjectionTypeSpecific: [u8; 28],
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_ERROR_INJECTION_FLAGS {
    pub Enable: B1,
    pub SingleInstance: B1,
    pub Reserved0: B6,
}

//
// Definitions are used in "Error Injection Type" field.
//
#[derive(Debug, Clone, Copy)]
pub enum NVME_ERROR_INJECTION_TYPES {
    NVME_ERROR_INJECTION_TYPE_RESERVED0 = 0,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_CPU_CONTROLLER_HANG = 1,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_NAND_HANG = 2,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_PLP_DEFECT = 3,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_LOGICAL_FW_ERROR = 4,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_DRAM_CORRUPTION_CRITICAL = 5,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_DRAM_CORRUPTION_NONCRITICAL = 6,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_NAND_CORRUPTION = 7,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_SRAM_CORRUPTION = 8,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_HW_MALFUNCTION = 9,
    NVME_ERROR_INJECTION_TYPE_RESERVED1 = 10,
    NVME_ERROR_INJECTION_TYPE_MAX = 0xFFFF,
}
// Parameter for set feature NVME_FEATURE_CLEAR_FW_UPDATE_HISTORY
// This is from OCP NVMe Cloud SSD spec.
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY {
    Reserved0: B31,
    pub Clear: B1, // Clear Firmware Update History Log.
}

// Parameter for set feature NVME_FEATURE_READONLY_WRITETHROUGH_MODE
// This is from OCP NVMe Cloud SSD spec.
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE {
    Reserved0: B30,
    pub EOLBehavior: B2, // End of Life Behavior.
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE {
    pub EOLBehavior: B3, // End of Life Behavior.
    Reserved0: B29,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS {
    Reserved0: B31,
    pub Clear: B1, // Clear PCIe Error Counters.
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO {
    Reserved0: B31,
    pub Enable: B1, // Enable IEEE1667 Silo.
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO {
    pub Enabled: B3, // IEEE1667 Silo Enabled.
    Reserved0: B29,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_HOST_IDENTIFIER {
    pub EXHID: B1, // Enable Extended Host Identifier (EXHID)
    Reserved: B31,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_FEATURE_HOST_IDENTIFIER_DATA {
    pub HOSTID: [u8; 16], // Host Identifier (HOSTID)
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE {
    pub PTPL: B1, // Persist Through Power Loss (PTPL)
    Reserved: B31,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK {
    Reserved: B1,
    pub REGPRE: B1, // Mask Registration Preempted Notification (REGPRE)
    pub RESREL: B1, // Mask Reservation Released Notification (RESREL)
    pub RESPRE: B1, // Mast Reservation Preempted Notification (RESPRE)
    Reserved1: B28,
}

#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURES {
    NumberOfQueues: NVME_CDW11_FEATURE_NUMBER_OF_QUEUES,
    InterruptCoalescing: NVME_CDW11_FEATURE_INTERRUPT_COALESCING,
    InterruptVectorConfig: NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG,
    LbaRangeType: NVME_CDW11_FEATURE_LBA_RANGE_TYPE,
    Arbitration: NVME_CDW11_FEATURE_ARBITRATION,
    VolatileWriteCache: NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE,
    AsyncEventConfig: NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG,
    PowerManagement: NVME_CDW11_FEATURE_POWER_MANAGEMENT,
    AutoPowerStateTransition: NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION,
    TemperatureThreshold: NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD,
    ErrorRecovery: NVME_CDW11_FEATURE_ERROR_RECOVERY,
    HostMemoryBuffer: NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER,
    WriteAtomicityNormal: NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL,
    NonOperationalPowerState: NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE,
    IoCommandSetProfile: NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE,
    ErrorInjection: NVME_CDW11_FEATURE_ERROR_INJECTION,
    HostIdentifier: NVME_CDW11_FEATURE_HOST_IDENTIFIER,
    ReservationPersistence: NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE,
    ReservationNotificationMask: NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK,
    GetHostMetadata: NVME_CDW11_FEATURE_GET_HOST_METADATA,
    SetHostMetadata: NVME_CDW11_FEATURE_SET_HOST_METADATA,
    AsUlong: u32,
}

impl Default for NVME_CDW11_FEATURES {
    fn default() -> Self {
        NVME_CDW11_FEATURES { AsUlong: 0 }
    }
}

#[derive(Clone, Copy)]
pub union NVME_CDW12_FEATURES {
    pub HostMemoryBuffer: NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: u32,
}

impl Default for NVME_CDW12_FEATURES {
    fn default() -> Self {
        NVME_CDW12_FEATURES { AsUlong: 0 }
    }
}

#[derive(Clone, Copy)]
pub union NVME_CDW13_FEATURES {
    pub HostMemoryBuffer: NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: u32,
}

impl Default for NVME_CDW13_FEATURES {
    fn default() -> Self {
        NVME_CDW13_FEATURES { AsUlong: 0 }
    }
}

#[derive(Clone, Copy)]
pub union NVME_CDW14_FEATURES {
    pub HostMemoryBuffer: NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: u32,
}

impl Default for NVME_CDW14_FEATURES {
    fn default() -> Self {
        NVME_CDW14_FEATURES { AsUlong: 0 }
    }
}

#[derive(Clone, Copy)]
pub union NVME_CDW15_FEATURES {
    pub HostMemoryBuffer: NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: u32,
}

impl Default for NVME_CDW15_FEATURES {
    fn default() -> Self {
        NVME_CDW15_FEATURES { AsUlong: 0 }
    }
}

//
// NVMe Maximum log size
//
pub const NVME_MAX_LOG_SIZE: usize = 0x1000;

//
// Parameters for NVME_ADMIN_COMMAND_GET_LOG_PAGE Command
//
#[derive(Debug, Clone, Copy)]
pub enum NVME_LOG_PAGES {
    NVME_LOG_PAGE_ERROR_INFO = 0x01,
    NVME_LOG_PAGE_HEALTH_INFO = 0x02,
    NVME_LOG_PAGE_FIRMWARE_SLOT_INFO = 0x03,
    NVME_LOG_PAGE_CHANGED_NAMESPACE_LIST = 0x04,
    NVME_LOG_PAGE_COMMAND_EFFECTS = 0x05,
    NVME_LOG_PAGE_DEVICE_SELF_TEST = 0x06,
    NVME_LOG_PAGE_TELEMETRY_HOST_INITIATED = 0x07,
    NVME_LOG_PAGE_TELEMETRY_CTLR_INITIATED = 0x08,
    NVME_LOG_PAGE_ENDURANCE_GROUP_INFORMATION = 0x09,
    NVME_LOG_PAGE_PREDICTABLE_LATENCY_NVM_SET = 0x0A,
    NVME_LOG_PAGE_PREDICTABLE_LATENCY_EVENT_AGGREGATE = 0x0B,
    NVME_LOG_PAGE_ASYMMETRIC_NAMESPACE_ACCESS = 0x0C,
    NVME_LOG_PAGE_PERSISTENT_EVENT_LOG = 0x0D,
    NVME_LOG_PAGE_LBA_STATUS_INFORMATION = 0x0E,
    NVME_LOG_PAGE_ENDURANCE_GROUP_EVENT_AGGREGATE = 0x0F,
    NVME_LOG_PAGE_RESERVATION_NOTIFICATION = 0x80,
    NVME_LOG_PAGE_SANITIZE_STATUS = 0x81,
    NVME_LOG_PAGE_CHANGED_ZONE_LIST = 0xBF,
}

//
// Get LOG PAGE format which confines to  < 1.3 NVMe Specification
//
// #[derive(Clone, Copy)]
// union NVME_CDW10_GET_LOG_PAGE {
//     bits: u32,
//     fields: NVME_CDW10_GET_LOG_PAGE_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_GET_LOG_PAGE {
    pub LID: B8, // Log Page Identifier (LID)
    Reserved0: B8,
    pub NUMD: B12, // Number of Dwords (NUMD)
    Reserved1: B4,
}

//
// Get LOG PAGE format which confines to  >= 1.3 NVMe Specification
//
// #[derive(Clone, Copy)]
// union NVME_CDW10_GET_LOG_PAGE_V13 {
//     bits: u32,
//     fields: NVME_CDW10_GET_LOG_PAGE_V13_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_GET_LOG_PAGE_V13 {
    pub LID: B8, // Log Page Identifier (LID)
    pub LSP: B4, // Log Specific Field (LSP)
    Reserved0: B3,
    pub RAE: B1,    // Retain Asynchronous Event (RAE)
    pub NUMDL: B16, // Number of Lower Dwords (NUMDL)
}

// #[derive(Clone, Copy)]
// union NVME_CDW11_GET_LOG_PAGE {
//     bits: u32,
//     fields: NVME_CDW11_GET_LOG_PAGE_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_GET_LOG_PAGE {
    pub NUMDU: u16,                 // Number of Upper Dwords (NUMDU)
    pub LogSpecificIdentifier: u16, // Log Specific Identifier
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW12_GET_LOG_PAGE {
    pub LPOL: u32, // Log Page Offset Lower (LPOL)
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW13_GET_LOG_PAGE {
    pub LPOU: u32, // Log Page Offset Upper (LPOU)
}

// #[derive(Clone, Copy)]
// union NVME_CDW14_GET_LOG_PAGE {
//     bits: u32,
//     fields: NVME_CDW14_GET_LOG_PAGE_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW14_GET_LOG_PAGE {
    pub UUIDIndex: B7, // UUID Index
    Reserved: B17,
    pub CommandSetIdentifier: B8, // Command Set Identifier
}

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_PARAMETER_ERROR_LOCATION {
    pub Byte: B8, // Byte in command that contained the error.
    pub Bit: B3,  // Bit in command that contained the error.
    pub Reserved: B5,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_ERROR_INFO_LOG {
    pub ErrorCount: u64,
    pub SQID: u16,                   // Submission Queue ID
    pub CMDID: u16,                  // Command ID
    pub Status: NVME_COMMAND_STATUS, // Status Field: This field indicates the Status Field for the command  that completed.  The Status Field is located in bits 15:01, bit 00 corresponds to the Phase Tag posted for the command.
    pub ParameterErrorLocation: NVME_PARAMETER_ERROR_LOCATION,
    pub Lba: u64, // LBA: This field indicates the first LBA that experienced the error condition, if applicable.
    pub NameSpace: u32, // Namespace: This field indicates the namespace that the error is associated with, if applicable.
    pub VendorInfoAvailable: u8, // Vendor Specific Information Available
    pub Reserved0: [u8; 3],
    pub CommandSpecificInfo: u64, // This field contains command specific information. If used, the command definition specifies the information returned.
    pub Reserved1: [u8; 24],
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_HEALTH_INFO_LOG_CRITICAL_WARNING {
    pub AvailableSpaceLow: B1,                // Available Space Low
    pub TemperatureThreshold: B1,             // Temperature Threshold
    pub ReliabilityDegraded: B1,              // Reliability Degraded
    pub ReadOnly: B1,                         // Read Only
    pub VolatileMemoryBackupDeviceFailed: B1, // Volatile Memory Backup Device Failed
    pub Reserved: B3,                         // Reserved
}

#[derive(Debug, Clone, Copy)]
pub struct NVME_HEALTH_INFO_LOG {
    pub CriticalWarning: NVME_HEALTH_INFO_LOG_CRITICAL_WARNING, // Critical Warning
    pub Temperature: u16,                                       // Temperature
    pub AvailableSpare: u8,                                     // Available Spare
    pub AvailableSpareThreshold: u8,                            // Available Spare Threshold
    pub PercentageUsed: u8,                                     // Percentage Used
    pub Reserved0: [u8; 26],
    pub DataUnitRead: [u8; 16],                // Data Units Read
    pub DataUnitWritten: [u8; 16],             // Data Units Written
    pub HostReadCommands: [u8; 16],            // Host Read Commands
    pub HostWrittenCommands: [u8; 16],         // Host Write Commands
    pub ControllerBusyTime: [u8; 16],          // Controller Busy Time
    pub PowerCycle: [u8; 16],                  // Power Cycles
    pub PowerOnHours: [u8; 16],                // Power On Hours
    pub UnsafeShutdowns: [u8; 16],             // Unsafe Shutdowns
    pub MediaErrors: [u8; 16],                 // Media Errors
    pub ErrorInfoLogEntryCount: [u8; 16],      // Number of Error Information Log Entries
    pub WarningCompositeTemperatureTime: u32,  // Warning Composite Temperature Time
    pub CriticalCompositeTemperatureTime: u32, // Critical Composite Temperature Time
    pub TemperatureSensor1: u16,               // Temperature Sensor 1
    pub TemperatureSensor2: u16,               // Temperature Sensor 2
    pub TemperatureSensor3: u16,               // Temperature Sensor 3
    pub TemperatureSensor4: u16,               // Temperature Sensor 4
    pub TemperatureSensor5: u16,               // Temperature Sensor 5
    pub TemperatureSensor6: u16,               // Temperature Sensor 6
    pub TemperatureSensor7: u16,               // Temperature Sensor 7
    pub TemperatureSensor8: u16,               // Temperature Sensor 8
    pub Reserved1: [u8; 296],
}

//
// "Telemetry Host-Initiated Log" structure definition.
//
const NVME_TELEMETRY_DATA_BLOCK_SIZE: usize = 0x200; // All NVMe Telemetry Data Blocks are 512 bytes in size.

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_TELEMETRY_HOST_INITIATED_LOG {
    pub LogIdentifier: u8,                           // Byte 0
    pub Reserved0: [u8; 4],                          // Bytes 1-4
    pub OrganizationID: [u8; 3],                     // Bytes 5-7 - IEEE OUI Identifier
    pub Area1LastBlock: u16,                         // Bytes 8-9
    pub Area2LastBlock: u16,                         // Bytes 10-11
    pub Area3LastBlock: u16,                         // Bytes 12-13
    pub Reserved1: [u8; 2],                          // Bytes 14-15
    pub Area4LastBlock: u32,                         // Bytes 16-19
    pub Reserved2: [u8; 361],                        // Bytes 20-380
    pub HostInitiatedDataGenerationNumber: u8,       // Byte 381
    pub ControllerInitiatedDataAvailable: u8,        // Byte 382
    pub ControllerInitiatedDataGenerationNumber: u8, // Byte 383
    pub ReasonIdentifier: [u8; 128],                 // Bytes 384-511
}

impl Default for NVME_TELEMETRY_HOST_INITIATED_LOG {
    fn default() -> Self {
        NVME_TELEMETRY_HOST_INITIATED_LOG {
            LogIdentifier: 0,
            Reserved0: [0; 4],
            OrganizationID: [0; 3],
            Area1LastBlock: 0,
            Area2LastBlock: 0,
            Area3LastBlock: 0,
            Reserved1: [0; 2],
            Area4LastBlock: 0,
            Reserved2: [0; 361],
            HostInitiatedDataGenerationNumber: 0,
            ControllerInitiatedDataAvailable: 0,
            ControllerInitiatedDataGenerationNumber: 0,
            ReasonIdentifier: [0; 128],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_TELEMETRY_CONTROLLER_INITIATED_LOG {
    pub LogIdentifier: u8,                           // Byte 0
    pub Reserved0: [u8; 4],                          // Bytes 1-4
    pub OrganizationID: [u8; 3],                     // Bytes 5-7 - IEEE OUI Identifier
    pub Area1LastBlock: u16,                         // Bytes 8-9
    pub Area2LastBlock: u16,                         // Bytes 10-11
    pub Area3LastBlock: u16,                         // Bytes 12-13
    pub Reserved1: [u8; 2],                          // Bytes 14-15
    pub Area4LastBlock: u32,                         // Bytes 16-19
    pub Reserved2: [u8; 362],                        // Bytes 20-381
    pub ControllerInitiatedDataAvailable: u8,        // Byte 382
    pub ControllerInitiatedDataGenerationNumber: u8, // Byte 383
    pub ReasonIdentifier: [u8; 128],                 // Bytes 384-511
}

impl Default for NVME_TELEMETRY_CONTROLLER_INITIATED_LOG {
    fn default() -> Self {
        NVME_TELEMETRY_CONTROLLER_INITIATED_LOG {
            LogIdentifier: 0,
            Reserved0: [0; 4],
            OrganizationID: [0; 3],
            Area1LastBlock: 0,
            Area2LastBlock: 0,
            Area3LastBlock: 0,
            Reserved1: [0; 2],
            Area4LastBlock: 0,
            Reserved2: [0; 362],
            ControllerInitiatedDataAvailable: 0,
            ControllerInitiatedDataGenerationNumber: 0,
            ReasonIdentifier: [0; 128],
        }
    }
}

//
// Information of log: NVME_LOG_PAGE_FIRMWARE_SLOT_INFO. Size: 512 bytes
//
#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_FIRMWARE_SLOT_INFO_LOG_AFI {
    pub ActiveSlot: B3, // Bits 2:0 indicates the firmware slot that contains the actively running firmware revision.
    Reserved0: B1,
    pub PendingActivateSlot: B3, // Bits 6:4 indicates the firmware slot that is going to be activated at the next controller reset.
    Reserved1: B1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_FIRMWARE_SLOT_INFO_LOG {
    pub AFI: NVME_FIRMWARE_SLOT_INFO_LOG_AFI, // Active Firmware Info (AFI)
    Reserved0: [u8; 7],
    pub FRS: [u64; 7], // Firmware Revision for Slot 1 - 7(FRS1 - FRS7):  Contains the revision of the firmware downloaded to firmware slot 1 - 7.
    Reserved1: [u8; 448],
}

impl Default for NVME_FIRMWARE_SLOT_INFO_LOG {
    fn default() -> Self {
        NVME_FIRMWARE_SLOT_INFO_LOG {
            AFI: NVME_FIRMWARE_SLOT_INFO_LOG_AFI::default(),
            Reserved0: [0; 7],
            FRS: [0; 7],
            Reserved1: [0; 448],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_CHANGED_NAMESPACE_LIST_LOG {
    pub NSID: [u32; 1024], // List of Namespace ID up to 1024 entries
}

impl Default for NVME_CHANGED_NAMESPACE_LIST_LOG {
    fn default() -> Self {
        NVME_CHANGED_NAMESPACE_LIST_LOG { NSID: [0; 1024] }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_CHANGED_ZONE_LIST_LOG {
    pub ZoneIdentifiersCount: u16, // Number of Zone Identifiers
    Reserved: [u8; 6],
    pub ZoneIdentifier: [u64; 511], // List of Zone Identifiers up to 511 entries. Identifier contains Zone Start Logical Block Address(ZSLBA)
}

impl Default for NVME_CHANGED_ZONE_LIST_LOG {
    fn default() -> Self {
        NVME_CHANGED_ZONE_LIST_LOG {
            ZoneIdentifiersCount: 0,
            Reserved: [0; 6],
            ZoneIdentifier: [0; 511],
        }
    }
}

//
// Information of log: NVME_LOG_PAGE_COMMAND_EFFECTS. Size: 4096 bytes
#[derive(Debug, Clone, Copy)]
pub enum NVME_COMMAND_EFFECT_SUBMISSION_EXECUTION_LIMITS {
    NVME_COMMAND_EFFECT_SUBMISSION_EXECUTION_LIMIT_NONE = 0,
    NVME_COMMAND_EFFECT_SUBMISSION_EXECUTION_LIMIT_SINGLE_PER_NAMESPACE = 1,
    NVME_COMMAND_EFFECT_SUBMISSION_EXECUTION_LIMIT_SINGLE_PER_CONTROLLER = 2,
}

// #[derive(Clone, Copy)]
// union NVME_COMMAND_EFFECTS_DATA {
//     bits: u32,
//     fields: NVME_COMMAND_EFFECTS_DATA_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_EFFECTS_DATA {
    pub CSUPP: B1,      // Command Supported (CSUPP)
    pub LBCC: B1,       // Logical Block Content Change (LBCC)
    pub NCC: B1,        // Namespace Capability Change (NCC)
    pub NIC: B1,        // Namespace Inventory Change (NIC)
    pub CCC: B1,        // Controller Capability Change (CCC)
    pub Reserved0: B11, // Reserved
    pub CSE: B3,        // Command Submission and Execution (CSE)
    pub Reserved1: B13, // Reserved
}

#[derive(Clone, Copy)]
pub struct NVME_COMMAND_EFFECTS_LOG {
    pub ACS: [NVME_COMMAND_EFFECTS_DATA; 256], // Admin Command Supported
    pub IOCS: [NVME_COMMAND_EFFECTS_DATA; 256], // I/O Command Supported
    Reserved: [u8; 2048],
}
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA {
    pub Status: NVME_DEVICE_SELF_TEST_RESULT_DATA_Status,
    pub SegmentNumber: u8,
    pub ValidDiagnostics: NVME_DEVICE_SELF_TEST_RESULT_DATA_ValidDiagnostics,
    pub Reserved: u8,
    pub POH: u64,
    pub NSID: u32,
    pub FailingLBA: u64,
    pub StatusCodeType: NVME_DEVICE_SELF_TEST_RESULT_DATA_StatusCodeType,
    pub StatusCode: u8,
    pub VendorSpecific: u16,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA_Status {
    pub Result: B4,
    pub CodeValue: B4,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA_ValidDiagnostics {
    pub NSIDValid: B1,
    pub FLBAValid: B1,
    pub SCTValid: B1,
    pub SCValid: B1,
    pub Reserved: B4,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA_StatusCodeType {
    pub AdditionalInfo: B3,
    Reserved: B5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_LOG {
    pub CurrentOperation: NVME_DEVICE_SELF_TEST_LOG_CurrentOperation,
    pub CurrentCompletion: NVME_DEVICE_SELF_TEST_LOG_CurrentCompletion,
    pub Reserved: [u8; 2],
    pub ResultData: [NVME_DEVICE_SELF_TEST_RESULT_DATA; 20],
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_LOG_CurrentOperation {
    pub Status: B4,
    Reserved: B4,
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_LOG_CurrentCompletion {
    pub CompletePercent: B7,
    Reserved: B1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_ENDURANCE_GROUP_LOG {
    pub Reserved0: u32,
    pub AvailableSpareThreshold: u8,
    pub PercentageUsed: u8,
    pub Reserved1: [u8; 26],
    pub EnduranceEstimate: [u8; 16],
    pub DataUnitsRead: [u8; 16],
    pub DataUnitsWritten: [u8; 16],
    pub MediaUnitsWritten: [u8; 16],
    pub Reserved2: [u8; 416],
}
impl Default for NVME_ENDURANCE_GROUP_LOG {
    fn default() -> Self {
        NVME_ENDURANCE_GROUP_LOG {
            Reserved0: 0,
            AvailableSpareThreshold: 0,
            PercentageUsed: 0,
            Reserved1: [0; 26],
            EnduranceEstimate: [0; 16],
            DataUnitsRead: [0; 16],
            DataUnitsWritten: [0; 16],
            MediaUnitsWritten: [0; 16],
            Reserved2: [0; 416],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_PERSISTENT_EVENT_LOG_HEADER {
    pub LogIdentifier: u8,
    pub Reserved0: [u8; 3],
    pub TotalNumberOfEvents: u32,
    pub TotalLogLength: u64,
    pub LogRevision: u8,
    pub Reserved1: u8,
    pub LogHeaderLength: u16,
    pub Timestamp: u64,
    pub PowerOnHours: [u8; 16],
    pub PowerCycleCount: u64,
    pub PciVendorId: u16,
    pub PciSubsystemVendorId: u16,
    pub SerialNumber: [u8; 20],
    pub ModelNumber: [u8; 40],
    pub NVMSubsystemNVMeQualifiedName: [u8; 256],
    pub Reserved: [u8; 108],
    pub SupportedEventsBitmap: [u8; 32],
}

impl Default for NVME_PERSISTENT_EVENT_LOG_HEADER {
    fn default() -> Self {
        NVME_PERSISTENT_EVENT_LOG_HEADER {
            LogIdentifier: 0,
            Reserved0: [0; 3],
            TotalNumberOfEvents: 0,
            TotalLogLength: 0,
            LogRevision: 0,
            Reserved1: 0,
            LogHeaderLength: 0,
            Timestamp: 0,
            PowerOnHours: [0; 16],
            PowerCycleCount: 0,
            PciVendorId: 0,
            PciSubsystemVendorId: 0,
            SerialNumber: [0; 20],
            ModelNumber: [0; 40],
            NVMSubsystemNVMeQualifiedName: [0; 256],
            Reserved: [0; 108],
            SupportedEventsBitmap: [0; 32],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_PERSISTENT_EVENT_LOG_EVENT_HEADER {
    pub EventType: u8,
    pub EventTypeRevision: u8,
    pub EventHeaderLength: u8,
    pub Reserved0: u8,
    pub ControllerIdentifier: u16,
    pub EventTimestamp: u64,
    pub Reserved1: [u8; 6],
    pub VendorSpecificInformationLength: u16,
    pub EventLength: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES {
    NVME_PERSISTENT_EVENT_TYPE_RESERVED0 = 0x00,
    NVME_PERSISTENT_EVENT_TYPE_SMART_HEALTH_LOG_SNAPSHOT = 0x01,
    NVME_PERSISTENT_EVENT_TYPE_FIRMWARE_COMMIT = 0x02,
    NVME_PERSISTENT_EVENT_TYPE_TIMESTAMP_CHANGE = 0x03,
    NVME_PERSISTENT_EVENT_TYPE_POWER_ON_OR_RESET = 0x04,
    NVME_PERSISTENT_EVENT_TYPE_NVM_SUBSYSTEM_HARDWARE_ERROR = 0x05,
    NVME_PERSISTENT_EVENT_TYPE_CHANGE_NAMESPACE = 0x06,
    NVME_PERSISTENT_EVENT_TYPE_FORMAT_NVM_START = 0x07,
    NVME_PERSISTENT_EVENT_TYPE_FORMAT_NVM_COMPLETION = 0x08,
    NVME_PERSISTENT_EVENT_TYPE_SANITIZE_START = 0x09,
    NVME_PERSISTENT_EVENT_TYPE_SANITIZE_COMPLETION = 0x0A,
    NVME_PERSISTENT_EVENT_TYPE_SET_FEATURE = 0x0B,
    NVME_PERSISTENT_EVENT_TYPE_TELEMETRY_LOG_CREATED = 0x0C,
    NVME_PERSISTENT_EVENT_TYPE_THERMAL_EXCURSION = 0x0D,
    NVME_PERSISTENT_EVENT_TYPE_RESERVED1_BEGIN = 0x0E,
    NVME_PERSISTENT_EVENT_TYPE_RESERVED1_END = 0xDD,
    NVME_PERSISTENT_EVENT_TYPE_VENDOR_SPECIFIC_EVENT = 0xDE,
    NVME_PERSISTENT_EVENT_TYPE_TCG_DEFINED = 0xDF,
    NVME_PERSISTENT_EVENT_TYPE_RESERVED2_BEGIN = 0xE0,
    NVME_PERSISTENT_EVENT_TYPE_MAX = 0xFF,
}

//
// Information of log: NVME_LOG_PAGE_RESERVATION_NOTIFICATION. Size: 64 bytes
//
#[derive(Debug, Clone, Copy)]
pub enum NVME_RESERVATION_NOTIFICATION_TYPES {
    NVME_RESERVATION_NOTIFICATION_TYPE_EMPTY_LOG_PAGE = 0,
    NVME_RESERVATION_NOTIFICATION_TYPE_REGISTRATION_PREEMPTED = 1,
    NVME_RESERVATION_NOTIFICATION_TYPE_REGISTRATION_RELEASED = 2,
    NVME_RESERVATION_NOTIFICATION_TYPE_RESERVATION_PREEMPTED = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_RESERVATION_NOTIFICATION_LOG {
    pub LogPageCount: u64,         // Log Page Count
    pub LogPageType: u8,           // Reservation Notification Log Page Type.
    pub AvailableLogPageCount: u8, // Number of Available Log Pages
    pub Reserved0: [u8; 2],
    pub NameSpaceId: u32, // Namespace ID
    pub Reserved1: [u8; 48],
}
impl Default for NVME_RESERVATION_NOTIFICATION_LOG {
    fn default() -> Self {
        NVME_RESERVATION_NOTIFICATION_LOG {
            LogPageCount: 0,
            LogPageType: 0,
            AvailableLogPageCount: 0,
            Reserved0: [0; 2],
            NameSpaceId: 0,
            Reserved1: [0; 48],
        }
    }
}

//
// Information of log: NVME_SANITIZE_STATUS_LOG. Size: 512 bytes
//
#[derive(Debug, Clone, Copy)]
pub enum NVME_SANITIZE_OPERATION_STATUS {
    NVME_SANITIZE_OPERATION_NONE = 0,
    NVME_SANITIZE_OPERATION_SUCCEEDED = 1,
    NVME_SANITIZE_OPERATION_IN_PROGRESS = 2,
    NVME_SANITIZE_OPERATION_FAILED = 3,
    NVME_SANITIZE_OPERATION_SUCCEEDED_WITH_FORCED_DEALLOCATION = 4,
}

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_SANITIZE_STATUS {
    pub MostRecentSanitizeOperationStatus: B3,
    pub NumberCompletedPassesOfOverwrite: B4,
    pub GlobalDataErased: B1, // Changed from bool to u8 to satisfy repr(C)
    pub Reserved: B8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_SANITIZE_STATUS_LOG {
    pub SPROG: u16,
    pub SSTAT: NVME_SANITIZE_STATUS,
    pub SCDW10: u32,
    pub EstimatedTimeForOverwrite: u32,
    pub EstimatedTimeForBlockErase: u32,
    pub EstimatedTimeForCryptoErase: u32,
    pub EstimatedTimeForOverwriteWithNoDeallocateMediaModification: u32,
    pub EstimatedTimeForBlockEraseWithNoDeallocateMediaModification: u32,
    pub EstimatedTimeForCryptoEraseWithNoDeallocateMediaModification: u32,
    pub Reserved: [u8; 480],
}

impl Default for NVME_SANITIZE_STATUS_LOG {
    fn default() -> Self {
        NVME_SANITIZE_STATUS_LOG {
            SPROG: 0,
            SSTAT: NVME_SANITIZE_STATUS::default(),
            SCDW10: 0,
            EstimatedTimeForOverwrite: 0,
            EstimatedTimeForBlockErase: 0,
            EstimatedTimeForCryptoErase: 0,
            EstimatedTimeForOverwriteWithNoDeallocateMediaModification: 0,
            EstimatedTimeForBlockEraseWithNoDeallocateMediaModification: 0,
            EstimatedTimeForCryptoEraseWithNoDeallocateMediaModification: 0,
            Reserved: [0; 480],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_FIRMWARE_DOWNLOAD {
    pub NUMD: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_FIRMWARE_DOWNLOAD {
    pub OFST: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_FIRMWARE_ACTIVATE_ACTIONS {
    NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT = 0,
    NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT_AND_ACTIVATE = 1,
    NVME_FIRMWARE_ACTIVATE_ACTION_ACTIVATE = 2,
    NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT_AND_ACTIVATE_IMMEDIATE = 3,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_FIRMWARE_ACTIVATE {
    FS: B3,
    AA: B2,
    Reserved: B27,
}

//
// Parameters for FORMAT NVM Commands
//
#[derive(Debug, Clone, Copy)]
pub enum NVME_PROTECTION_INFORMATION_TYPES {
    NVME_PROTECTION_INFORMATION_NOT_ENABLED = 0,
    NVME_PROTECTION_INFORMATION_TYPE1 = 1,
    NVME_PROTECTION_INFORMATION_TYPE2 = 2,
    NVME_PROTECTION_INFORMATION_TYPE3 = 3,
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_SECURE_ERASE_SETTINGS {
    NVME_SECURE_ERASE_NONE = 0,
    NVME_SECURE_ERASE_USER_DATA = 1,
    NVME_SECURE_ERASE_CRYPTOGRAPHIC = 2,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_FORMAT_NVM {
    pub LBAF: B4, // LBA Format (LBAF)
    pub MS: B1,   // Metadata Settings (MS)
    pub PI: B3,   // Protection Information (PI)
    pub PIL: B1,  // Protection Information Location (PIL)
    pub SES: B3,  // Secure Erase Settings (SES)
    pub ZF: B2,   // Zone Format (ZF)
    pub Reserved: B18,
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE {
    NVME_MEDIA_ADDITIONALLY_MODIFIED_AFTER_SANITIZE_NOT_DEFINED = 0,
    NVME_MEDIA_NOT_ADDITIONALLY_MODIFIED_AFTER_SANITIZE = 1,
    NVME_MEDIA_ADDITIONALLY_MODIFIED_AFTER_SANITIZE = 2,
}

//
// Parameters for Sanitize.
//

#[derive(Debug, Clone, Copy)]
pub enum NVME_SANITIZE_ACTION {
    NVME_SANITIZE_ACTION_RESERVED = 0,
    NVME_SANITIZE_ACTION_EXIT_FAILURE_MODE = 1,
    NVME_SANITIZE_ACTION_START_BLOCK_ERASE_SANITIZE = 2,
    NVME_SANITIZE_ACTION_START_OVERWRITE_SANITIZE = 3,
    NVME_SANITIZE_ACTION_START_CRYPTO_ERASE_SANITIZE = 4,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_SANITIZE {
    pub SANACT: B3, // Sanitize Action (SANACT)
    pub AUSE: B1,   // Allow Unrestricted Sanitize Exit (AUSE)
    pub OWPASS: B4, // Overwrite Pass Count (OWPASS)
    pub OIPBP: B1,  // Overwrite Invert Pattern Between Passes (OIPBP)
    pub NDAS: B1,   // No Deallocate After Sanitize
    pub Reserved: B22,
}

// #[derive(Clone, Copy)]
// union NVME_CDW10_SANITIZE {
//     DUMMYSTRUCTNAME: NVME_CDW10_SANITIZE,
//     AsUlong: u32,
// }

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_SANITIZE {
    pub OVRPAT: u32, // Overwrite Pattern
}

//
// Parameters for RESERVATION Commands
//
#[derive(Debug, Clone, Copy)]
pub enum NVME_RESERVATION_TYPES {
    NVME_RESERVATION_TYPE_RESERVED = 0,
    NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE = 1,
    NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS = 2,
    NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE_REGISTRANTS_ONLY = 3,
    NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS_REGISTRANTS_ONLY = 4,
    NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE_ALL_REGISTRANTS = 5,
    NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS_ALL_REGISTRANTS = 6,
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_RESERVATION_ACQUIRE_ACTIONS {
    NVME_RESERVATION_ACQUIRE_ACTION_ACQUIRE = 0,
    NVME_RESERVATION_ACQUIRE_ACTION_PREEMPT = 1,
    NVME_RESERVATION_ACQUIRE_ACTION_PREEMPT_AND_ABORT = 2,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW0_RESERVATION_PERSISTENCE {
    pub PTPL: B1, // Persist Through Power Loss (PTPL)
    Reserved: B31,
}

// #[derive(Clone, Copy)]
// union NVME_CDW10_RESERVATION_ACQUIRE {
//     bits: u32,
//     fields: NVME_CDW10_RESERVATION_ACQUIRE_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_RESERVATION_ACQUIRE {
    pub RACQA: B3, // Reservation Acquire Action (RACQA)
    pub IEKEY: B1, // Ignore Existing Key (IEKEY)
    Reserved: B4,
    pub RTYPE: B8, // Reservation Type (RTYPE)
    Reserved1: B16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_RESERVATION_ACQUIRE_DATA_STRUCTURE {
    pub CRKEY: u64, // Current Reservation Key (CRKEY)
    pub PRKEY: u64, // Preempt Reservation Key (PRKEY)
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_RESERVATION_REGISTER_ACTIONS {
    NVME_RESERVATION_REGISTER_ACTION_REGISTER = 0,
    NVME_RESERVATION_REGISTER_ACTION_UNREGISTER = 1,
    NVME_RESERVATION_REGISTER_ACTION_REPLACE = 2,
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES {
    NVME_RESERVATION_REGISTER_PTPL_STATE_NO_CHANGE = 0,
    NVME_RESERVATION_REGISTER_PTPL_STATE_RESERVED = 1,
    NVME_RESERVATION_REGISTER_PTPL_STATE_SET_TO_0 = 2, // Reservations are released and registrants are cleared on a power on.
    NVME_RESERVATION_REGISTER_PTPL_STATE_SET_TO_1 = 3, // Reservations and registrants persist across a power loss.
}

// #[derive(Clone, Copy)]
// union NVME_CDW10_RESERVATION_REGISTER {
//     bits: u32,
//     fields: NVME_CDW10_RESERVATION_REGISTER_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_RESERVATION_REGISTER {
    pub RREGA: B3, // Reservation Register Action (RREGA)
    pub IEKEY: B1, // Ignore Existing Key (IEKEY)
    Reserved: B26,
    pub CPTPL: B2, // Change Persist Through Power Loss State (CPTPL)
}

//
// Reservation Register Data Structure
//
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_RESERVATION_REGISTER_DATA_STRUCTURE {
    pub CRKEY: u64, // Current Reservation Key (CRKEY)
    pub NRKEY: u64, // New Reservation Key (NRKEY)
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_RESERVATION_RELEASE_ACTIONS {
    NVME_RESERVATION_RELEASE_ACTION_RELEASE = 0,
    NVME_RESERVATION_RELEASE_ACTION_CLEAR = 1,
}

// #[derive(Clone, Copy)]
// union NVME_CDW10_RESERVATION_RELEASE {
//     bits: u32,
//     fields: NVME_CDW10_RESERVATION_RELEASE_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_RESERVATION_RELEASE {
    pub RRELA: B3, // Reservation Release Action (RRELA)
    pub IEKEY: B1, // IgnoreExistingKey (IEKEY)
    pub Reserved: B4,
    pub RTYPE: B8, // Reservation Type (RTYPE)
    pub Reserved1: B16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_RESERVATION_RELEASE_DATA_STRUCTURE {
    pub CRKEY: u64, // Current Reservation Key (CRKEY)
}

// #[derive(Clone, Copy)]
// union NVME_CDW10_RESERVATION_REPORT {
//     bits: u32,
//     fields: NVME_CDW10_RESERVATION_REPORT_FIELDS,
// }

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_RESERVATION_REPORT {
    pub NUMD: u32, // Number of Dwords (NUMD), NOTE: 0's based value.
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_RESERVATION_REPORT {
    pub EDS: B1, // Extended Data Structure (EDS)
    Reserved: B31,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_RESERVATION_REPORT_STATUS_HEADER {
    pub GEN: u32,    // Generation (Gen)
    pub RTYPE: u8,   // Reservation Type (RTYPE)
    pub REGCTL: u16, // Number of Registered Controllers (REGCTL)
    pub Reserved: [u8; 2],
    pub PTPLS: u8, // Persist Through Power Loss State (PTPLS)
    pub Reserved1: [u8; 14],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_REGISTERED_CONTROLLER_DATA {
    pub CNTLID: u16,                                  // Controller ID (CNTLID)
    pub RCSTS: NVME_REGISTERED_CONTROLLER_DATA_RCSTS, // Reservation Status (RCSTS)
    Reserved: [u8; 5],
    pub HOSTID: [u8; 8], // Host Identifier (HOSTID)
    pub RKEY: u64,       // Reservation Key (RKEY)
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_REGISTERED_CONTROLLER_DATA_RCSTS {
    pub HoldReservation: B1,
    Reserved: B7,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE {
    pub Header: NVME_RESERVATION_REPORT_STATUS_HEADER,
    pub RegisteredControllersData: [NVME_REGISTERED_CONTROLLER_DATA; 0], // ANYSIZE_ARRAY equivalent
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_REGISTERED_CONTROLLER_EXTENDED_DATA {
    pub CNTLID: u16,                                           // Controller ID (CNTLID)
    pub RCSTS: NVME_REGISTERED_CONTROLLER_EXTENDED_DATA_RCSTS, // Reservation Status (RCSTS)
    pub Reserved: [u8; 5],
    pub RKEY: u64,        // Reservation Key (RKEY)
    pub HOSTID: [u8; 16], // 128-bit Host Identifier (HOSTID)
    pub Reserved1: [u8; 32],
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_REGISTERED_CONTROLLER_EXTENDED_DATA_RCSTS {
    pub HoldReservation: B1,
    pub Reserved: B7,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE {
    pub Header: NVME_RESERVATION_REPORT_STATUS_HEADER,
    pub Reserved1: [u8; 40],
    pub RegisteredControllersExtendedData: [NVME_REGISTERED_CONTROLLER_EXTENDED_DATA; 0], // ANYSIZE_ARRAY equivalent
}

impl Default for NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE {
    fn default() -> Self {
        NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE {
            Header: NVME_RESERVATION_REPORT_STATUS_HEADER::default(),
            Reserved1: [0; 40],
            RegisteredControllersExtendedData: [NVME_REGISTERED_CONTROLLER_EXTENDED_DATA::default();
                0],
        }
    }
}

//
// Parameters for Directives.
//
#[derive(Debug, Clone, Copy)]
pub enum NVME_DIRECTIVE_TYPES {
    NVME_DIRECTIVE_TYPE_IDENTIFY = 0x00,
    NVME_DIRECTIVE_TYPE_STREAMS = 0x01,
}

const NVME_STREAMS_ID_MIN: u16 = 1;
const NVME_STREAMS_ID_MAX: u16 = 0xFFFF;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_DIRECTIVE_RECEIVE {
    pub NUMD: u32, // Number of Dwords (NUMD)
}

// #[derive(Clone, Copy)]
// union NVME_CDW11_DIRECTIVE_RECEIVE {
//     bits: u32,
//     fields: NVME_CDW11_DIRECTIVE_RECEIVE_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_DIRECTIVE_RECEIVE {
    pub DOPER: B8,  // Directive Operation
    pub DTYPE: B8,  // Directive Type
    pub DSPEC: B16, // Directive Specific
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_DIRECTIVE_SEND {
    pub NUMD: u32, // Number of Dwords (NUMD)
}

// #[derive(Clone, Copy)]
// union NVME_CDW11_DIRECTIVE_SEND {
//     bits: u32,
//     fields: NVME_CDW11_DIRECTIVE_SEND_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_DIRECTIVE_SEND {
    pub DOPER: B8,  // Directive Operation
    pub DTYPE: B8,  // Directive Type
    pub DSPEC: B16, // Directive Specific
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_DIRECTIVE_RECEIVE_IDENTIFY_OPERATIONS {
    NVME_DIRECTIVE_RECEIVE_IDENTIFY_OPERATION_RETURN_PARAMETERS = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_DIRECTIVE_SEND_IDENTIFY_OPERATIONS {
    NVME_DIRECTIVE_SEND_IDENTIFY_OPERATION_ENABLE_DIRECTIVE = 1,
}

#[bitfield]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR {
    pub Identify: B1,
    pub Streams: B1,
    Reserved0: B6,
    Reserved1: B120,
    Reserved2: B128,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS {
    pub DirectivesSupported: NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR,
    pub DirectivesEnabled: NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR,
    // Reserved: [u8; 4032], // Uncomment if needed
}

// #[derive(Clone, Copy)]
// union NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE {
//     bits: u32,
//     fields: NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE {
    pub ENDIR: B1, // Enable Directive
    Reserved0: B7,
    pub DTYPE: B8, // Directive Type
    Reserved1: B16,
}

//
// Parameters for the Streams Directive Type
//
#[derive(Debug, Clone, Copy)]
pub enum NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATIONS {
    NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_RETURN_PARAMETERS = 1,
    NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_GET_STATUS = 2,
    NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_ALLOCATE_RESOURCES = 3,
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_DIRECTIVE_SEND_STREAMS_OPERATIONS {
    NVME_DIRECTIVE_SEND_STREAMS_OPERATION_RELEASE_IDENTIFIER = 1,
    NVME_DIRECTIVE_SEND_STREAMS_OPERATION_RELEASE_RESOURCES = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS {
    pub MSL: u16,  // Max Streams Limit
    pub NSSA: u16, // NVM Subsystem Streams Available
    pub NSSO: u16, // NVM Subsystem Streams Open
    pub Reserved0: [u8; 10],
    pub SWS: u32, // Stream Write Size
    pub SGS: u16, // Stream Granularity Size
    pub NSA: u16, // Namespace Streams Allocated
    pub NSO: u16, // Namespace Streams Open
    pub Reserved1: [u8; 6],
}

const NVME_STREAMS_GET_STATUS_MAX_IDS: usize = 65535;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA {
    pub OpenStreamCount: u16, // Number of currently open streams.
    pub StreamIdentifiers: [u16; NVME_STREAMS_GET_STATUS_MAX_IDS], // Array of stream IDs that are currently open.
}

impl Default for NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA {
    fn default() -> Self {
        NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA {
            OpenStreamCount: 0,
            StreamIdentifiers: [0; NVME_STREAMS_GET_STATUS_MAX_IDS],
        }
    }
}

// #[derive(Clone, Copy)]
// union NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
//     bits: u32,
//     fields: NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_FIELDS,
// }

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    pub NSR: u16, // Namespace Streams Requested
    Reserved: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    pub NSA: u16, // Namespace Streams Allocated
    Reserved: u16,
}

#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_SEND {
    pub EnableDirective: NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE,
    pub AsUlong: u32,
}

#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_RECEIVE {
    pub AllocateResources: NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES,
    pub AsUlong: u32,
}

//
// Parameters for SECURITY SEND / RECEIVE Commands
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_SECURITY_SEND_RECEIVE {
    Reserved0: B8, // Reserved0
    pub SPSP: B16, // SP Specific (SPSP)
    pub SECP: B8,  // Security Protocol (SECP)
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_SECURITY_SEND {
    pub TL: u32, // Transfer Length (TL)
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_SECURITY_RECEIVE {
    pub AL: u32, // Transfer Length (AL)
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NVME_NVM_COMMANDS {
    NVME_NVM_COMMAND_FLUSH = 0x00,
    NVME_NVM_COMMAND_WRITE = 0x01,
    NVME_NVM_COMMAND_READ = 0x02,
    NVME_NVM_COMMAND_WRITE_UNCORRECTABLE = 0x04,
    NVME_NVM_COMMAND_COMPARE = 0x05,
    NVME_NVM_COMMAND_WRITE_ZEROES = 0x08,
    NVME_NVM_COMMAND_DATASET_MANAGEMENT = 0x09,
    NVME_NVM_COMMAND_VERIFY = 0x0C,
    NVME_NVM_COMMAND_RESERVATION_REGISTER = 0x0D,
    NVME_NVM_COMMAND_RESERVATION_REPORT = 0x0E,
    NVME_NVM_COMMAND_RESERVATION_ACQUIRE = 0x11,
    NVME_NVM_COMMAND_RESERVATION_RELEASE = 0x15,
    NVME_NVM_COMMAND_COPY = 0x19,
    NVME_NVM_COMMAND_ZONE_MANAGEMENT_SEND = 0x79,
    NVME_NVM_COMMAND_ZONE_MANAGEMENT_RECEIVE = 0x7A,
    NVME_NVM_COMMAND_ZONE_APPEND = 0x7D,
}

//
// Data structure of CDW12 for Read/Write command
//
#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW12_READ_WRITE {
    pub NLB: B16, // Number of Logical Blocks (NLB)
    pub Reserved0: B4,
    pub DTYPE: B4, // Directive Type (DTYPE)
    pub Reserved1: B2,
    pub PRINFO: B4, // Protection Information Field (PRINFO)
    pub FUA: B1,    // Force Unit Access (FUA)
    pub LR: B1,     // Limited Retry (LR)
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_ACCESS_FREQUENCIES {
    NVME_ACCESS_FREQUENCY_NONE = 0, // No frequency information provided.
    NVME_ACCESS_FREQUENCY_TYPICAL = 1, // Typical number of reads and writes expected for this LBA range.
    NVME_ACCESS_FREQUENCY_INFR_WRITE_INFR_READ = 2, // Infrequent writes and infrequent reads to the LBA range indicated.
    NVME_ACCESS_FREQUENCY_INFR_WRITE_FR_READ = 3, // Infrequent writes and frequent reads to the LBA range indicated.
    NVME_ACCESS_FREQUENCY_FR_WRITE_INFR_READ = 4, // Frequent writes and infrequent reads to the LBA range indicated.
    NVME_ACCESS_FREQUENCY_FR_WRITE_FR_READ = 5, // Frequent writes and frequent reads to the LBA range indicated.
    NVME_ACCESS_FREQUENCY_ONE_TIME_READ = 6, // One time read. E.g. command is due to virus scan, backup, file copy, or archive.
    NVME_ACCESS_FREQUENCY_SPECULATIVE_READ = 7, // Speculative read. The command is part of a prefetch operation.
    NVME_ACCESS_FREQUENCY_WILL_BE_OVERWRITTEN = 8, // The LBA range is going to be overwritten in the near future.
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_ACCESS_LATENCIES {
    NVME_ACCESS_LATENCY_NONE = 0, // None.  No latency information provided.
    NVME_ACCESS_LATENCY_IDLE = 1, // Idle. Longer latency acceptable
    NVME_ACCESS_LATENCY_NORMAL = 2, // Normal. Typical latency.
    NVME_ACCESS_LATENCY_LOW = 3,  // Low. Smallest possible latency
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW13_READ_WRITE_DSM {
    pub AccessFrequency: B4,
    pub AccessLatency: B2,
    pub SequentialRequest: B1,
    pub Incompressible: B1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW13_READ_WRITE {
    pub DSM: NVME_CDW13_READ_WRITE_DSM, // Dataset Management (DSM)
    Reserved: u8,
    pub DSPEC: u16, // Directive Specific Value
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW15_READ_WRITE {
    pub ELBAT: u16,  // Expected Logical Block Application Tag (ELBAT)
    pub ELBATM: u16, // Expected Logical Block Application Tag Mask (ELBATM)
}

//
// Dataset Management - Range Definition
//
// #[repr(C)]
// #[derive(Clone, Copy)]
// union NVME_CONTEXT_ATTRIBUTES {
//     bits: u32,
//     fields: NVME_CONTEXT_ATTRIBUTES_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CONTEXT_ATTRIBUTES {
    pub AccessFrequency: B4,      // 4 bits
    pub AccessLatency: B2,        // 2 bits
    pub Reserved0: B2,            // 2 bits
    pub SequentialReadRange: B1,  // 1 bit
    pub SequentialWriteRange: B1, // 1 bit
    pub WritePrepare: B1,         // 1 bit
    pub Reserved1: B13,           // 13 bits
    pub CommandAccessSize: B8,    // 8 bits
}

#[repr(C)]
pub struct NVME_LBA_RANGE {
    pub Attributes: NVME_CONTEXT_ATTRIBUTES, // The use of this information is optional and the controller is not required to perform any specific action.
    pub LogicalBlockCount: u32,
    pub StartingLBA: u64,
}

// #[repr(C)]
// #[derive(Clone, Copy)]
// union NVME_CDW10_DATASET_MANAGEMENT {
//     bits: u32,
//     fields: NVME_CDW10_DATASET_MANAGEMENT_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_DATASET_MANAGEMENT {
    pub NR: B8,    // Number of Ranges (NR)
    Reserved: B24, // 24 bits
}

// #[repr(C)]
// #[derive(Clone, Copy)]
// union NVME_CDW11_DATASET_MANAGEMENT {
//     bits: u32,
//     fields: NVME_CDW11_DATASET_MANAGEMENT_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW11_DATASET_MANAGEMENT {
    pub IDR: B1,   // Integral Dataset for Read (IDR)
    pub IDW: B1,   // Integral Dataset for Write (IDW)
    pub AD: B1,    // Deallocate (AD)
    Reserved: B29, // 29 bits
}

//
// Zone Descriptor
//
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_ZONE_DESCRIPTOR {
    pub ZT: u8,                  // Zone Type
    pub ZS: u8,                  // Zone State
    pub ZA: NVME_ZONE_ATTRIBUTE, // Zone Attribute
    pub Reserved3: [u8; 5],
    pub ZCAP: u64,         // Zone Capacity
    pub ZSLBA: u64,        // Zone Start Logical Block Address
    pub WritePointer: u64, // Current Write pointer of the Zone
    pub Reserved4: [u8; 32],
}

#[bitfield]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_ZONE_ATTRIBUTE {
    pub ZFC: B1, // Zone Finished by Controller (ZFC)
    pub FZR: B1, // Finish Zone Recommended (FZR)
    pub RZR: B1, // Reset Zone Recommended (RZR)
    pub Reserved: B4,
    pub ZDEV: B1, // Zone Descriptor Extension Valid (ZDEV)
}

#[derive(Debug, Clone, Copy)]
enum ZONE_STATE {
    NVME_STATE_ZSE = 0x1,  // Zone State Empty
    NVME_STATE_ZSIO = 0x2, // Zone State Implicitly Opened
    NVME_STATE_ZSEO = 0x3, // Zone State Explicitly Opened
    NVME_STATE_ZSC = 0x4,  // Zone State Closed
    NVME_STATE_ZSRO = 0xD, // Zone State Read-Only
    NVME_STATE_ZSF = 0xE,  // Zone State Full
    NVME_STATE_ZSO = 0xF,  // Zone State Offline
}

#[derive(Debug, Clone, Copy)]
pub enum NVME_ZONE_SEND_ACTION {
    NVME_ZONE_SEND_CLOSE = 1,                  // Close one or more zones
    NVME_ZONE_SEND_FINISH = 2,                 // Finish one or more zones
    NVME_ZONE_SEND_OPEN = 3,                   // Open one or more zones
    NVME_ZONE_SEND_RESET = 4,                  // Reset one or more zones
    NVME_ZONE_SEND_OFFLINE = 5,                // Offline one or more zones
    NVME_ZONE_SEND_SET_ZONE_DESCRIPTOR = 0x10, // Attach Zone Descriptor Extension data to a zone in the Empty state and transition the zone to the Closed state
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_ZONE_MANAGEMENT_SEND {
    pub SLBA: u64, // Starting LBA (SLBA)
}

// #[derive(Clone, Copy)]
// union NVME_CDW13_ZONE_MANAGEMENT_SEND {
//     bits: u32,
//     fields: NVME_CDW13_ZONE_MANAGEMENT_SEND_FIELDS,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW13_ZONE_MANAGEMENT_SEND {
    pub ZSA: B8,       // Zone Send Action, as defined in NVME_ZONE_SEND_ACTION
    pub SelectAll: B1, // Select all the zones. SLBA is ignored if set
    Reserved: B23,
}

//
// Report Zone Data Structure
//
#[repr(C)]
pub struct NVME_REPORT_ZONE_INFO {
    pub ZoneCount: u64, // Number of Zones
    Reserved: [u64; 7],
    pub ZoneDescriptor: [NVME_ZONE_DESCRIPTOR; 0], // ANYSIZE_ARRAY equivalent
}

#[repr(C)]
pub struct NVME_ZONE_DESCRIPTOR_EXTENSION {
    pub ZoneDescriptorExtensionInfo: [u8; 64],
}

#[repr(C)]
pub struct NVME_ZONE_EXTENDED_REPORT_ZONE_DESC {
    pub ZoneDescriptor: NVME_ZONE_DESCRIPTOR,
    pub ZoneDescriptorExtension: [NVME_ZONE_DESCRIPTOR_EXTENSION; 0], // ANYSIZE_ARRAY equivalent
}

#[repr(C)]
pub struct NVME_EXTENDED_REPORT_ZONE_INFO {
    pub ZoneCount: u64, // Number of Zones
    Reserved: [u64; 7],
    pub Desc: [NVME_ZONE_EXTENDED_REPORT_ZONE_DESC; 0], // ANYSIZE_ARRAY equivalent
}

#[repr(C)]
pub enum NVME_ZONE_RECEIVE_ACTION {
    NVME_ZONE_RECEIVE_REPORT_ZONES = 0, // Returns report zone Descriptors
    NVME_ZONE_RECEIVE_EXTENDED_REPORT_ZONES = 1, // Returns report zone descriptors with extended report zone information
}

#[repr(C)]
pub enum NVME_ZONE_RECEIVE_ACTION_SPECIFIC {
    NVME_ZRA_ALL_ZONES = 0,           // List all zones
    NVME_ZRA_EMPTY_STATE_ZONES = 1,   // List zones with state Zone State Empty
    NVME_ZRA_IO_STATE_ZONES = 2,      // List zones with state Zone State Implicitly Opened
    NVME_ZRA_EO_STATE_ZONES = 3,      // List zones with state Zone State Explicitly Opened
    NVME_ZRA_CLOSED_STATE_ZONES = 4,  // List zones with state Zone State Closed
    NVME_ZRA_FULL_STATE_ZONES = 5,    // List zones with state Zone State Full
    NVME_ZRA_RO_STATE_ZONES = 6,      // List zones with state Zone State Read-Only
    NVME_ZRA_OFFLINE_STATE_ZONES = 7, // List zones with state Zone State Offline
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_ZONE_MANAGEMENT_RECEIVE {
    pub SLBA: u64, // Starting LBA (SLBA)
}

// #[repr(C)]
// #[derive(Clone, Copy)]
// union NVME_CDW13_ZONE_MANAGEMENT_RECEIVE {
//     DUMMYSTRUCTNAME: NVME_CDW13_ZONE_MANAGEMENT_RECEIVE_FIELDS,
//     AsUlong: u32,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW13_ZONE_MANAGEMENT_RECEIVE {
    pub ZRA: B8,         // Zone Receive Action, as defined in NVME_ZONE_RECEIVE_ACTION
    pub ZRASpecific: B8, // Zone Receive Action Specific field, as defined in NVME_ZONE_RECEIVE_ACTION_SPECIFIC
    pub Partial: B1,     // Report Zones and Extended Report Zones: Partial Report
    Reserved: B15,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW10_ZONE_APPEND {
    pub SLBA: u64, // Starting LBA (SLBA)
}

// #[repr(C)]
// #[derive(Clone, Copy)]
// union NVME_CDW12_ZONE_APPEND {
//     DUMMYSTRUCTNAME: NVME_CDW12_ZONE_APPEND_FIELDS,
//     AsUlong: u32,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW12_ZONE_APPEND {
    pub NLB: B16, // Number of Logical Blocks (NLB)
    Reserved: B9,
    pub PIREMAP: B1, // Protection Information Remap (PIREMAP)
    pub PRINFO: B4,  // Protection Information Field (PRINFO)
    pub FUA: B1,     // Force Unit Access (FUA)
    pub LR: B1,      // Limited Retry(LR)
}

// #[repr(C)]
// #[derive(Clone, Copy)]
// union NVME_CDW15_ZONE_APPEND {
//     DUMMYSTRUCTNAME: NVME_CDW15_ZONE_APPEND_FIELDS,
//     AsUlong: u32,
// }

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_CDW15_ZONE_APPEND {
    pub LBAT: B16,  // Logical Block Application Tag
    pub LBATM: B16, // Logical Block Application Tag Mask (LBATM)
}

#[bitfield]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_DWORD0 {
    pub OPC: B8,  // Opcode (OPC)
    pub FUSE: B2, // Fused Operation (FUSE)
    pub Reserved0: B5,
    pub PSDT: B1, // PRP or SGL for Data Transfer (PSDT)
    pub CID: B16, // Command Identifier (CID)
}

#[repr(C)]
pub enum NVME_FUSED_OPERATION_CODES {
    NVME_FUSED_OPERATION_NORMAL = 0,
    NVME_FUSED_OPERATION_FIRST_CMD = 1,
    NVME_FUSED_OPERATION_SECOND_CMD = 2,
}

#[bitfield]
#[repr(u64)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_PRP_ENTRY {
    pub Reserved0: B2,
    pub PBAO: B62, // Page Base Address and Offset (PBAO)
}

const NVME_NAMESPACE_ALL: u32 = 0xFFFFFFFF;

//
// NVMe command data structure
//
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMMAND {
    pub CDW0: NVME_COMMAND_DWORD0,
    pub NSID: u32,
    Reserved0: [u32; 2],
    pub MPTR: u64,
    pub PRP1: u64,
    pub PRP2: u64,
    pub u: NVME_COMMAND_UNION,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_UNION {
    pub GENERAL: NVME_COMMAND_GENERAL,
    pub IDENTIFY: NVME_COMMAND_IDENTIFY,
    pub ABORT: NVME_COMMAND_ABORT,
    pub GETFEATURES: NVME_COMMAND_GETFEATURES,
    pub SETFEATURES: NVME_COMMAND_SETFEATURES,
    pub GETLOGPAGE: NVME_COMMAND_GETLOGPAGE,
    pub CREATEIOCQ: NVME_COMMAND_CREATEIOCQ,
    pub CREATEIOSQ: NVME_COMMAND_CREATEIOSQ,
    pub DATASETMANAGEMENT: NVME_COMMAND_DATASETMANAGEMENT,
    pub SECURITYSEND: NVME_COMMAND_SECURITYSEND,
    pub SECURITYRECEIVE: NVME_COMMAND_SECURITYRECEIVE,
    pub FIRMWAREDOWNLOAD: NVME_COMMAND_FIRMWAREDOWNLOAD,
    pub FIRMWAREACTIVATE: NVME_COMMAND_FIRMWAREACTIVATE,
    pub FORMATNVM: NVME_COMMAND_FORMATNVM,
    pub SANITIZE: NVME_COMMAND_SANITIZE,
    pub READWRITE: NVME_COMMAND_READWRITE,
}

impl Default for NVME_COMMAND_UNION {
    fn default() -> Self {
        NVME_COMMAND_UNION {
            GENERAL: NVME_COMMAND_GENERAL::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_GENERAL {
    pub CDW10: u32,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMMAND_IDENTIFY {
    pub CDW10: NVME_CDW10_IDENTIFY,
    pub CDW11: NVME_CDW11_IDENTIFY,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_ABORT {
    pub CDW10: NVME_CDW10_ABORT,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMMAND_GETFEATURES {
    pub CDW10: NVME_CDW10_GET_FEATURES,
    pub CDW11: NVME_CDW11_FEATURES,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMMAND_SETFEATURES {
    pub CDW10: NVME_CDW10_SET_FEATURES,
    pub CDW11: NVME_CDW11_FEATURES,
    pub CDW12: NVME_CDW12_FEATURES,
    pub CDW13: NVME_CDW13_FEATURES,
    pub CDW14: NVME_CDW14_FEATURES,
    pub CDW15: NVME_CDW15_FEATURES,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_GETLOGPAGE {
    pub CDW10: NVME_CDW10_GET_LOG_PAGE,
    pub CDW11: NVME_CDW11_GET_LOG_PAGE,
    pub CDW12: NVME_CDW12_GET_LOG_PAGE,
    pub CDW13: NVME_CDW13_GET_LOG_PAGE,
    pub CDW14: NVME_CDW14_GET_LOG_PAGE,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_CREATEIOCQ {
    pub CDW10: NVME_CDW10_CREATE_IO_QUEUE,
    pub CDW11: NVME_CDW11_CREATE_IO_CQ,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_CREATEIOSQ {
    pub CDW10: NVME_CDW10_CREATE_IO_QUEUE,
    pub CDW11: NVME_CDW11_CREATE_IO_SQ,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_DATASETMANAGEMENT {
    pub CDW10: NVME_CDW10_DATASET_MANAGEMENT,
    pub CDW11: NVME_CDW11_DATASET_MANAGEMENT,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_SECURITYSEND {
    pub CDW10: NVME_CDW10_SECURITY_SEND_RECEIVE,
    pub CDW11: NVME_CDW11_SECURITY_SEND,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_SECURITYRECEIVE {
    pub CDW10: NVME_CDW10_SECURITY_SEND_RECEIVE,
    pub CDW11: NVME_CDW11_SECURITY_RECEIVE,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_FIRMWAREDOWNLOAD {
    pub CDW10: NVME_CDW10_FIRMWARE_DOWNLOAD,
    pub CDW11: NVME_CDW11_FIRMWARE_DOWNLOAD,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_FIRMWAREACTIVATE {
    pub CDW10: NVME_CDW10_FIRMWARE_ACTIVATE,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_FORMATNVM {
    pub CDW10: NVME_CDW10_FORMAT_NVM,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_SANITIZE {
    pub CDW10: NVME_CDW10_SANITIZE,
    pub CDW11: NVME_CDW11_SANITIZE,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NVME_COMMAND_READWRITE {
    pub LBALOW: u32,
    pub LBAHIGH: u32,
    pub CDW12: NVME_CDW12_READ_WRITE,
    pub CDW13: NVME_CDW13_READ_WRITE,
    pub CDW14: u32,
    pub CDW15: NVME_CDW15_READ_WRITE,
}
