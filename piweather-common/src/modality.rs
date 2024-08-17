use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum Wind {
    Kph(u16),
    Mph(u16),
}

/// Represent temperature reading
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
}

impl Temperature {
    pub fn to_celsius(&self) -> Self {
        match self {
            Temperature::Celsius(temp) => Temperature::Celsius(*temp),
            Temperature::Fahrenheit(temp) => Temperature::Celsius((temp - 32.0) / 1.8),
        }
    }

    pub fn to_fahrenheit(&self) -> Self {
        match self {
            Temperature::Celsius(temp) => Temperature::Fahrenheit(temp * 1.8 + 32.0),
            Temperature::Fahrenheit(temp) => Temperature::Fahrenheit(*temp),
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum Modality {
    Pressure(u16),
    Wind(Wind),
    Temperature(Temperature),
}

#[cfg(test)]
mod tests {
    use crate::modality::Temperature;

    #[test]
    fn temperature_celius_to_fahrenheit() {
        assert_eq!(
            Temperature::Fahrenheit(50.0).to_celsius(),
            Temperature::Celsius(10.0)
        );
    }

    #[test]
    fn temperature_fahrenheit_to_celsius() {
        assert_eq!(
            Temperature::Celsius(9.9).to_fahrenheit(),
            Temperature::Fahrenheit(49.82)
        );
    }
}
