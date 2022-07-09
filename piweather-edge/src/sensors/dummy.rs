use piweather_commons::{PiWeatherError, Readout, ReadoutKind, Sensor};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use smallvec::{smallvec, SmallVec};
use std::time::Instant;
use tracing::{debug, instrument};

#[derive(Debug, Clone)]
pub struct DummySensor {
    rng: ThreadRng,
}

impl DummySensor {
    pub fn new() -> Self {
        Self { rng: thread_rng() }
    }
}

impl Sensor for DummySensor {
    #[instrument]
    fn read(&mut self) -> Result<Option<SmallVec<[Readout; 4]>>, PiWeatherError> {
        debug!("Reading from DummySensor");
        Ok(Some(smallvec![Readout::now(
            ReadoutKind::Temperature,
            self.rng.gen()
        )]))
    }
}
