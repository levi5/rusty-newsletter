use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer, middleware::Logger};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
  let db_pool = web::Data::new(db_pool);
  let server = HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .service(health_check)
      .service(subscribe)
      .app_data(db_pool.clone())
  })
  .listen(listener)?
  .run();
  Ok(server)
}
