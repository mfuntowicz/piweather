use clap::Parser;
use piweather_common::Payload;
use tokio::sync::mpsc::{channel, Receiver};
use tracing::{debug, error, info};
use tracing_subscriber;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    destination: String,

    #[arg(long, default_value = "16")]
    backlog: usize,
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
async fn main() {
    tracing_subscriber::fmt::init();

    // Parse arguments
    let args = Args::parse();

    // Start the looper
    let (_sender, receiver) = channel(args.backlog);

    // Display the epilogue if any
    if let Err(ref err) = tokio::spawn(weather_readouts_scheduler(receiver)).await {
        error!("Got an error when terminating the application {}", err)
    }
}
