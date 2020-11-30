use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime,Duration, Utc};
use crate::util;

/* AppCache */

#[derive(Debug, Clone)]
pub struct AppCache {
  pub qod: Option<Vec<Quote>>,
  pub wod: Option<HashMap<String, WOD>>, // "san francisco,ca" -> ...
  pub nod: Option<HashMap<String, NOD>>, // "country" -> ...
  pub hod: Option<HOD>,
  pub qod_dt: Option<DateTime<Utc>>,
  pub wod_dt: Option<DateTime<Utc>>,
  pub nod_dt: Option<DateTime<Utc>>,
  pub hod_dt: Option<DateTime<Utc>>
}

impl AppCache {
  pub fn qod_exists(&self) -> bool {
    let exists = !self.qod.is_none();
    println!("\nQOD cache is {}", if exists { "full"  } else { "empty" });
    match exists {
      true => {
        match self.qod_dt {
          Some(field_dt) => util::datetime::in_range(field_dt, Duration::hours(12)),
          None => exists,
        }
      },
      false => exists
    }
  }
  
  pub fn wod_exists(&self, key: String) -> bool {
    let exists = !self.wod.is_none() && !self.wod.as_ref().unwrap().get(&key).is_none();
    println!("\nWOD cache for lookup {} is {}", key, if exists { "full"  } else { "empty" });
    match exists {
      true => {
        match self.wod_dt {
          Some(field_dt) => util::datetime::in_range(field_dt, Duration::hours(3)),
          None => exists,
        }
      },
      false => exists
    }
  }
  pub fn nod_exists(&self, key: String) -> bool {
    let exists = !self.nod.is_none() && !self.nod.as_ref().unwrap().get(&key).is_none();
    println!("\nNOD cache for lookup {} is {}", key, if exists { "full"  } else { "empty" });
    match exists {
      true => {
        match self.nod_dt {
          Some(field_dt) => util::datetime::in_range(field_dt, Duration::hours(6)),
          None => exists,
        }
      },
      false => exists
    }
  }

  pub fn hod_exists(&self) -> bool {
    let exists = !self.hod.is_none();
    println!("\nHOD cache is {}", if exists { "full"  } else { "empty" });
    match exists {
      true => {
        match self.hod_dt {
          Some(field_dt) => util::datetime::in_range(field_dt, Duration::days(1)),
          None => exists,
        }
      },
      false => exists
    }
  }

  pub fn print(&self) {
    println!("\n-----AppCache data-----");
    println!("QOD: {:?} {:?}", self.qod, self.qod_dt);
    println!("WOD: {:?} {:?}", self.wod, self.wod_dt);
    println!("NOD: {:?} {:?}", self.nod, self.nod_dt);
    println!("HOD: {:?} {:?}", self.hod, self.hod_dt);
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
  pub location: Option<String>,
  pub unit: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weather {
  pub main: Option<String>,
  pub description: Option<String>,
  pub icon: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherMain {
  pub temp: Option<f32>,
  pub feels_like: Option<f32>,
  pub temp_min: Option<f32>,
  pub temp_max: Option<f32>,
  pub pressure: Option<f32>,
  pub humidity: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherWind {
  pub speed: Option<f32>,
  pub deg: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherClouds {
  pub all: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherSys {
  pub country: Option<String>,
  pub sunrise: Option<i64>,
  pub sunset: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WOD {
  pub weather: Option<Vec<Weather>>,
  pub main: Option<WeatherMain>,
  pub visibility: Option<f32>,
  pub wind: Option<WeatherWind>,
  pub clouds: Option<WeatherClouds>,
  pub sys: Option<WeatherSys>,
  pub name: Option<String>,
}

/* News of day */

#[derive(Deserialize)]
pub struct NODRequest {
  pub country: Option<String>,
  pub limit: Option<u32>,
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

/* History of day */

#[derive(Deserialize)]
pub struct HODRequest {
  pub limit: Option<u32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HODLink {
  title: Option<String>,
  link: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HODItem {
  pub year: Option<String>,
  pub text: Option<String>,
  pub links: Vec<HODLink>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HODData {
  pub Events: Vec<HODItem>,
  pub Births: Vec<HODItem>,
  pub Deaths: Vec<HODItem>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HOD {
  pub date: Option<String>,
  pub url: Option<String>,
  pub data: HODData
}

/* Today Unified API  */
#[derive(Deserialize)]
pub struct TodayRequest {
  pub location: Option<String>,
  pub country: Option<String>,
  pub wod_unit: Option<String>,
  pub nod_limit: Option<u32>,
  pub hod_limit: Option<u32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodayResponse {
  pub qod: Option<Vec<Quote>>,
  pub wod: Option<WOD>,
  pub nod: Option<NOD>,
  pub hod: Option<HOD>
}
