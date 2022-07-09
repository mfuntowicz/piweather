use smallvec::SmallVec;
use std::fmt::{Debug, Display};

use crate::{PiWeatherError, Readout};

pub trait Sensor: Display + Debug {
    fn name(&self) -> &'static str;
    fn read(&mut self) -> Result<Option<SmallVec<[Readout; 4]>>, PiWeatherError>;
}
