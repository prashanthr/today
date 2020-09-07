
use actix_web::{HttpResponse, Responder, http};
use http::StatusCode;
use std::collections::HashMap;
use std::time::Duration;
extern crate reqwest;
use reqwest::{header, ClientBuilder};

async fn make_request (url: &str) -> Result<HashMap<String, String>, reqwest::Error> {
  let mut headers = header::HeaderMap::new();
  headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));
  let client = ClientBuilder::new()
    .default_headers(headers)
    .user_agent(concat!(
      env!("CARGO_PKG_NAME"),
      "/",
      env!("CARGO_PKG_VERSION")))
    .timeout(Duration::from_secs(10))
    .build()?;
  match client
    .get(url)
    .send()
    .await {
      Ok(data) =>  {
        println!("Rez {:?}", data);
        match data.status().is_success() {
          true => Ok(
            data
            .json::<HashMap<String, String>>()
            .await
            .unwrap()
          ) ,
          false => Ok(HashMap::new())
        }    
      },
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
  const QOD_URL: &str = "https://quotes.rest/qod?language=en";
  match make_request(QOD_URL).await {
    Ok(data) => {
      println!("Inner {:?}", data);
      HttpResponse::Ok()
        .json(data)
    },
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