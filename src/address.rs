/// The I2C address of the OpenLogger.
#[derive(Copy, Clone, Debug, Default)]
pub enum DeviceAddr {
    /// The default address, 0x2A.
    #[default]
    ADDR1,
    /// The secondary address, 0x29.
    ADDR2,
    /// Any other I2C address, e.g. when using a mux.
    Custom(u8),
}

impl From<DeviceAddr> for u8 {
    fn from(addr: DeviceAddr) -> u8 {
        match addr {
            DeviceAddr::ADDR1 => 0x2A,
            DeviceAddr::ADDR2 => 0x29,
            DeviceAddr::Custom(addr) => addr,
        }
    }
}
