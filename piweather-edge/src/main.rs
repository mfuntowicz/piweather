use anyhow::{anyhow, Result};
use piweather_edge::transmitters::http::PiWeatherHttpTransmitter;
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

    #[structopt(
        short,
        long,
        help = "Interval between two readouts (in seconds), default = 10min (600s)",
        default_value = "600"
    )]
    interval: usize,
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
        false => tracing::Level::INFO,
    });

    let transmitter = PiWeatherHttpTransmitter::default();

    info!("Starting PiWeather.");
    let terminated = register_sigterm_hook()?;

    info!("Creating PiWeatherEngine");
    let mut engine = PiWeatherEngine::new(
        Duration::from_secs(piweather.interval as u64),
        terminated,
        transmitter,
    );
    engine.register_sensor(DummySensor::new().into());
    engine.run()?;

    info!("PiWeather is shutting down.");
    Ok(())
}
