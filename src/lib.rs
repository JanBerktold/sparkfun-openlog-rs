//! A platform-agnostic, embedded-hal driver for the [Sparkfun OpenLogger](https://www.sparkfun.com/products/13712).
#![no_std]

mod command;
use command::Command;

mod version;
use embedded_hal::i2c::Operation;
pub use version::Version;

mod status;
pub use status::Status;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub enum DeviceAddr {
    #[default]
    ADDR1 = 0x2A,
    ADDR2 = 0x29,
}

#[derive(Debug)]
pub struct OpenLogger<I2C> {
    address: DeviceAddr,
    i2c: I2C,
}

impl<I2C> OpenLogger<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    /// Create a new OpenLogger instance.
    pub fn new(address: DeviceAddr, i2c: I2C) -> Self {
        Self { i2c, address }
    }

    /// Create a new OpenLogger instance and validate that it is ready for writing.
    pub fn new_and_validate(address: DeviceAddr, i2c: I2C) -> Result<Self, I2C::Error> {
        let mut logger = Self::new(address, i2c);
        let status = logger.get_status()?;

        if !status.init_ok() {
            todo!("find the correct error type");
        }

        Ok(logger)
    }

    pub fn get_version(&mut self) -> Result<Version, I2C::Error> {
        let mut major = [0u8; 2];
        self.i2c.write_read(
            self.address as u8,
            &Command::FirmwareMajor.as_bytes(),
            &mut major,
        )?;

        let mut minor = [0u8; 2];
        self.i2c.write_read(
            self.address as u8,
            &Command::FirmwareMinor.as_bytes(),
            &mut minor,
        )?;

        Ok(Version {
            major: major[0],
            minor: minor[0],
        })
    }

    pub fn get_status(&mut self) -> Result<Status, I2C::Error> {
        let mut result = [0u8; 1];
        self.i2c
            .write_read(self.address as u8, &Command::Status.as_bytes(), &mut result)?;

        Ok(Status::from(result[0]))
    }

    /// Create a new directory, relative to the current directory.
    pub fn make_directory(&mut self, directory: &str) -> Result<(), I2C::Error> {
        let command = Command::MkDir.as_bytes();

        let mut operations = [
            Operation::Write(&command),
            Operation::Write(directory.as_bytes()),
        ];

        self.i2c.transaction(self.address as u8, &mut operations)
    }

    /// Open and append to a file.
    pub fn append(&mut self, file: &str) -> Result<(), I2C::Error> {
        let command = Command::OpenFile.as_bytes();

        let mut operations = [
            Operation::Write(&command),
            Operation::Write(file.as_bytes()),
        ];

        self.i2c.transaction(self.address as u8, &mut operations)
    }

    /// Create a new file, but don't open it for writing.
    pub fn create(&mut self, file: &str) -> Result<(), I2C::Error> {
        let command = Command::CreateFile.as_bytes();

        let mut operations = [
            Operation::Write(&command),
            Operation::Write(file.as_bytes()),
        ];

        self.i2c.transaction(self.address as u8, &mut operations)
    }
}

/*
    Outstanding commands:

    virtual size_t write(uint8_t character);
    int writeString(String string);
    bool syncFile(void);

    boolean setI2CAddress(uint8_t addr); //Set the I2C address we read and write to
    boolean changeDirectory(String directoryName); //Change to the given directory
    int32_t size(String fileName); //Given a file name, read the size of the file

    void read(uint8_t* userBuffer, uint16_t bufferSize, String fileName); //Read the contents of a file into the provided buffer

    boolean searchDirectory(String options); //Search the current directory for a given wildcard
    String getNextDirectoryItem(); //Return the next file or directory from the search

    uint32_t removeFile(String thingToDelete); //Remove file
    uint32_t removeDirectory(String thingToDelete); //Remove a directory including the contents of the directory
    uint32_t remove(String thingToDelete, boolean removeEverthing); //Remove file or directory including the contents of the directory
*/
