use crate::Modality;
use std::time::Instant;

#[derive(Debug)]
pub struct Payload<const N: usize> {
    when: Instant,
    readouts: [Modality; N],
}

impl<const N: usize> Payload<N> {
    pub fn new(when: Instant, readouts: [Modality; N]) -> Self {
        Self { when, readouts }
    }

    pub fn now(readouts: [Modality; N]) -> Self {
        Self {
            when: Instant::now(),
            readouts,
        }
    }
}
