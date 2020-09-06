use actix_web::{web, App, HttpServer};

mod route_handler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    const HOST: &str = "127.0.0.1";
    const PORT: i32 = 8088;

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(route_handler::index))
            .route("/again", web::get().to(route_handler::index2))
    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}
