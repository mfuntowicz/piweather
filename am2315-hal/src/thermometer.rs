use crate::{Am2315Error, Temperature};
use std::error::Error;
use std::fmt::{Debug, Display};

pub trait Thermometer {
    type Error: Debug + Display + Error;

    fn temperature(&self) -> Result<Temperature, Self::Error>;
}
