use piweather_commons::{PiWeatherError, Readout, ReadoutKind, Sensor};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use smallvec::{smallvec, SmallVec};
use std::fmt::{Debug, Display, Formatter};
use tracing::{debug, instrument};

const SENSOR_NAME: &str = "dummy";

#[derive(Clone)]
pub struct DummySensor {
    rng: ThreadRng,
}

impl DummySensor {
    pub fn new() -> Self {
        Self { rng: thread_rng() }
    }
}

impl Sensor for DummySensor {
    #[inline]
    fn name(&self) -> &'static str {
        SENSOR_NAME
    }

    #[instrument]
    fn read(&mut self) -> Result<Option<SmallVec<[Readout; 4]>>, PiWeatherError> {
        debug!("Reading from DummySensor");
        Ok(Some(smallvec![Readout::now(
            ReadoutKind::Temperature,
            self.rng.gen()
        )]))
    }
}

impl Default for DummySensor {
    #[inline]
    fn default() -> Self {
        DummySensor::new()
    }
}

impl Display for DummySensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Debug for DummySensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DummySensor").finish()
    }
}
