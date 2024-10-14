use crate::Modality;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    when: DateTime<Utc>,
    readout: Modality,
}

impl Payload {
    pub fn new<TZ: TimeZone>(when: DateTime<TZ>, readout: Modality) -> Self {
        Self {
            when: when.to_utc(),
            readout,
        }
    }

    pub fn now(readout: Modality) -> Self {
        Self::new(DateTime::<Utc>::default(), readout)
    }
}
