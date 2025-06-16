#[allow(unused)]
pub const RESET_VALUE: u8 = 0xB7;

#[derive(Debug, Copy, Clone, Default)]
#[allow(dead_code)]
pub enum Address {
    #[default]
    Primary = 0x76,
    Secondary = 0x77,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Register {
    CalibrationData = 0x88,
    Id = 0xD0,
    Reset = 0xE0,
    Status = 0xF3,
    Config = 0xF5,
    Ctrl = 0xF4,
    Temperature = 0xFA, //[FA-FC]
    Pressure = 0xF7,    //[F7-F9]
}
impl Into<u8> for Register {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<u8> for Address {
    fn into(self) -> u8 {
        self as u8
    }
}
