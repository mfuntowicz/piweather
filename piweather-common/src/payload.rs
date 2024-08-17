use crate::Modality;
use smallvec::SmallVec;
use std::time::Instant;

#[derive(Debug)]
pub struct Payload {
    when: Instant,
    readouts: SmallVec<Modality, 3>,
}

impl Payload {
    pub fn new(when: Instant, readouts: SmallVec<Modality, 3>) -> Self {
        Self { when, readouts }
    }

    pub fn now(readouts: SmallVec<Modality, 3>) -> Self {
        Self {
            when: Instant::now(),
            readouts,
        }
    }
}
