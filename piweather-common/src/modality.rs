use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::mem;
use std::mem::MaybeUninit;

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
    /// Convert `Temperature::Fahrenheit` to `Temperature::Celsius.
    /// If the temperature is already expressed in Celsius, the returned value is the same
    ///
    /// ```
    /// use piweather_common::Temperature;
    ///
    /// let ft = Temperature::Fahrenheit(0.0);
    /// let ct = ft.to_celsius();
    /// assert_eq!(ct, Temperature::Celsius(-17.777779))
    /// ```
    pub fn to_celsius(&self) -> Self {
        match self {
            Temperature::Celsius(temp) => Temperature::Celsius(*temp),
            Temperature::Fahrenheit(temp) => Temperature::Celsius((temp - 32.0) / 1.8),
        }
    }

    /// Convert `Temperature::Celsius` to `Temperature::Fahrenheit.
    /// If the temperature is already expressed in Fahrenheit, the returned value is the same
    ///
    /// ```
    /// use piweather_common::Temperature;
    ///
    /// let ct = Temperature::Celsius(0.0);
    /// let ft = ct.to_fahrenheit();
    /// assert_eq!(ft, Temperature::Fahrenheit(32.0))
    /// ```
    pub fn to_fahrenheit(&self) -> Self {
        match self {
            Temperature::Celsius(temp) => Temperature::Fahrenheit(temp * 1.8 + 32.0),
            Temperature::Fahrenheit(temp) => Temperature::Fahrenheit(*temp),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Particle {
    PM0_3,
    PM0_5,
    PM1_0,
    PM2_5,
    PM5_0,
    PM10_0,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum AirQuality {
    // Expressed in μg/m3
    Concentration(Particle, u16),

    // Expressed in number of particles in 0.1L of air
    Count(Particle, u16),
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum Modality {
    Humidity(f32),
    Pressure(u16),
    Temperature(Temperature),
    Wind(Wind),
    AirQuality(AirQuality),
}

#[derive(Debug)]
pub struct ModalityArray<const N: usize>(pub [Modality; N]);
impl<const N: usize> Serialize for ModalityArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(N)?;
        for modality in self.0 {
            seq.serialize_element(&modality)?;
        }

        seq.end()
    }
}

struct ArrayVisitor<const N: usize>(PhantomData<fn() -> Modality>);
impl<'de, const N: usize> Visitor<'de> for ArrayVisitor<N> {
    type Value = ModalityArray<N>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("array of length {}", N))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut data = [const { MaybeUninit::<Modality>::uninit() }; N];
        for i in 0..N {
            match seq.next_element()? {
                Some(val) => data[i].write(val),
                None => return Err(serde::de::Error::invalid_length(N, &self)),
            };
        }

        // This is the recommended way for now https://github.com/rust-lang/rust/issues/62875
        unsafe {
            let ptr = &data as *const _ as *const ModalityArray<N>;
            let modalities = ptr.read();
            mem::forget(data);
            Ok(modalities)
        }
    }
}

impl<'de, const N: usize> Deserialize<'de> for ModalityArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(N, ArrayVisitor::<N>(PhantomData))
    }

    // fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    // where
    //     D: Deserializer<'de>,
    // {
    //     deserializer.deserialize_tuple_structN, ArrayVisitor::<N>(PhantomData))
    // }
}

#[cfg(test)]
mod tests {
    use crate::modality::Temperature;

    #[test]
    fn temperature_celsius_to_celsius() {
        assert_eq!(
            Temperature::Celsius(10.5).to_celsius(),
            Temperature::Celsius(10.5)
        );
        assert_eq!(
            Temperature::Celsius(0.0).to_celsius(),
            Temperature::Celsius(0.0)
        );

        assert_eq!(
            Temperature::Celsius(-19.3).to_celsius(),
            Temperature::Celsius(-19.3)
        );
    }

    #[test]
    fn temperature_fahrenheit_to_fahrenheit() {
        assert_eq!(
            Temperature::Fahrenheit(10.5).to_fahrenheit(),
            Temperature::Fahrenheit(10.5)
        );
        assert_eq!(
            Temperature::Fahrenheit(0.0).to_fahrenheit(),
            Temperature::Fahrenheit(0.0)
        );

        assert_eq!(
            Temperature::Fahrenheit(-19.3).to_fahrenheit(),
            Temperature::Fahrenheit(-19.3)
        );
    }

    #[test]
    fn temperature_celsius_to_fahrenheit() {
        assert_eq!(
            Temperature::Fahrenheit(-40.0).to_celsius(),
            Temperature::Celsius(-40.0),
        );
        assert_eq!(
            Temperature::Fahrenheit(50.0).to_celsius(),
            Temperature::Celsius(10.0)
        );
    }

    #[test]
    fn temperature_fahrenheit_to_celsius() {
        assert_eq!(
            Temperature::Celsius(-40.0).to_fahrenheit(),
            Temperature::Fahrenheit(-40.0)
        );
        assert_eq!(
            Temperature::Celsius(9.9).to_fahrenheit(),
            Temperature::Fahrenheit(49.82)
        );
    }
}
