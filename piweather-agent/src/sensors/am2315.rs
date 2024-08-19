use crate::Sensor;
use i2cdev::core::I2CDevice;

const AM2315_I2C_SLAVE_ADDRESS: u8 = 0xB8;
const AM2315_I2C_WRITE_FUNC_CODE: u8 = 0x03;
const AM2315_I2C_READ_FUNC_CODE: u8 = 0x10;

#[derive(Debug, Copy, Clone)]
pub enum Am2315Readout {
    Temperature(f32),
    Humidity(f32),
}

pub struct Am2315<T: I2CDevice + Sized> {
    device: T,
}

impl<T> Am2315<T>
where
    T: I2CDevice + Sized,
{
    pub fn new(device: T) -> Self {
        Self { device }
    }

    pub fn temperature(&mut self) -> Result<Am2315Readout, T::Error> {
        self.device.write([])
    }

    pub fn humidity(&mut self) -> Result<Am2315Readout, T::Error> {
        unimplemented!()
    }
}

impl<T> Sensor<T> for Am2315<T>
where
    T: I2CDevice + Sized,
{
    type Output = Am2315Readout;
}
