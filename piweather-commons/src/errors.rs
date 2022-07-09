use thiserror::Error;

#[derive(Debug, Error)]
pub enum PiWeatherError {
    #[error("Unable to complete HTTP request (status: {0:?} -> reason: {1:?})")]
    HttpTransmitterError(Option<u16>, Option<String>),
}
