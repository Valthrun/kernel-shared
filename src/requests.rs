use crate::{
    KeyboardState,
    ModuleInfo,
    MouseState,
    ProcessId,
    ProcessModuleInfo,
    IO_MAX_DEREF_COUNT,
};

pub trait DriverRequest: Sized + Copy {
    type Result: Send + Sync + Sized + Copy + Default;

    /// The 10 bit user function code for the request
    fn function_code() -> u16;
}

#[derive(Debug, Clone, Copy)]
pub struct RequestHealthCheck;

#[derive(Debug, Default, Clone, Copy)]
pub struct ResponseHealthCheck {
    pub success: bool,
}

impl DriverRequest for RequestHealthCheck {
    type Result = ResponseHealthCheck;

    fn function_code() -> u16 {
        0x01
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequestCSModule {
    /// Number of elements module_buffer can hold
    pub module_buffer_length: usize,
    pub module_buffer: *mut ModuleInfo,
}

#[derive(Debug, Clone, Copy)]
pub enum ResponseProcessModules {
    Success(ProcessModuleInfo),
    BufferTooSmall { expected: usize },
    UbiquitousProcesses(usize),
    NoProcess,
}
impl Default for ResponseProcessModules {
    fn default() -> Self {
        Self::NoProcess
    }
}
impl DriverRequest for RequestCSModule {
    type Result = ResponseProcessModules;

    fn function_code() -> u16 {
        0x02
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryAccessMode {
    /// Read from/write to the process memory using `KeStackAttachProcess`
    AttachProcess,

    /// Read from/write to the process memory using `MmCopyVirtualMemory`
    CopyVirtualMemory,

    /// Read from/write to the process memory by manually resolving the physical address
    /// and read from physical memory.
    Physical,
}

#[derive(Debug, Clone, Copy)]
pub struct RequestRead {
    pub process_id: ProcessId,
    pub mode: MemoryAccessMode,

    pub offsets: [u64; IO_MAX_DEREF_COUNT],
    pub offset_count: usize,

    pub buffer: *mut u8,
    pub count: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum ResponseRead {
    Success,
    InvalidAddress {
        resolved_offsets: [u64; IO_MAX_DEREF_COUNT],
        resolved_offset_count: usize,
    },
    UnknownProcess,

    /// The desired access mode is unavailable ether because it's not supported
    /// or not enabled in your driver version.
    AccessModeUnavailable,
}
impl Default for ResponseRead {
    fn default() -> Self {
        Self::InvalidAddress {
            resolved_offsets: Default::default(),
            resolved_offset_count: 0,
        }
    }
}
impl DriverRequest for RequestRead {
    type Result = ResponseRead;

    fn function_code() -> u16 {
        0x03
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequestProtectionToggle {
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ResponseProtectionToggle;

impl DriverRequest for RequestProtectionToggle {
    type Result = ResponseProtectionToggle;

    fn function_code() -> u16 {
        0x04
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequestMouseMove {
    pub buffer: *const MouseState,
    pub state_count: usize,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ResponseMouseMove;

impl DriverRequest for RequestMouseMove {
    type Result = ResponseMouseMove;

    fn function_code() -> u16 {
        0x05
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequestKeyboardState {
    pub buffer: *const KeyboardState,
    pub state_count: usize,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ResponseKeyboardState;

impl DriverRequest for RequestKeyboardState {
    type Result = ResponseKeyboardState;

    fn function_code() -> u16 {
        0x06
    }
}

/// Success
pub const INIT_STATUS_SUCCESS: u32 = 0x01;
/// The requested driver version is newer then supported
pub const INIT_STATUS_DRIVER_OUTDATED: u32 = 0x02;
/// The requested driver version is older then supported
pub const INIT_STATUS_CONTROLLER_OUTDATED: u32 = 0x03;

/// Additional information provided by the controller, which can be changed depending on the version.
pub struct ControllerInfo {}

/// Additional driver information, which can be changed depending on the version.
pub struct DriverInfo {}

#[derive(Debug, Clone, Copy)]
pub struct RequestInitialize {
    pub target_version: u32,

    pub controller_info: *const ControllerInfo,
    pub controller_info_length: usize,

    pub driver_info: *mut DriverInfo,
    pub driver_info_length: usize,
}

/// Driver initializsation response.
/// This should never be changed, as this would brack backwards compatibility issues
/// on detecting whatever the driver is compatible with the requested version.
///
/// Exchanging data should be done via controller_info or driver_info after verifying the version.
#[derive(Debug, Default, Clone, Copy)]
pub struct ResponseInitialize {
    pub driver_version: u32,
    pub status_code: u32,
}

impl DriverRequest for RequestInitialize {
    type Result = ResponseInitialize;

    fn function_code() -> u16 {
        0x07
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequestReportSend {
    pub report_type: *const u8,
    pub report_type_length: usize,

    pub report_payload: *const u8,
    pub report_payload_length: usize,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ResponseReportSend;

impl DriverRequest for RequestReportSend {
    type Result = ResponseReportSend;

    fn function_code() -> u16 {
        0x08
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequestWrite {
    pub process_id: ProcessId,
    pub mode: MemoryAccessMode,
    pub address: usize,

    pub buffer: *const u8,
    pub count: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum ResponseWrite {
    Success,
    InvalidAddress,
    UnknownProcess,
    UnsuppportedAccessMode,
}
impl Default for ResponseWrite {
    fn default() -> Self {
        Self::InvalidAddress
    }
}
impl DriverRequest for RequestWrite {
    type Result = ResponseWrite;

    fn function_code() -> u16 {
        0x09
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequestProcessModules {
    pub filter: ProcessFilter,

    /// Number of elements module_buffer can hold
    pub module_buffer_length: usize,
    pub module_buffer: *mut ModuleInfo,
}
impl DriverRequest for RequestProcessModules {
    type Result = ResponseProcessModules;

    fn function_code() -> u16 {
        0x0A
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessFilter {
    Id { id: i32 },
    Name { name: *const u8, name_length: usize },
}
