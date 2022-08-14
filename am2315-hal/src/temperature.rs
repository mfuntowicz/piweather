const FAHRENHEIT_TO_CELSIUS_COEFF: f32 = 5. / 9.;
const CELSIUS_TO_FAHRENHEIT_COEFF: f32 = 9. / 5.;

#[derive(Debug, Clone, Copy)]
pub enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
}

impl Temperature {
    #[inline]
    pub fn to_celsius(&self) -> Temperature {
        match self {
            Temperature::Celsius(_) => self.copy(),
            Temperature::Fahrenheit(fahrenheit) => {
                Temperature::Celsius((fahrenheit - 32) * FAHRENHEIT_TO_CELSIUS_COEFF)
            }
        }
    }

    #[inline]
    pub fn to_fahrenheit(&self) -> Temperature {
        match self {
            Temperature::Celsius(celsius) => {
                Temperature::Fahrenheit(celsius * CELSIUS_TO_FAHRENHEIT_COEFF + 32)
            }
            Temperature::Fahrenheit(_) => self.copy(),
        }
    }
}
