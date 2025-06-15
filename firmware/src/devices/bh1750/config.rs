use crate::devices::bh1750::registers::Address;

#[derive(Default)]
pub struct Config {
    pub address: Address,
    pub resolution_mode: ResolutionMode,
    pub operation_mode: OperationMode,
}

#[derive(Default)]
#[allow(dead_code)]
pub enum ResolutionMode {
    #[default]
    UltraHighResolution, //120ms 0.5lx
    HighResolution, //120ms 1lx
    LowResolution,  //16ms 4lx
}

#[derive(Default, Eq, PartialEq)]
#[allow(dead_code)]
pub enum OperationMode {
    #[default]
    Continuously,
    OneTime,
}
