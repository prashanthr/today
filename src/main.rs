use actix_web::{web, App, HttpServer, http};
use actix_cors::Cors;
use std::sync::Mutex;

mod api;
mod util;
mod types;

use types::AppCache;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_name: String = String::from("today");
    let host: String = util::environment::get_env("TODAY_API_HOST", Some("0.0.0.0"));
    let port: String =  util::environment::get_env("TODAY_API_PORT", Some("9000"));

    // AppCache shared data
    let app_data = web::Data::new(
        Mutex::new(
            AppCache { 
                qod: None,
                wod: None,
                nod: None,
                hod: None,
                qod_dt: None,
                wod_dt: None,
                nod_dt: None,
                hod_dt: None
            }
        )
    );
    
    println!("Running {} server at {}:{}", app_name, host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        App::new()
            .app_data(app_data.clone())
            .wrap(cors)
            .route("/", web::get().to(api::health))
            .route("/health", web::get().to(api::health))
            .route("/debug", web::get().to(api::debug))
            .route("/nod", web::get().to(api::news_of_day))
            .route("/qod", web::get().to(api::quote_of_day))
            .route("/wod", web::get().to(api::weather_of_day))
            .route("/hod", web::get().to(api::history_of_day))
            .route("/today", web::get().to(api::today))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
