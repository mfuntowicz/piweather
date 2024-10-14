mod am2315;
mod pmsa003;

use crate::i2c::I2CDeviceFactory;
pub use am2315::*;
use i2cdev::core::I2CDevice;
use piweather_common::errors::PiWeatherError;
use piweather_common::Modality;

pub trait Sensor<D, const N: usize>
where
    D: I2CDevice + Sized,
    Self: Sized,
{
    const NAME: &'static str;
    const CARDINALITY: usize = N;

    #[inline]
    fn name(&self) -> &'static str {
        const { Self::NAME }
    }

    ///
    fn with_i2c_factory<F>(factory: F) -> Result<Self, PiWeatherError>
    where
        F: I2CDeviceFactory<Device = D>;

    ///
    ///
    fn probe(&mut self) -> Result<Option<[Modality; N]>, PiWeatherError>;
}
