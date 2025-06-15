#[derive(Debug, Copy, Clone, Default)]
#[allow(dead_code)]
pub enum Address {
    #[default]
    Primary = 0b0100011,
    Secondary = 0b1011100,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Command {
    PowerDown = 0b0000_0000,
    PowerUp = 0b0000_0001,
    Reset = 0b0000_0111,
    ContinuouslyHighResolution = 0b0001_0000,
    ContinuouslyUltraHighResolution = 0b0001_0001,
    ContinuouslyLowResolution = 0b0001_0011,
    OneTimeHighResolution = 0b0010_0000,
    OneTimeUltraHighResolution = 0b0010_0001,
    OneTimeLowResolution = 0b0010_0011,
}
impl Into<u8> for Command {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<u8> for Address {
    fn into(self) -> u8 {
        self as u8
    }
}
