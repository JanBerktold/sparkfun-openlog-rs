use std::fmt;

#[derive(Debug)]
pub struct Status(u8);

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Status: (")?;
        write!(f, "Init OK = {}, ", self.init_ok() as u8)?;
        write!(
            f,
            "Last Cmd Success = {}, ",
            self.last_command_succeeded() as u8
        )?;
        write!(f, "Last Cmd Known = {}, ", self.last_command_known() as u8)?;
        write!(f, "File Open = {}, ", self.file_open() as u8)?;
        write!(f, "In Root = {})", self.in_root_directory() as u8)
    }
}

impl Status {
    pub fn init_ok(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    pub fn last_command_succeeded(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    pub fn last_command_known(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    pub fn file_open(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    pub fn in_root_directory(&self) -> bool {
        self.0 & (1 << 4) != 0
    }
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_reading() {
        let status = Status::from(0b00010101);
        assert!(status.init_ok());
        assert!(!status.last_command_succeeded());
        assert!(status.last_command_known());
        assert!(!status.file_open());
        assert!(status.in_root_directory());
    }
}
