pub mod sensors;

use i2cdev::core::I2CDevice;

pub trait Sensor<T>
where
    T: I2CDevice + Sized,
{
    type Output: Sized;
}
pub mod i2c;
