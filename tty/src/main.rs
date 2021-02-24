
#[macro_use]
extern crate log;
#[macro_use]
extern crate simple_error;
#[macro_use]
extern crate actix_web;

use clap::{load_yaml, App};
use actix_web::{HttpServer, middleware, App as WebApp, HttpResponse};

use actix_files::Files;

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("true").await.unwrap()
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("../../clap/tty.yaml");
    let matches = App::from(yaml).get_matches();
    setup_log(matches.occurrences_of("verbose"));

    if let Err(e) = dotenv::dotenv() {
        error!(".env file is not read in completely {:?}", e);
    }
    let bind_addr = std::env::var("ADDRESS").unwrap_or_else(|_| matches.value_of("ADDRESS").map(|s| s.to_string()).unwrap_or("0.0.0.0:8000".to_string()));
    let static_folder = std::env::var("STATIC").unwrap_or_else(|_| matches.value_of("STATIC").unwrap_or_else(|| "./chart/www").to_string());


    HttpServer::new(move || {
        WebApp::new()
            .wrap(middleware::Logger::default())
            // .app_data(client.clone())
            .service(health)
            .service(Files::new("/", &static_folder).index_file("index.html"))

    })
        .bind(bind_addr)?
        .run()
        .await?;

    Ok(())
}

pub const DEFAULT_FILTER_ENV: &str = "LOG_LEVEL";

pub fn setup_log(verbose: u64) {
    let default_filter = match verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    let env = env_logger::Env::default().filter_or(DEFAULT_FILTER_ENV, default_filter);

    env_logger::Builder::from_env(env).init();
}

