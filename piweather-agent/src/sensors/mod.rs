mod am2315;

use i2cdev::core::I2CDevice;

pub trait Sensor<T>
where
    T: I2CDevice + Sized,
{
    type Output: Sized;
}
