use actix_web::{web, App, HttpServer};

mod api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    const HOST: &str = "127.0.0.1";
    const PORT: i32 = 8088;
    
    // #[derive(Deserialize)]
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(api::health))
            .route("/health", web::get().to(api::health))
            .route("/test", web::get().to(api::test))
            .route("/qod", web::get().to(api::quote_of_day))
            .route("/wod", web::get().to(api::weather_of_day))
            // .service(
            //   web::resource("/wod")
            //   .route(web::get())
            //   .to(api::weather_of_day) // <- use `Query` extractor
            // )
    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}
