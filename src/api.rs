
use actix_web::{web, HttpResponse, Responder, http};
use http::StatusCode;
use std::collections::HashMap;
use std::time::Duration;
use std::env;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
use serde_json::{Result};
use reqwest::{header, ClientBuilder};
use serde::{Deserialize, Serialize};

/*
 Makes a HTTP GET request
*/
async fn make_request<T: for<'de> serde::Deserialize<'de>> (url: &str) -> Result<T> {
  let mut headers = header::HeaderMap::new();
  headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));
  let client = ClientBuilder::new()
    .default_headers(headers)
    .user_agent(concat!(
      env!("CARGO_PKG_NAME"),
      "/",
      env!("CARGO_PKG_VERSION")))
    .timeout(Duration::from_secs(10))
    .build().unwrap();
  println!("Making request to {}", url);
  match client
    .get(url)
    .send()
    .await {
      Ok(data) =>  {
        println!("Rez {:?}", data);
        match data.status().is_success() {
          true => Ok(data.json::<T>().await.unwrap()),
          false => panic!("Received non OK response")
        }    
      },
      Err(err) => {
        panic!(format!("Error occurred when trying to make request to {}: {}", url, err))
      }
  }
}

/*
 Fetches the value of any environment variable
*/
pub fn get_env (key: &str, default_value: Option<&str>) -> String {
  let empty: &str = "";
  match env::var(key) {
    Ok(val) => {
      println!("{}: {}", key, val);
      String::from(val)
    },
    Err(err) => {
      println!("Couldn't interpret env {:?}: {}", key, err);
      match default_value {
        Some(val) => String::from(val),
        None => String::from(empty)
      }
    },
  }
}

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
  match make_request::<Ip>(test_url).await {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}

/*
  Quote of day
*/
pub async fn quote_of_day() -> impl Responder {
  #[derive(Serialize, Deserialize, Debug)]
  struct Quote {
    quote: String,
    author: String
  };
  #[derive(Serialize, Deserialize, Debug)]
  struct Contents {
    quotes: Vec<Quote>
  };
  #[derive(Serialize, Deserialize, Debug)]
  struct QOD {
    contents: Contents,
  };
  let qod_url: &str = "http://quotes.rest/qod.json?category=inspire&language=en";
  match make_request::<QOD>(qod_url).await {
    Ok(data) => {
      println!("Inner {:?}", data);
      HttpResponse::Ok()
        .json(data.contents.quotes)
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
  let api_key: String = get_env("WEATHER_API_KEY", None);
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
  match make_request::<WOD>(wod_url).await {
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
  let api_key: String = get_env("NEWS_API_KEY", None);
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
  match make_request::<NOD>(nod_url).await {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(_err) => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}
