use crate::sensors::PiWeatherSensor;
use crate::transmitters::http::PiWeatherHttpTransmitter;
use anyhow::{anyhow, Result};
use piweather_commons::{Readout, Sensor};
use std::ops::Add;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{sleep, yield_now};
use std::time::Instant;
use std::{fmt::Debug, thread, time::Duration};
use tracing::{debug, error, info, instrument};

pub struct PiWeatherEngine {
    interval: Duration,
    terminated: Arc<AtomicBool>,
    transmitter: PiWeatherHttpTransmitter,
    sensors: Vec<PiWeatherSensor>,
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
    pub fn register_sensor(&mut self, sensor: PiWeatherSensor) {
        info!("Registering sensor {}", sensor);
        self.sensors.push(sensor);
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
                    Ok(readouts) => readouts,
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

            let awake = Instant::now().add(self.interval);
            while Instant::now() <= awake {
                yield_now()
            }
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
