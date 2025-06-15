#![no_std]
#![allow(async_fn_in_trait)]

pub trait LightSensor {
    type Error;
    async fn read_value(&mut self) -> Result<u16, Self::Error>;
}
