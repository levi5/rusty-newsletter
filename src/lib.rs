use std::net::TcpListener;
use actix_web::{dev::Server, get, post, App, HttpRequest, HttpResponse, HttpServer, Responder, web};

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}


#[get("/health_check")]
async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();
    Ok(server)
}
