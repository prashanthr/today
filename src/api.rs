
use actix_web::{web, HttpResponse, Responder, http};
use http::StatusCode;
use std::collections::HashMap;

extern crate serde;
use serde::{Deserialize, Serialize};

use crate::util;

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
pub async fn test() -> impl Responder {
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
pub async fn quote_of_day() -> impl Responder {
  /* Cache */
  #[derive(Serialize, Deserialize, Debug, Clone)]
  struct Quote {
    quote: String,
    author: String
  };
  #[derive(Serialize, Deserialize, Debug, Clone)]
  struct Contents {
    quotes: Vec<Quote>
  };
  #[derive(Serialize, Deserialize, Debug, Clone)]
  struct QOD {
    contents: Contents,
  };

  let mut quote_cache = util::cache::create::<Vec<Quote>>(2);
  util::cache::print::<Vec<Quote>>(quote_cache);
  let mut cache_map: HashMap<String, Vec<Quote>> = HashMap::new();
  let qod_url: &str = "http://quotes.rest/qod.json?category=inspire&language=en";
  match util::http_client::make_request::<QOD>(qod_url).await {
    Ok(data) => {
      println!("Inner {:?}", data);
      //util::cache::put::<Vec<Quote>>(quote_cache, String::from("QOD"), data.contents.quotes);
      //util::cache::print::<Vec<Quote>>(quote_cache);
      // quote_cache.put(String::from("QOD"), data.contents.quotes);
      let result = data.clone();
      let r2 = data.clone();
      cache_map.insert(String::from("QOD"), result.contents.quotes);
      println!("Cache map val {:?}", cache_map.get("QOD"));
      // util::cache::put::<Vec<Quote>>(quote_cache, String::from("QOD"), result.contents.quotes);
      HttpResponse::Ok()
        .json(r2.contents.quotes)
    },
    Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}

#[derive(Deserialize)]
pub struct WODRequest {
  location: Option<String>
}
/*
  Wrather of day
*/
pub async fn weather_of_day(info: web::Query<WODRequest>) -> impl Responder {
  let resolved_location: String = match &info.location {
    None => String::from(""),
    Some(loc) => loc.to_string(),
  };
  let base_url: &str = "https://api.openweathermap.org/data/2.5/weather";
  let api_key: String = util::environment::get_env("WEATHER_API_KEY", None);
  let wod_url: &str = &(base_url.to_owned() + "?q=" + &resolved_location.to_owned() + "&APPID=" + &api_key.to_owned());
  #[derive(Serialize, Deserialize, Debug)]
  struct Weather {
    main: String,
    description: String,
    icon: String
  };
  #[derive(Serialize, Deserialize, Debug)]
  struct WeatherMain {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: f32,
    humidity: f32,
  }
  #[derive(Serialize, Deserialize, Debug)]
  struct WeatherWind {
    speed: f32,
    deg: f32,
  }
  #[derive(Serialize, Deserialize, Debug)]
  struct WeatherClouds {
    all: f32,
  }
  #[derive(Serialize, Deserialize, Debug)]
  struct WeatherSys {
    country: String,
    sunrise: i64,
    sunset: i64,
  }
  #[derive(Serialize, Deserialize, Debug)]
  struct WOD {
    weather: Vec<Weather>,
    main: WeatherMain,
    visibility: f32,
    wind: WeatherWind,
    clouds: WeatherClouds,
    sys: WeatherSys,
    name: String,
  };
  match util::http_client::make_request::<WOD>(wod_url).await {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}
/*
  News of day
*/
pub async fn news_of_day() -> impl Responder {
  let base_url: &str = "http://newsapi.org/v2/top-headlines";
  let country = "us";
  let api_key: String = util::environment::get_env("NEWS_API_KEY", None);
  let nod_url: &str = &(base_url.to_owned() + "?country=" + country + "&apiKey=" + &api_key.to_owned());
  #[derive(Serialize, Deserialize, Debug)]
  struct NewsArticleSource {
    name: String
  }
  #[allow(non_snake_case)]
  #[derive(Serialize, Deserialize, Debug)]
  struct NewsArticle {
    source: NewsArticleSource,
    author: String,
    title: String,
    description: String,
    content: String,
    url: String,
    publishedAt: String,
  }
  #[derive(Serialize, Deserialize, Debug)]
  struct NOD {
    articles: Vec<NewsArticle>
  }
  match util::http_client::make_request::<NOD>(nod_url).await {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}
