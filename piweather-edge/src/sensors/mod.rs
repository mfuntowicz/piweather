use piweather_commons::{PiWeatherError, Readout, Sensor};
use smallvec::SmallVec;
use std::fmt::{Display, Formatter};
use tracing::instrument;

#[derive(Debug)]
pub enum PiWeatherSensor {
    Dummy(DummySensor),
}

impl Display for PiWeatherSensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PiWeatherSensor::Dummy(sensor) => write!(f, "{}", &sensor),
        }
    }
}

impl PiWeatherSensor {
    #[instrument]
    pub(crate) fn read(&mut self) -> Result<Option<SmallVec<[Readout; 4]>>, PiWeatherError> {
        match self {
            PiWeatherSensor::Dummy(sensor) => sensor.read(),
        }
    }
}

impl From<DummySensor> for PiWeatherSensor {
    fn from(sensor: DummySensor) -> Self {
        PiWeatherSensor::Dummy(sensor)
    }
}

#[cfg(feature = "am2315")]
mod am2315;

#[cfg(feature = "am2315")]
pub use am2315::*;

mod dummy;
pub use dummy::*;
