use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ReadoutKind {
    Humidity,
    Temperature,
    Pressure,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Readout {
    when: DateTime<Utc>,
    kind: ReadoutKind,
    value: f32,
}

impl Readout {
    pub fn now(kind: ReadoutKind, value: f32) -> Self {
        Self {
            when: Utc::now(),
            kind,
            value,
        }
    }
}
