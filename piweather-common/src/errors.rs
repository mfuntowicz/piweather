use i2cdev::linux::LinuxI2CError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PiWeatherError {
    #[error("Caught I2C error: {0}")]
    I2CError(String)
}

#[cfg(target_os = "linux")]
impl From<LinuxI2CError> for PiWeatherError {

    #[inline]
    fn from(error: LinuxI2CError) -> Self {
        Self::I2CError(error.to_string())
    }
}