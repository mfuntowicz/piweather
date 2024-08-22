mod am2315;
mod pmsa003;

use crate::i2c::I2CDeviceFactory;
pub use am2315::*;
use piweather_common::errors::PiWeatherError;
use piweather_common::Payload;

pub trait Sensor<T, D, const N: usize>
where
    T: I2CDeviceFactory<Device = D>,
    Self: Sized,
{
    const NAME: &'static str;

    ///
    fn with_i2c_factory(factory: T) -> Result<Self, PiWeatherError>;

    ///
    ///
    fn payload(&mut self) -> Result<Option<Payload>, PiWeatherError>;
}
