use clap::Parser;
use piweather_agent::i2c::get_os_i2c_factory;
use piweather_common::errors::PiWeatherError;
use piweather_common::Payload;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc::{channel, Receiver};
use tracing::{debug, error, info, warn};
use tracing_subscriber;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        long,
        default_value = "16",
        help = "Maximum number of queued readouts before blocking"
    )]
    backlog: usize,

    #[arg(short, long, help = "The I2C device to use")]
    bus: PathBuf,

    #[arg(required = true, help = "URI where to push the readouts")]
    destination: String,
}

async fn weather_readouts_scheduler(mut readouts: Receiver<Payload>) {
    loop {
        match readouts.recv().await {
            Some(payload) => {
                debug!("Received payload: {:?}", payload);
            }
            None => {
                debug!("Received termination from the channel");
                break;
            }
        }
    }

    info!("Scheduler exiting");
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), PiWeatherError> {
    tracing_subscriber::fmt::init();

    // Parse arguments
    let args = Args::parse();

    // Create the I2C bus from the provided file address
    let factory = get_os_i2c_factory(&args.bus)?;
    info!("Reading from I2C bus at {}", &args.bus.display());

    // Start the looper
    let (_sender, receiver) = channel(args.backlog);

    // Display the epilogue if any
    if let Err(ref err) = tokio::spawn(weather_readouts_scheduler(receiver)).await {
        error!("Got an error when terminating the application {}", err);
    }

    Ok(())
}
