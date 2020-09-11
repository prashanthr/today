use actix_web::{web, App, HttpServer};

mod api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_name: String = String::from("today");
    let host: String = api::get_env("TODAY_API_HOST", Some("127.0.0.1"));
    let port: String = api::get_env("TODAY_API_PORT", Some("8088"));
    
    println!("Running {} server at {}:{}", app_name, host, port);
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(api::health))
            .route("/health", web::get().to(api::health))
            .route("/test", web::get().to(api::test))
            .route("/nod", web::get().to(api::news_of_day))
            .route("/qod", web::get().to(api::quote_of_day))
            .route("/wod", web::get().to(api::weather_of_day))
            // .service(
            //   web::resource("/wod")
            //   .route(web::get())
            //   .to(api::weather_of_day) // <- use `Query` extractor
            // )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
