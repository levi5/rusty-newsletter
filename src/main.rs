use std::net::TcpListener;

use env_logger::Env;
use newsletters::{settings::get_settings, startup::run};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let settings = get_settings().expect("Failed to read settings.");
  let connection_pool = PgPool::connect(&settings.database.connection_string())
    .await
    .expect("Failed to connect to Postgres.");

  let address = format!("127.0.0.1:{}", settings.application_port);
  let listener = TcpListener::bind(address).expect("Failed to bind random port");
  run(listener, connection_pool)?.await
}
