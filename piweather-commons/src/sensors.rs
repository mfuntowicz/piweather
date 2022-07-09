use smallvec::SmallVec;

use crate::{PiWeatherError, Readout};

pub trait Sensor {
    fn name(&self) -> &'static str;
    fn read(&mut self) -> Result<Option<SmallVec<[Readout; 4]>>, PiWeatherError>;
}
