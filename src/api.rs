
use actix_web::{web, HttpResponse, Responder, http};
use http::StatusCode;
use std::collections::HashMap;
use std::time::Duration;
use std::env;
extern crate reqwest;
extern crate serde;
use reqwest::{header, ClientBuilder};
use serde::{Deserialize};
// #[macro_use] 
// extern crate serde_derive;

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

// fn get_env (key: &str, defaultValue: &str) -> &str {
//   let empty: &str = "";
//   match env::var(key) {
//     Ok(val) => {
//       println!("{}: {}", key, val);
//       &val
//     },
//     Err(e) => {
//       println!("couldn't interpret {:?}: {}", key, e);
//       if defaultValue.is_empty() {
//         &empty
//       } else {
//         &defaultValue.to_owned()
//       }
//     },
//   }
// }

pub async fn health() -> impl Responder {
  let status_data: HashMap<&str, &str> = [("status", "healthy")].iter().cloned().collect();
  HttpResponse::Ok().json(status_data)
}

pub async fn quote_of_day() -> impl Responder {
  let qod_url: &str = "https://httpbin.org/ip"; //"https://quotes.rest/qod?language=en";
  match make_request(qod_url).await {
    Ok(data) => {
      println!("Inner {:?}", data);
      HttpResponse::Ok()
        .json(data)
    },
    Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}

#[derive(Deserialize)]
pub struct WODRequest {
  location: Option<String>
}

pub async fn weather_of_day(info: web::Query<WODRequest>) -> impl Responder {
  println!("here {:?}", info.location);
  let resolved_location: String = match &info.location {
    None => String::from(""),
    Some(loc) => loc.to_string(),
  };
  let base_url: &str = "https://api.openweathermap.org/data/2.5/weather";
  //let api_key: &str = get_env(String::from("WEATHER_API_KEY"), String::from(""));
  let api_key: &str = "db5a05bd35c42b20c0934a36d0735441";
  let wod_url: &str = &(base_url.to_owned() + "?q=" + &resolved_location.to_owned() + "&appId=" + &api_key.to_owned());
  println!("WOD URL: {}", wod_url);
  //const WOD_URL: &str = format!("{}?q={}&appid={}", base_url, LOCATION, API_KEY);
  match make_request(wod_url).await {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}