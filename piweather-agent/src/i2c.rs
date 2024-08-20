use crate::i2c::linux::LinuxI2CDeviceFactory;
use i2cdev::core::I2CDevice;
use piweather_common::errors::PiWeatherError;
use std::path::Path;

pub trait I2CDeviceFactory {
    type Device: I2CDevice + Sized;

    /// Attempt to open the device at the specified address
    ///
    fn open(&self, address: u16) -> Result<Self::Device, PiWeatherError>;
}

#[cfg(target_os = "linux")]
pub fn get_os_i2c_factory<P: AsRef<Path>>(
    fd: P,
) -> Result<LinuxI2CDeviceFactory<P>, PiWeatherError> {
    LinuxI2CDeviceFactory::new(fd)
}

#[cfg(target_os = "linux")]
pub mod linux {
    use super::I2CDeviceFactory;
    use i2cdev::linux::LinuxI2CDevice;
    use piweather_common::errors::PiWeatherError;
    use std::path::Path;

    pub struct LinuxI2CDeviceFactory<P: AsRef<Path>> {
        fd: P,
    }

    impl<P: AsRef<Path>> LinuxI2CDeviceFactory<P> {
        pub fn new(fd: P) -> Result<Self, PiWeatherError> {
            if fd.as_ref().exists() {
                Ok(Self { fd })
            } else {
                Err(PiWeatherError::I2CError(format!(
                    "{} I2C device doesn't exist",
                    fd.as_ref().display()
                )))
            }
        }
    }

    impl<P: AsRef<Path>> I2CDeviceFactory for LinuxI2CDeviceFactory<P> {
        type Device = LinuxI2CDevice;

        #[inline]
        fn open(&self, address: u16) -> Result<Self::Device, PiWeatherError> {
            LinuxI2CDevice::new(&self.fd.as_ref(), address)
                .map_err(|err| PiWeatherError::I2CError(err.to_string()))
        }
    }
}
