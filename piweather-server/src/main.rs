use actix_web::web::{put, Json};
use actix_web::{App, HttpResponse, HttpServer};
use anyhow::{anyhow, Result};
use piweather_commons::Readout;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "PiWeather",
    about = "Weather Station",
    author = "Morgan Funtowicz <funtowizmo [at] gmail [dot] com>"
)]
struct PiWeatherOpt {
    #[structopt(short, long, default_value = "0.0.0.0")]
    host: String,

    #[structopt(short, long, default_value = "8080")]
    port: u16,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let options: PiWeatherOpt = PiWeatherOpt::from_args_safe().map_err(|err| anyhow!(err))?;

    let _ = HttpServer::new(|| {
        App::new().route(
            "/readouts",
            put().to(|readouts: Json<Vec<Readout>>| async move {
                println!("Received readouts: {:?}", readouts);
                HttpResponse::Ok().await
            }),
        )
    })
    .bind((&*options.host, options.port))?
    .run()
    .await;

    Ok(())
}
