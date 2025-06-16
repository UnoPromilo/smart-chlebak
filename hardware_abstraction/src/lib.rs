#![no_std]
#![allow(async_fn_in_trait)]

pub trait LightSensor {
    type Error;
    async fn read_value(&mut self) -> Result<u16, Self::Error>; // Returns value in lux
}

pub trait PressureSensor {
    type Error;
    async fn read_value(&mut self) -> Result<Pressure, Self::Error>;
}

pub trait TemperatureSensor {
    type Error;
    async fn read_value(&mut self) -> Result<Temperature, Self::Error>;
}

pub struct Pressure {
    raw_value: u32, //Q24.8 *100
}

pub struct Temperature {
    raw_value: i32, //C *100
}

impl Pressure {
    pub fn from_q248_100(value: u32) -> Self {
        Pressure { raw_value: value }
    }

    pub fn to_hpa(&self) -> f32 {
        self.raw_value as f32 / (25600f32)
    }
}

impl Temperature {
    pub fn from_deg_c_100(value: i32) -> Self {
        Temperature { raw_value: value }
    }

    pub fn to_deg_c(&self) -> f32 {
        self.raw_value as f32 / 100f32
    }
}
