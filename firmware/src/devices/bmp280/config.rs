use crate::devices::bmp280::registers::Address;

#[derive(Default)]
pub struct Config {
    pub address: Address,
    pub temperature: Oversampling,
    pub pressure: Oversampling,
    pub filter: Filter,
    pub standby_time: StandbyTime,
}

#[derive(Default, Copy, Clone)]
#[allow(dead_code)]
pub enum Oversampling {
    Skipper = 0b000,
    UltraLowPower = 0b001, //16bit
    LowPower = 0b010,      //17bit
    #[default]
    StandardResolution = 0b011, //18bit
    HighResolution = 0b100, //19bit
    UltrahighResolution = 0b101, //20bit
}

#[derive(Default, Copy, Clone)]
#[allow(dead_code)]
pub enum Filter {
    Off = 0b000,
    X2 = 0b001, //16bit
    #[default]
    X4 = 0b010, //17bit
    X8 = 0b011, //18bit
    X16 = 0b100, //19bit
}

#[derive(Default, Copy, Clone)]
#[allow(dead_code)]
pub enum StandbyTime {
    #[default]
    Zero = 0b000,
    Ms62 = 0b001,
    Ms125 = 0b010,
    Ms250 = 0b011,
    Ms500 = 0b100,
    Ms1000 = 0b101,
    Ms2000 = 0b110,
    Ms4000 = 0b111,
}

impl Into<u8> for Oversampling {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<u8> for Filter {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<u8> for StandbyTime {
    fn into(self) -> u8 {
        self as u8
    }
}
