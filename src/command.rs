/// Command registers for the OpenLogger.
/// Purely for internal use.
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
#[allow(unused)]
pub enum Command {
    Status = 0x01,
    FirmwareMajor = 0x02,
    FirmwareMinor = 0x03,
    I2CAddress = 0x1E,
    LogInit = 0x05,
    CreateFile = 0x06,
    OpenFile = 0x0B,
    MkDir = 0x07,
    Cd = 0x08,
    ReadFile = 0x09,
    StartPosition = 0x0A,
    WriteFile = 0x0C,
    FileSize = 0x0D,
    List = 0x0E,
    Rm = 0x0F,
    RmRf = 0x10,
    SyncFile = 0x11,
}

impl Command {
    pub const fn as_bytes(&self) -> [u8; 1] {
        [*self as u8]
    }
}
