use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    destination: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
