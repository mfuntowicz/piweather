use crate::sensors::Sensor;
use i2cdev::core::I2CDevice;
use piweather_common::errors::PiWeatherError;
use std::thread::sleep;
use std::time::{Duration, Instant};

const AM2315_I2C_SLAVE_ADDRESS: u8 = 0xB8;
const AM2315_I2C_READ_FUNC_CODE: u8 = 0x03;
const AM2315_I2C_READ_CALL: [u8; 3] = [AM2315_I2C_READ_FUNC_CODE, 0x0, 0x4];
const AM2315_WAKEUP_TIME_MS: Duration = Duration::from_millis(100);

#[derive(Debug, Copy, Clone)]
pub enum Am2315Readout {
    Temperature(f32),
    Humidity(f32),
}

pub struct Am2315<T: I2CDevice + Sized> {
    last_read: Option<Instant>,
    last_readouts: Option<[Am2315Readout; 2]>,
    device: T,
}

impl<T> Am2315<T>
where
    T: I2CDevice + Sized,
{
    pub fn new(device: T) -> Self {
        Self {
            last_read: None,
            last_readouts: None,
            device,
        }
    }

    pub fn read(&mut self) -> Result<Option<[Am2315Readout; 2]>, PiWeatherError> {
        if let Some(last_read) = self.last_read {
            let since_last_read = Instant::now() - last_read;
            if since_last_read > Duration::from_secs(2) {
                self.device
                    .write(&[AM2315_I2C_READ_FUNC_CODE])
                    .map_err(|_| {
                        PiWeatherError::I2CError("Failed to write wake-up op to AM2315".into())
                    })?;
                sleep(AM2315_WAKEUP_TIME_MS);

                // Create the buffers to send & store the request and response content
                let mut data = [0u8; 8];
                self.device.write(&AM2315_I2C_READ_CALL).map_err(|_| {
                    PiWeatherError::I2CError("Failed to write read op to AM2315".into())
                })?;
                self.device.read(&mut data).map_err(|_| {
                    PiWeatherError::I2CError("Failed to read data from AM2315".into())
                })?;

                // Update last time we read the sensor
                self.last_read = Some(Instant::now());

                // Sanity checks
                if data[0] != AM2315_I2C_READ_FUNC_CODE {
                    return Err(PiWeatherError::Io("Mismatched op-code".into()));
                }

                if data[1] != 4 {
                    return Err(PiWeatherError::Io("Mismatched number of bytes read".into()));
                }

                // Convert to meaningful values
                let humidity = Self::humidity_from_le_bytes(data[2], data[3]);
                let temperature = Self::temperature_from_le_bytes(data[4], data[5]);

                let readouts = Some([
                    Am2315Readout::Temperature(temperature),
                    Am2315Readout::Humidity(humidity),
                ]);

                self.last_readouts = readouts;
            }
        }

        Ok(self.last_readouts)
    }

    fn temperature_from_le_bytes(low: u8, high: u8) -> f32 {
        let t = u16::from_le_bytes([high, low & 0x7F]);

        // Convert to float
        let temperature = t as f32 / 10.0f32;
        if (low >> 7) & 0x1 == 1 {
            -temperature
        } else {
            temperature
        }
    }

    fn humidity_from_le_bytes(low: u8, high: u8) -> f32 {
        let h = u16::from_le_bytes([high, low]);
        h as f32 / 10.0
    }
}

impl<T> Sensor<T> for Am2315<T>
where
    T: I2CDevice + Sized,
{
    type Output = Am2315Readout;
}

#[cfg(test)]
mod tests {
    use crate::sensors::am2315::Am2315;
    use i2cdev::mock::MockI2CDevice;

    fn read_humidity() {
        let humidity = Am2315::<MockI2CDevice>::humidity_from_le_bytes(0x39, 0x03);
        assert_eq!(humidity, 82.5);
    }

    fn read_temperature() {
        let temperature = Am2315::<MockI2CDevice>::temperature_from_le_bytes(0x15, 0x01);
        assert_eq!(temperature, 27.7);
    }
}
