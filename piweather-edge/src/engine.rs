use crate::transmitters::http::PiWeatherHttpTransmitter;
use anyhow::{anyhow, Result};
use piweather_commons::{Readout, Sensor};
use smallvec::SmallVec;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Instant;
use std::{fmt::Debug, thread, time::Duration};
use tracing::{debug, error, info, instrument};

pub struct PiWeatherEngine {
    interval: Duration,
    terminated: Arc<AtomicBool>,
    transmitter: PiWeatherHttpTransmitter,
    sensors: Vec<Box<dyn Sensor>>,
}

impl PiWeatherEngine {
    pub fn new(
        interval: Duration,
        terminated: Arc<AtomicBool>,
        transmitter: PiWeatherHttpTransmitter,
    ) -> Self {
        Self {
            interval,
            terminated,
            transmitter,
            sensors: Vec::new(),
        }
    }

    #[instrument]
    pub fn register_sensor<T>(&mut self, sensor: T)
    where
        T: 'static + Sensor + Debug,
    {
        info!("Registering sensor {:?}", sensor);
        self.sensors.push(Box::new(sensor));
    }

    pub fn run(&mut self) -> Result<()> {
        info!("Starting PiWeatherEngine");

        while !self.terminated.load(Ordering::Relaxed) {
            debug!("Engine tick");
            let start = Instant::now();

            let readouts: Vec<Readout> = self
                .sensors
                .iter_mut()
                .filter_map(|sensor| match sensor.read() {
                    Ok(readouts) => {
                        info!("Read sensor {}: {:?}", sensor, readouts);
                        readouts
                    }
                    Err(err) => {
                        error!("Caught error while reading sensor {}: {}", sensor, err);
                        None
                    }
                })
                .flatten()
                .collect();

            let _ = self
                .transmitter
                .send(&readouts)
                .map_err(|err| anyhow!(err))?;

            let duration = Instant::now() - start;
            debug!("Engine about to sleep (loop: {:?})", duration);
            sleep(self.interval);
        }

        info!("Exiting PiWeatherEngine");
        Ok(())
    }
}

impl Debug for PiWeatherEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PiWeatherEngine")
            .field("interval", &self.interval)
            .field("sensors", &self.sensors)
            .finish()
    }
}
