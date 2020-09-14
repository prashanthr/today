
use actix_web::{web, HttpResponse, Responder, http};
use std::sync::Mutex;
use http::StatusCode;
use std::collections::HashMap;

extern crate serde;
use serde::{Deserialize, Serialize};

use crate::util;
use crate::types::{AppCache, QOD, WODRequest, WOD, NODRequest, NOD};

/* Route Handlers */

/*
 Health Check
*/
pub async fn health() -> impl Responder {
  let status_data: HashMap<&str, &str> = [("status", "healthy")].iter().cloned().collect();
  HttpResponse::Ok().json(status_data)
}

/*
  Test Route
*/
pub async fn test(data: web::Data<Mutex<AppCache>>) -> impl Responder {
  let app_cache = data.lock().unwrap();
  app_cache.print();
  #[derive(Serialize, Deserialize, Debug)]
  struct Ip {
    origin: String,
  };
  let test_url: &str = "https://httpbin.org/ip";
  match util::http_client::make_request::<Ip>(test_url).await {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}

/*
  Quote of day
*/
pub async fn quote_of_day(data: web::Data<Mutex<AppCache>>) -> impl Responder {
  let mut app_cache = data.lock().unwrap();
  let qod_url: &str = "http://quotes.rest/qod.json?category=inspire&language=en";
  if app_cache.qod_exists() {
    HttpResponse::Ok()
          .json(app_cache.qod.as_ref())
  } else {
    match util::http_client::make_request::<QOD>(qod_url).await {
      Ok(data) => {
        app_cache.qod = Some(data.clone().contents.quotes);
        HttpResponse::Ok()
          .json(data.contents.quotes)
      },
      Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
    }
  }
}

/*
  Wrather of day
*/
pub async fn weather_of_day(data: web::Data<Mutex<AppCache>>, info: web::Query<WODRequest>) -> impl Responder {
  let mut app_cache = data.lock().unwrap();
  let resolved_location: String = match &info.location {
    None => String::from("san francisco,usa"),
    Some(loc) => loc.to_string(),
  };
  let resolved_location_cache = resolved_location.clone();
  let base_url: &str = "https://api.openweathermap.org/data/2.5/weather";
  let api_key: String = util::environment::get_env("WEATHER_API_KEY", None);
  let wod_url: &str = &(base_url.to_owned() + "?q=" + &resolved_location.to_owned() + "&APPID=" + &api_key.to_owned());
  
  if app_cache.wod_exists(resolved_location) {
    HttpResponse::Ok()
          .json(app_cache.wod.as_ref().unwrap().get(&resolved_location_cache))
  } else {
    match util::http_client::make_request::<WOD>(wod_url).await {
      Ok(data) => {
        let mut new_cache: HashMap<String, WOD> = 
          if !app_cache.wod.as_ref().is_none() {
            app_cache.wod.clone().unwrap()
          } else {
            HashMap::new()
          };
        new_cache.insert(resolved_location_cache.clone(), data.clone());
        app_cache.wod = Some(
          new_cache.clone()
        );
        HttpResponse::Ok().json(data)
      },
      Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
    }
  }
}
/*
  News of day
*/
pub async fn news_of_day(data: web::Data<Mutex<AppCache>>, info: web::Query<NODRequest>) -> impl Responder {
  let mut app_cache = data.lock().unwrap();
  let base_url: &str = "http://newsapi.org/v2/top-headlines";
  let resolved_country_code: String = match &info.country {
    None => String::from("us"),
    Some(country) => country.to_string(),
  };
  let resolved_country_code_cache = resolved_country_code.clone();
  let api_key: String = util::environment::get_env("NEWS_API_KEY", None);
  let nod_url: &str = &(base_url.to_owned() + "?country=" + &resolved_country_code.to_owned() + "&apiKey=" + &api_key.to_owned());
  
  if app_cache.nod_exists(resolved_country_code) {
    HttpResponse::Ok().json(
      app_cache.nod.as_ref().unwrap().get(&resolved_country_code_cache)
    )
  } else {
    match util::http_client::make_request::<NOD>(nod_url).await {
      Ok(data) => {
        let mut new_cache: HashMap<String, NOD> = 
          if !app_cache.nod.as_ref().is_none() {
            app_cache.nod.clone().unwrap()
          } else {
            HashMap::new()
          };
        new_cache.insert(resolved_country_code_cache.clone(), data.clone());
        app_cache.nod = Some(
          new_cache.clone()
        );
        HttpResponse::Ok().json(data)
      },
      Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
    }
  }
}
