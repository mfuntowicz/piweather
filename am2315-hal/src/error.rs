use embedded_hal::i2c::ErrorKind as I2CError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Am2315Error {
    #[error("Failed to create Am2315 because bus device has not been provided.")]
    NoDeviceProvided,

    #[error("Caught an error during I2C I/O: {0}.")]
    BusError(I2CError),

    #[error("Data command read from buffer: {:#02x} doesn't match read command: {:#02x}.")]
    InvalidPreamble(u8, u8),

    #[error("Data length read from buffer: {:#02x} doesn't match buffer length: {:#02x}.")]
    MismatchingBufferLength(u8, usize),

    #[error("CRC mismatch {:#04x} | {:#04x}.")]
    MismatchingCrc(u16, u16),
}
