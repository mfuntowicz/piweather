use anyhow::{anyhow, Result};
use piweather_edge::{sensors::DummySensor, utils::setup_logging, PiWeatherEngine};
use signal_hook::{consts::SIGINT, flag::register};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;
use structopt::StructOpt;
use tracing::{error, info, instrument};

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "PiWeather", about = "A RaspberryPi Weather Station.")]
pub struct PiWeather {
    #[structopt(short, long)]
    verbose: bool,
}

#[instrument]
fn register_sigterm_hook() -> Result<Arc<AtomicBool>> {
    info!("Registering SIGTERM hook");

    let flag = Arc::new(AtomicBool::new(false));
    match register(SIGINT, flag.clone()) {
        Ok(sigid) => {
            info!("Registered {:?}", sigid);
            Ok(flag)
        }
        Err(err) => {
            error!("Failed to set SIGTERM hook: {}", err);
            Err(anyhow!(err))
        }
    }
}

fn main() -> Result<()> {
    let piweather = PiWeather::from_args_safe()?;

    // Setup logging
    setup_logging(match piweather.verbose {
        true => tracing::Level::DEBUG,
        false => tracing::Level::TRACE,
    });

    info!("Starting PiWeather.");
    let terminated = register_sigterm_hook()?;

    info!("Creating PiWeatherEngine");
    let mut engine = PiWeatherEngine::new(Duration::from_millis(500), terminated);
    engine.register_sensor(DummySensor::new());
    engine.run()?;

    info!("PiWeather is shutting down.");
    Ok(())
}
