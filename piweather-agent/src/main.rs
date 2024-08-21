use clap::Parser;
use piweather_agent::i2c::get_os_i2c_factory;
use piweather_agent::sensors::{Am2315, Sensor};
use piweather_common::errors::PiWeatherError;
use piweather_common::Payload;
use std::path::PathBuf;
use tokio::sync::mpsc::{channel, Receiver};
use tracing::{debug, error, info};
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

async fn weather_readouts_scheduler(mut readouts: Receiver<Payload<3>>) {
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
    info!("Opening I2C bus {}", &args.bus.display());

    // Initiate sensors
    let mut am2315 = Am2315::with_i2c_factory(factory)?;
    let readouts = am2315.read()?;

    info!("Am2315 read: {:?}", readouts);

    // Start the looper
    let (_sender, receiver) = channel(args.backlog);

    // Display the epilogue if any
    if let Err(ref err) = tokio::spawn(weather_readouts_scheduler(receiver)).await {
        error!("Got an error when terminating the application {}", err);
    }

    Ok(())
}
