use core::fmt;

#[derive(Clone, Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}
