use crate::{Am2315Error, Temperature, Thermometer};
use embedded_hal::i2c::{blocking::I2c, ErrorKind};
use std::thread::sleep;
use std::time::Duration;
use tracing::{debug, instrument};

const DEFAULT_AWAKE_WAIT: Duration = Duration::from_millis(10);
const DEFAULT_READ_WAIT: Duration = Duration::from_millis(2);
const DEFAULT_DEVICE_ADDRESS: u8 = 0x5c;

const REGISTER_HUMIDITY_OFFSET: u8 = 0x00;
const REGISTER_TEMPERATURE_OFFSET: u8 = 0x02;

const REGISTER_CMD_SENSOR_WAKE_UP: [u8; 1] = [0u8];
const REGISTER_CMD_SENSOR_READ: [u8; 1] = [0x03];

#[inline]
fn crc16(&buffer: &[u8]) -> u16 {
    let mut crc = 0xFFFFu16;

    for b in buffer {
        crc ^= (*b as u16);
        for _ in 0..8 {
            if (crc & 0x0001) == 0x0001 {
                crc >>= 1;
                crc ^= 0xA001;
            } else {
                crc >>= 1
            }
        }
    }

    crc
}

#[inline]
fn check_preamble(buffer: &[u8; 6]) -> Result<(), Am2315Error> {
    if buffer[0] != REGISTER_CMD_SENSOR_READ {
        return Err(Am2315Error::InvalidPreamble(
            output[0],
            REGISTER_CMD_SENSOR_READ[0],
        ));
    }

    if buffer[1] != output.len() {
        return Err(Am2315Error::MismatchingBufferLength(
            output[1],
            output.len(),
        ));
    }

    Ok(())
}

#[inline]
fn check_crc(buffer: &[u8; 6]) -> Result<(), Am2315Error> {
    let reference_crc = u16::from_ne_bytes([buffer[4], buffer[5]]);
    let computed_crc = crc16(&buffer[0..6]);

    if reference_crc != computed_crc {
        return Err(Am2315Error::MismatchingCrc(reference_crc, computed_crc));
    }

    Ok(())
}

#[derive(Debug)]
pub struct Am2315<I>
where
    I: I2c,
{
    i2c: I,
    address: u8,
}

impl<I> Am2315<I>
where
    I: I2c,
{
    fn read_register(&mut self, buffer: &mut [u8; 6], offset: u8) -> Result<(), Am2315Error> {
        debug!("Initiating reading temperature, waking up device.");
        self.i2c.write(self.address, &REGISTER_CMD_SENSOR_WAKE_UP)?;
        sleep(DEFAULT_AWAKE_WAIT);

        debug!(
            "Sending register read ({:#02x}) command.",
            *REGISTER_CMD_SENSOR_READ[0]
        );
        self.i2c.write(self.address, &REGISTER_CMD_SENSOR_READ)?;
        sleep(DEFAULT_READ_WAIT);

        debug!("Reading temperature register ({:#02x})", offset);
        self.i2c.read(offset, buffer.as_slice_mut())?;

        check_preamble(&output)?;
        check_crc(&output)?;

        Ok(())
    }
}

impl<I> Thermometer for Am2315<I>
where
    I: I2c,
{
    type Error = Am2315Error;

    #[instrument]
    fn temperature(&mut self) -> Result<Temperature, Self::Error> {
        let mut readout = [0u8; 6];
        self.read_register(&mut readout, REGISTER_TEMPERATURE_OFFSET)?;

        let mut raw_value = i16::from_ne_bytes([readout[0], readout[1]]);
        let temperature = raw_value as f32 / 10.0f32;

        Ok(Temperature::Celsius(temperature))
    }
}

pub struct Am2315Builder<I>
where
    I: I2c,
{
    device: Option<I>,
    address: Option<u8>,
}

impl<I> Am2315Builder<I>
where
    I: I2c,
{
    pub fn with_device(&mut self, device: I) -> &mut self {
        self.device = Some(device);
        self
    }

    pub fn with_address(&mut self, address: u8) -> &mut self {
        self.address = Some(address);
        self
    }
}

impl<I> TryFrom<Am2315Builder<I>> for Am2315<I>
where
    I: I2c,
{
    type Error = Am2315Error;

    fn try_from(builder: Am2315Builder<I>) -> Result<Self, Self::Error> {
        if let Some(device) = builder.device {
            Ok(Am2315 {
                i2c: device,
                address: builder.address.unwrap_or(DEFAULT_DEVICE_ADDRESS),
            })
        } else {
            Err(Am2315Error::NoDeviceProvided)
        }
    }
}
