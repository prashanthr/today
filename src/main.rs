use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use env_logger::Env;
use std::sync::Mutex;

mod api;
mod util;
mod types;

use types::AppCache;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
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
                sod: None,
                qod_dt: None,
                wod_dt: None,
                nod_dt: None,
                hod_dt: None,
                sod_dt: None
            }
        )
    );
    
    println!("Running {} server at {}:{}", app_name, host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .max_age(3600);
        App::new()
            .app_data(app_data.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(api::health))
            .route("/health", web::get().to(api::health))
            .route("/debug", web::get().to(api::debug))
            .route("/nod", web::get().to(api::news_of_day))
            .route("/qod", web::get().to(api::quote_of_day))
            .route("/wod", web::get().to(api::weather_of_day))
            .route("/hod", web::get().to(api::history_of_day))
            .route("/sod", web::get().to(api::song_of_day))
            .route("/today", web::get().to(api::today))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
