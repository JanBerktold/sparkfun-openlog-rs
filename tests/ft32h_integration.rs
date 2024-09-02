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

#[test]
fn test_writing_to_file() -> anyhow::Result<()> {
    let device = Ftdi::new()?;
    let device: Ft232h = device.try_into()?;

    let hal = FtHal::init_freq(device, 50_000)?;
    let i2c = hal.i2c()?;

    let mut logger = OpenLogger::new_and_validate(DeviceAddr::ADDR1, i2c)?;

    let file = "test.txt";

    // TODO: Delete file;

    let size = logger.size(&file)?;
    assert_eq!(size, 0);

    println!("Creating file");

    // Open the file for writing.
    logger.append(&file)?;

    println!("Writing to file");
    logger.write_string("hello")?;

    println!("Writing to file2");
    logger.sync_file()?;

    let size = logger.size(&file)?;
    assert_eq!(size, 4);

    Ok(())
}
