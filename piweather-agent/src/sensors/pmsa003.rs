use crate::i2c::I2CDeviceFactory;
use crate::sensors::Sensor;
use i2cdev::core::I2CDevice;
use piweather_common::errors::PiWeatherError;
use piweather_common::{Modality, Particle};

const PMSA003_I2C_SLAVE_ADDRESS: u16 = 0x12;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PmsA003Particle {
    PM0_3 = 3,
    PM0_5 = 5,
    PM1_0 = 10,
    PM2_5 = 25,
    PM5_0 = 50,
    PM10_0 = 100,
}

impl From<PmsA003Particle> for Particle {
    fn from(value: PmsA003Particle) -> Self {
        match value {
            PmsA003Particle::PM0_3 => Particle::PM0_3,
            PmsA003Particle::PM0_5 => Particle::PM0_5,
            PmsA003Particle::PM1_0 => Particle::PM1_0,
            PmsA003Particle::PM2_5 => Particle::PM2_5,
            PmsA003Particle::PM5_0 => Particle::PM5_0,
            PmsA003Particle::PM10_0 => Particle::PM10_0,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ConcentrationUnit {
    Standard,
    Environmental,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PmsA003Readout {
    Concentration(PmsA003Particle, ConcentrationUnit, u16),
    Count(PmsA003Particle, u16),
}

impl From<PmsA003Readout> for Modality {
    fn from(value: PmsA003Readout) -> Self {
        value.into()
    }
}

pub struct PmsA003<T: I2CDevice + Sized> {
    device: T,
}

impl<T> PmsA003<T>
where
    T: I2CDevice + Sized,
{
    pub fn new(device: T) -> Self {
        Self { device }
    }

    fn read(&mut self) -> Result<Option<[PmsA003Readout; 12]>, PiWeatherError> {
        let mut data = [0u8; 32];

        self.device.read(&mut data).map_err(|e| {
            PiWeatherError::I2CError(format!("Failed to read data from PmsA003: {}", e))
        })?;

        // Check headers and size of the payload
        if data[0] != 'B' as u8 || data[1] != 'M' as u8 {
            return Err(PiWeatherError::I2CError(
                "Invalid header received from PmsA003".into(),
            ));
        }

        if u16::from_be_bytes([data[2], data[3]]) != 28 {
            return Err(PiWeatherError::I2CError(
                "Invalid decoded frame size received from PmsA003".into(),
            ));
        }

        // TODO : Maybe we can optimize the remaining elements as it does not leverage
        // packed instructions...
        let checksum = u16::from_be_bytes([data[30], data[31]]);
        let sum: u16 = data[0..30].iter().fold(0u16, |acc, x| acc + (*x as u16));

        if sum != checksum {
            return Err(PiWeatherError::I2CError(
                "Invalid checksum data received from PmsA003".into(),
            ));
        }

        Ok(Some([
            PmsA003Readout::Concentration(
                PmsA003Particle::PM1_0,
                ConcentrationUnit::Standard,
                u16::from_be_bytes([data[4], data[5]]),
            ),
            PmsA003Readout::Concentration(
                PmsA003Particle::PM2_5,
                ConcentrationUnit::Standard,
                u16::from_be_bytes([data[6], data[7]]),
            ),
            PmsA003Readout::Concentration(
                PmsA003Particle::PM10_0,
                ConcentrationUnit::Standard,
                u16::from_be_bytes([data[8], data[9]]),
            ),
            PmsA003Readout::Concentration(
                PmsA003Particle::PM1_0,
                ConcentrationUnit::Environmental,
                u16::from_be_bytes([data[10], data[11]]),
            ),
            PmsA003Readout::Concentration(
                PmsA003Particle::PM2_5,
                ConcentrationUnit::Environmental,
                u16::from_be_bytes([data[12], data[13]]),
            ),
            PmsA003Readout::Concentration(
                PmsA003Particle::PM10_0,
                ConcentrationUnit::Environmental,
                u16::from_be_bytes([data[14], data[15]]),
            ),
            PmsA003Readout::Count(
                PmsA003Particle::PM0_3,
                u16::from_be_bytes([data[16], data[17]]),
            ),
            PmsA003Readout::Count(
                PmsA003Particle::PM0_5,
                u16::from_be_bytes([data[18], data[19]]),
            ),
            PmsA003Readout::Count(
                PmsA003Particle::PM1_0,
                u16::from_be_bytes([data[20], data[21]]),
            ),
            PmsA003Readout::Count(
                PmsA003Particle::PM2_5,
                u16::from_be_bytes([data[22], data[23]]),
            ),
            PmsA003Readout::Count(
                PmsA003Particle::PM5_0,
                u16::from_be_bytes([data[24], data[25]]),
            ),
            PmsA003Readout::Count(
                PmsA003Particle::PM10_0,
                u16::from_be_bytes([data[26], data[27]]),
            ),
        ]))
    }
}

impl<D> Sensor<D, 12> for PmsA003<D>
where
    D: I2CDevice + Sized,
    Self: Sized,
{
    const NAME: &'static str = "PMSA003";

    fn with_i2c_factory<F>(factory: F) -> Result<Self, PiWeatherError>
    where
        F: I2CDeviceFactory<Device = D>,
    {
        let device = factory.open(PMSA003_I2C_SLAVE_ADDRESS)?;
        Ok(Self { device })
    }

    fn probe(&mut self) -> Result<Option<[Modality; 12]>, PiWeatherError> {
        if let Some(readouts) = self.read()? {
            let modalities = readouts.map(Modality::from);
            return Ok(Some(modalities));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::sensors::pmsa003::ConcentrationUnit::{Environmental, Standard};
    use crate::sensors::pmsa003::{PmsA003, PmsA003Particle, PmsA003Readout};
    use i2cdev::mock::MockI2CDevice;

    #[test]
    fn pmsa003_read() {
        const REGISTER: [u8; 32] = [
            'B' as u8,
            'M' as u8,
            0,
            28,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            0,
            (26 + 28 + 66 + 77),
        ];

        // Write dummy data to the register
        let mut device = MockI2CDevice::new();
        (&mut device.regmap).write_regs(0x0, &REGISTER);

        let mut pmsa003 = PmsA003::new(device);

        // Handle read
        let readouts = pmsa003.read();
        assert!(readouts.is_ok(), "Error while reading from the sensor");

        let readouts = readouts.unwrap();
        if let Some(readouts) = readouts {
            assert_eq!(
                readouts[0],
                PmsA003Readout::Concentration(PmsA003Particle::PM1_0, Standard, 257)
            );
            assert_eq!(
                readouts[1],
                PmsA003Readout::Concentration(PmsA003Particle::PM2_5, Standard, 257)
            );
            assert_eq!(
                readouts[2],
                PmsA003Readout::Concentration(PmsA003Particle::PM10_0, Standard, 257)
            );
            assert_eq!(
                readouts[3],
                PmsA003Readout::Concentration(PmsA003Particle::PM1_0, Environmental, 257)
            );
            assert_eq!(
                readouts[4],
                PmsA003Readout::Concentration(PmsA003Particle::PM2_5, Environmental, 257)
            );
            assert_eq!(
                readouts[5],
                PmsA003Readout::Concentration(PmsA003Particle::PM10_0, Environmental, 257)
            );
            assert_eq!(
                readouts[6],
                PmsA003Readout::Count(PmsA003Particle::PM0_3, 257)
            );
            assert_eq!(
                readouts[7],
                PmsA003Readout::Count(PmsA003Particle::PM0_5, 257)
            );
            assert_eq!(
                readouts[8],
                PmsA003Readout::Count(PmsA003Particle::PM1_0, 257)
            );
            assert_eq!(
                readouts[9],
                PmsA003Readout::Count(PmsA003Particle::PM2_5, 257)
            );
            assert_eq!(
                readouts[10],
                PmsA003Readout::Count(PmsA003Particle::PM5_0, 257)
            );
            assert_eq!(
                readouts[11],
                PmsA003Readout::Count(PmsA003Particle::PM10_0, 257)
            );
        }
    }
}
