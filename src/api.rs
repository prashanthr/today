
use actix_web::{HttpResponse, Responder, http};
use http::StatusCode;
use std::collections::HashMap;
extern crate reqwest;

async fn make_request (url: &str) -> Result<HashMap<String, String>, reqwest::Error> {
  match reqwest::get(url).await {
    Ok(data) =>  Ok(
      data
      .json::<HashMap<String, String>>()
      .await
      .unwrap()
    ),
    Err(err) => {
      println!("Error occurred when trying to make request to {}: {}", url, err);
      Err(err)
    }
  }
}

pub async fn health() -> impl Responder {
  let status_data: HashMap<&str, &str> = [("status", "healthy")].iter().cloned().collect();
  HttpResponse::Ok().json(status_data)
}

pub async fn quote_of_day() -> impl Responder {
  const QOD_URL: &str = "https://httpbin.org/ip";
  match make_request(QOD_URL).await {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}

pub async fn weather_of_day() -> impl Responder {
  const WOD_URL: &str = "https://httpbin.org/ip";
  match make_request(WOD_URL).await {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}