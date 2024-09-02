use ftdi_embedded_hal::{
    libftd2xx::{Ft232h, Ftdi},
    FtHal,
};
use sparkfun_openlog::{DeviceAddr, OpenLogger};

#[test]
fn test_status() -> anyhow::Result<()> {
    let device = Ftdi::new()?;
    let device: Ft232h = device.try_into()?;

    let hal = FtHal::init_freq(device, 50_000)?;
    let i2c = hal.i2c()?;

    let mut logger = OpenLogger::new(DeviceAddr::ADDR1, i2c);

    let status = logger.get_status()?;
    assert!(status.init_ok());

    println!("Gathered: {}", status);

    println!("OpenLogger is running firmware {}", logger.get_version()?);

    Ok(())
}
