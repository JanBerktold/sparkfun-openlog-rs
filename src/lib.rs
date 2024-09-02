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

#[repr(u8)]
enum Command {
    Status = 0x01,
    FirmwareMajor = 0x02,
    FirmwareMinor = 0x03,
    /*
        .i2cAddress = 0x1E,
        .logInit = 0x05,
        .createFile = 0x06,
        .mkDir = 0x07,
        .cd = 0x08,
        .readFile = 0x09,
        .startPosition = 0x0A,
        .openFile = 0x0B,
        .writeFile = 0x0C,
        .fileSize = 0x0D,
        .list = 0x0E,
        .rm = 0x0F,
        .rmrf = 0x10,
    .syncFile = 0x11,
     */
}

impl<I2C> OpenLogger<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    pub fn new(address: DeviceAddr, i2c: I2C) -> Self {
        Self { i2c, address }
    }

    pub fn get_version(&mut self) -> Result<String, I2C::Error> {
        let command = [Command::FirmwareMajor as u8];
        let mut major = [0u8; 2];
        self.i2c
            .write_read(self.address as u8, &command, &mut major)?;

        let command = [Command::FirmwareMinor as u8];
        let mut minor = [0u8; 2];
        self.i2c
            .write_read(self.address as u8, &command, &mut minor)?;

        Ok(format!("{}.{}", major[0], minor[0]))

        /*
          sendCommand(registerMap.firmwareMajor, "");
          // Upon completion Qwiic OpenLog will have 2 bytes ready to be read
          _i2cPort->requestFrom(_deviceAddress, (uint8_t)1);

          uint8_t versionMajor = _i2cPort->read();
          sendCommand(registerMap.firmwareMinor, "");
          // Upon completion Qwiic OpenLog will have 2 bytes ready to be read
          _i2cPort->requestFrom(_deviceAddress, (uint8_t)1);

          uint8_t versionMinor = _i2cPort->read();

          return (String(versionMajor) + "." + String(versionMinor));
        */
    }

    pub fn get_status(&mut self) -> Result<Status, I2C::Error> {
        let command = [Command::Status as u8];
        let mut result = [0u8; 1];
        self.i2c
            .write_read(self.address as u8, &command, &mut result)?;

        Ok(Status::from(result[0]))
    }
}

/*
    Outstanding commands:

    virtual size_t write(uint8_t character);
    int writeString(String string);
    bool syncFile(void);

    boolean setI2CAddress(uint8_t addr); //Set the I2C address we read and write to
    boolean append(String fileName); //Open and append to a file
    boolean create(String fileName); //Create a file but don't open it for writing
    boolean makeDirectory(String directoryName); //Create the given directory
    boolean changeDirectory(String directoryName); //Change to the given directory
    int32_t size(String fileName); //Given a file name, read the size of the file

    void read(uint8_t* userBuffer, uint16_t bufferSize, String fileName); //Read the contents of a file into the provided buffer

    boolean searchDirectory(String options); //Search the current directory for a given wildcard
    String getNextDirectoryItem(); //Return the next file or directory from the search

    uint32_t removeFile(String thingToDelete); //Remove file
    uint32_t removeDirectory(String thingToDelete); //Remove a directory including the contents of the directory
    uint32_t remove(String thingToDelete, boolean removeEverthing); //Remove file or directory including the contents of the directory
*/
