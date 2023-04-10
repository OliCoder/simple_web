use actix_web::{get, HttpServer, Responder, HttpResponse, App};
use log::{info, debug};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = simple_web::parse().expect("Parse Arguments Failed");
    config.init_log();
    info!("{}", serde_json::to_string_pretty(&config).expect("print config failed"));
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", config.server.port))?
    .run()
    .await
}
