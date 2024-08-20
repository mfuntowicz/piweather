use crate::i2c::I2CDeviceFactory;
use crate::sensors::Sensor;
use i2cdev::core::I2CDevice;
use piweather_common::errors::PiWeatherError;
use piweather_common::{Modality, Payload, Temperature};
use std::thread::sleep;
use std::time::{Duration, Instant};
use tracing::debug;

const AM2315_I2C_SLAVE_ADDRESS: u8 = 0x5C;
const AM2315_I2C_READ_FUNC_CODE: u8 = 0x03;
const AM2315_I2C_READ_CALL: [u8; 3] = [AM2315_I2C_READ_FUNC_CODE, 0x0, 0x4];
const AM2315_WAKEUP_TIME_MS: Duration = Duration::from_millis(100);
const AM2315_READ_INTERVAL_SEC: Duration = Duration::from_secs(2);

#[derive(Debug, Copy, Clone)]
pub enum Am2315Readout {
    Temperature(f32),
    Humidity(f32),
}

impl From<Am2315Readout> for Modality {
    fn from(value: Am2315Readout) -> Self {
        match value {
            Am2315Readout::Temperature(t) => Modality::Temperature(Temperature::Celsius(t)),
            Am2315Readout::Humidity(h) => Modality::Humidity(h),
        }
    }
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
            if since_last_read < AM2315_READ_INTERVAL_SEC {
                debug!(
                    "AM2315 read {}s ago, using cached value",
                    &since_last_read.as_secs_f32()
                );
                return Ok(self.last_readouts);
            }
        }

        // Wake up the sensor (sleeping to avoid self-heating)
        let _ = self.device.write(&[]);
        sleep(AM2315_WAKEUP_TIME_MS);

        // Create the buffers to send & store the request and response content
        let mut data = [0u8; 8];
        self.device.write(&AM2315_I2C_READ_CALL).map_err(|e| {
            PiWeatherError::I2CError(format!("Failed to write read op to AM2315: {}", e))
        })?;

        self.device.read(&mut data).map_err(|e| {
            PiWeatherError::I2CError(format!("Failed to read data from AM2315: {}", e))
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
        Ok(readouts)
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

impl<F, D> Sensor<F, D, 2> for Am2315<D>
where
    F: I2CDeviceFactory<Device = D>,
    D: I2CDevice + Sized,
    Self: Sized,
{
    fn with_i2c_factory(factory: F) -> Result<Self, PiWeatherError> {
        let device = factory.open(AM2315_I2C_SLAVE_ADDRESS as u16)?;
        Ok(Am2315::new(device))
    }

    fn payload(&mut self) -> Result<Option<Payload<2>>, PiWeatherError> {
        if let Some(readouts) = self.read()? {
            let modalities = [readouts[0].into(), readouts[1].into()];
            return Ok(Some(Payload::now(modalities)));
        }

        Ok(None)
    }
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
