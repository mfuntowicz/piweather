use anyhow::Result;
use flume::{bounded, Receiver, Sender};
use piweather_commons::{PiWeatherError, Readout, Sensor};
use smallvec::SmallVec;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{sleep, yield_now};
use std::time::Instant;
use std::{fmt::Debug, time::Duration};
use tracing::field::debug;
use tracing::{debug, error, info, instrument};

const DEFAULT_CAPACITY: usize = 32;

pub struct PiWeatherSensorPipe {
    sender: Sender<Readout>,
    sensor: Box<dyn Sensor>,
}

impl PiWeatherSensorPipe {
    pub fn new(sensor: Box<dyn Sensor>, sender: Sender<Readout>) -> Self {
        Self { sender, sensor }
    }
}

pub struct PiWeatherEngine {
    interval: Duration,
    sender: Sender<Readout>,
    receiver: Receiver<Readout>,
    terminated: Arc<AtomicBool>,
    sensors: Vec<PiWeatherSensorPipe>,
}

impl PiWeatherEngine {
    pub fn new(interval: Duration, capacity: usize, terminated: Arc<AtomicBool>) -> Self {
        let (sender, receiver) = bounded(capacity);
        Self {
            interval,
            sender,
            receiver,
            terminated,
            sensors: Vec::new(),
        }
    }

    #[instrument]
    pub fn register_sensor<T>(&mut self, sensor: T)
    where
        T: 'static + Sensor + Debug,
    {
        info!("Registering sensor {:?}", sensor);
        self.sensors.push(PiWeatherSensorPipe::new(
            Box::new(sensor),
            self.sender.clone(),
        ));
    }

    pub fn run(&mut self) -> Result<()> {
        info!("Starting PiWeatherEngine");

        while !self.terminated.load(Ordering::Relaxed) {
            debug!("Engine tick");
            let start = Instant::now();

            for mut sensor in &mut self.sensors {
                match sensor.sensor.read() {
                    Ok(readouts) => info!("Reading sensor: {:?}", readouts),
                    Err(err) => error!("Caught error while reading: {}", err),
                }
            }

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
            // .field("sensors", &self.sensors)
            .finish()
    }
}
