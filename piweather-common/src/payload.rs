use crate::Modality;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    when: DateTime<Utc>,
    readouts: SmallVec<Modality, 2>,
}

impl Payload {
    pub fn new<TZ: TimeZone>(when: DateTime<TZ>, readouts: &[Modality]) -> Self {
        Self {
            when: when.to_utc(),
            readouts: readouts.into(),
        }
    }

    pub fn now(readouts: &[Modality]) -> Self {
        Self::new(DateTime::<Utc>::default(), readouts)
    }
}
