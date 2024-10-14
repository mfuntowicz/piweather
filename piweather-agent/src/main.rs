use clap::Parser;
use piweather_agent::i2c::get_os_i2c_factory;
use piweather_agent::sensors::{Am2315, Sensor};
use piweather_common::errors::PiWeatherError;
use piweather_common::Modality;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tracing::{error, info};
use tracing_subscriber;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "The I2C device to use")]
    bus: PathBuf,

    #[arg(
        short,
        long,
        default_value = "10",
        help = "Interval between two sensor readings"
    )]
    frequency: usize,

    #[arg(required = true, help = "URI where to push the readouts")]
    destination: String,
}

// TODO : The need to parameterize over N is a bit too much?
fn weather_readouts_driver(
    bus: PathBuf,
    frequency: Duration,
    pipe: Sender<Modality>,
) -> Result<usize, PiWeatherError> {
    // Initiate sensors
    // Create the I2C bus from the provided file address
    let factory = get_os_i2c_factory(bus)?;
    let mut am2315 = Am2315::with_i2c_factory(factory)?;

    // Count the number of message sent
    let mut messages = 0usize;

    'looper: loop {
        // let when = DateTime::<Utc>::default();
        match am2315.probe() {
            Ok(readouts) => {
                if let Some(readouts) = readouts {
                    info!("{:?}", &readouts);

                    for readout in readouts {
                        if let Err(err) = pipe.send(readout) {
                            error!(
                                device = am2315.name(),
                                error = err.to_string(),
                                "Failed to send through the pipe"
                            );
                            break 'looper;
                        } else {
                            messages += 1;
                        }
                    }
                }
            }
            Err(err) => {
                error!(
                    device = "am2315",
                    error = err.to_string(),
                    "I/O Error on AM2315"
                )
            }
        }
        sleep(frequency);
    }

    info!("Scheduler exiting ({messages} readouts)");
    Ok(messages)
}

fn main() -> Result<(), PiWeatherError> {
    tracing_subscriber::fmt::init();

    // Parse arguments
    let args = Args::parse();
    let boot = Instant::now();

    // Start the looper
    let (sender, _) = channel();

    // Display the epilogue if any
    let frequency = Duration::from_secs(args.frequency as u64);
    let driver =
        thread::spawn(move || weather_readouts_driver(args.bus.clone(), frequency, sender));

    match driver.join() {
        Ok(stats) => {
            let stats = stats?;
            let uptime = Instant::now() - boot;
            info!(
                "Weather station exiting... Uptime {:?}, {} weather data points sent",
                uptime, stats
            )
        }
        Err(err) => error!("Got an error when terminating the application {:?}", err),
    };

    Ok(())
}
