use anyhow::{anyhow, Result};
use piweather_commons::{PiWeatherError, Readout};
use reqwest::blocking::Client;
use reqwest::{StatusCode, Url};
use std::fmt::{Debug, Formatter};
use tracing::instrument;

pub struct PiWeatherHttpTransmitter {
    remote: Url,
    client: Client,
}

impl PiWeatherHttpTransmitter {
    pub fn new(host: &str, port: u16) -> Result<Self> {
        let url = Url::parse(&format!("http://{}:{}/readouts", host, port))
            .map_err(|err| anyhow!(err))?;

        Ok(PiWeatherHttpTransmitter {
            remote: url,
            client: Client::new(),
        })
    }

    #[inline]
    pub fn remote(&self) -> &Url {
        &self.remote
    }

    #[instrument]
    pub fn send(&self, readouts: &[Readout]) -> Result<(), PiWeatherError> {
        let response = self
            .client
            .put(self.remote.clone())
            .json(readouts)
            .send()
            .map_err(|err| {
                PiWeatherError::HttpTransmitterError(
                    err.status().map_or(Some(0), |status| Some(status.as_u16())),
                    Some(err.to_string()),
                )
            })?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(PiWeatherError::HttpTransmitterError(
                Some(response.status().as_u16()),
                None,
            ))
        }
    }
}

impl Default for PiWeatherHttpTransmitter {
    #[inline]
    fn default() -> Self {
        PiWeatherHttpTransmitter::new("localhost", 8080).unwrap()
    }
}

impl Debug for PiWeatherHttpTransmitter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PiWeatherHttpTransmitter")
            .field("remote", &self.remote)
            .finish()
    }
}
