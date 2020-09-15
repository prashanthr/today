use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime,Duration, Utc};
use crate::util;

/* AppCache */

#[derive(Debug, Clone, Copy)]
pub struct AppCacheDT {
  pub qod: Option<DateTime<Utc>>,
  pub wod: Option<DateTime<Utc>>,
  pub nod: Option<DateTime<Utc>>
}

#[derive(Debug, Clone)]
pub struct AppCache {
  pub qod: Option<Vec<Quote>>,
  pub wod: Option<HashMap<String, WOD>>, // "san francisco,ca" -> ...
  pub nod: Option<HashMap<String, NOD>>, // "country" -> ...
  pub datetime: Option<AppCacheDT>
}

impl AppCache {
  pub fn qod_exists(&self) -> bool {
    let exists = !self.qod.is_none();
    println!("QOD cache is {}", if exists { "full"  } else { "empty" });
    match exists {
      true => {
        match &self.datetime {
          Some(dt) => match dt.qod {
            Some(field_dt) => util::datetime::in_range(field_dt, Duration::hours(6)),
            None => exists,
          },
          None => exists
        }
      },
      false => exists
    }
  }
  
  pub fn wod_exists(&self, location: String) -> bool {
    let exists = !self.wod.is_none() && !self.wod.as_ref().unwrap().get(&location).is_none();
    println!("WOD cache for lookup {} is {}", location, if exists { "full"  } else { "empty" });
    match exists {
      true => {
        match &self.datetime {
          Some(dt) => match dt.wod {
            Some(field_dt) => util::datetime::in_range(field_dt, Duration::hours(2)),
            None => exists,
          },
          None => exists
        }
      },
      false => exists
    }
  }
  pub fn nod_exists(&self, country: String) -> bool {
    let exists = !self.nod.is_none() && !self.nod.as_ref().unwrap().get(&country).is_none();
    println!("NOD cache for lookup {} is {}", country, if exists { "full"  } else { "empty" });
    match exists {
      true => {
        match &self.datetime {
          Some(dt) => match dt.nod {
            Some(field_dt) => util::datetime::in_range(field_dt, Duration::hours(12)),
            None => exists,
          },
          None => exists
        }
      },
      false => exists
    }
  }
  pub fn print(&self) {
    println!("--AppCache data--");
    println!("QOD: {:?}", self.qod);
    println!("WOD: {:?}", self.wod);
    println!("NOD: {:?}", self.nod);
    println!("DT: {:?}", self.datetime);
    println!("-----------------\n");
  }
}

/* Quote of day */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quote {
  pub quote: String,
  pub author: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contents {
  pub quotes: Vec<Quote>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QOD {
  pub contents: Contents,
}

/* Weather of day */

#[derive(Deserialize)]
pub struct WODRequest {
  pub location: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weather {
  pub main: String,
  pub description: String,
  pub icon: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherMain {
  pub temp: f32,
  pub feels_like: f32,
  pub temp_min: f32,
  pub temp_max: f32,
  pub pressure: f32,
  pub humidity: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherWind {
  pub speed: f32,
  pub deg: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherClouds {
  pub all: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherSys {
  pub country: String,
  pub sunrise: i64,
  pub sunset: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WOD {
  pub weather: Vec<Weather>,
  pub main: WeatherMain,
  pub visibility: f32,
  pub wind: WeatherWind,
  pub clouds: WeatherClouds,
  pub sys: WeatherSys,
  pub name: String,
}

/* News of day */

#[derive(Deserialize)]
pub struct NODRequest {
  pub country: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewsArticleSource {
  pub id: Option<String>,
  pub name: Option<String>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewsArticle {
  pub source: NewsArticleSource,
  pub author: Option<String>,
  pub title: Option<String>,
  pub description: Option<String>,
  pub content: Option<String>,
  pub url: Option<String>,
  pub publishedAt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NOD {
  pub articles: Vec<NewsArticle>
}
