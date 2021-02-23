extern crate csv;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime,Duration, Utc};
use crate::util;

fn get_default_copy() -> String {
   String::from("Unknown")
}

/* AppCache */

#[derive(Debug, Clone)]
pub struct AppCache {
  pub qod: Option<Vec<Quote>>,
  pub wod: Option<HashMap<String, WOD>>, // "san francisco,ca" -> ...
  pub nod: Option<HashMap<String, NOD>>, // "country" -> ...
  pub hod: Option<HOD>,
  pub sod: Option<Vec<SpotifyChartCsvRecord>>,
  pub qod_dt: Option<DateTime<Utc>>,
  pub wod_dt: Option<DateTime<Utc>>,
  pub nod_dt: Option<DateTime<Utc>>,
  pub hod_dt: Option<DateTime<Utc>>,
  pub sod_dt: Option<DateTime<Utc>>
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
          Some(field_dt) => util::datetime::in_range(field_dt, Duration::hours(2)),
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

  pub fn sod_exists(&self) -> bool {
    let exists = !self.sod.is_none();
    println!("\nSOD cache is {}", if exists { "full"  } else { "empty" });
    match exists {
      true => {
        match self.sod_dt {
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
    println!("SOD: {:?} {:?}", self.sod, self.sod_dt);
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

pub fn get_default_qod() -> QOD {
  const DEFAULT_QOD_AUTHOR: &str = "Aristole";
  const DEFAULT_QOD_QUOTE: &str  = "It is during our darkest moments that we must focus to see the light.";
  QOD {
    contents: Contents {
        quotes: vec![
          Quote { 
            author: String::from(DEFAULT_QOD_AUTHOR), 
            quote: String::from(DEFAULT_QOD_QUOTE)
          }
        ]
      }
  }
}

/* Weather of day */

#[derive(Deserialize)]
pub struct WODRequest {
  pub location: Option<String>,
  pub unit: Option<String>
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

pub fn get_default_wod() -> WOD {
  WOD {
    weather: vec![
      Weather {
        main: get_default_copy(),
        description: get_default_copy(),
        icon: get_default_copy(),
      }
    ],
    main: WeatherMain {
      temp: 0.0,
      feels_like: 0.0,
      temp_min: 0.0,
      temp_max: 0.0,
      pressure: 0.0,
      humidity: 0.0,
    },
    visibility: 0.0,
    wind: WeatherWind {
      speed: 0.0,
      deg: 0.0,
    },
    clouds: WeatherClouds {
      all: 0.0
    },
    sys: WeatherSys {
      country: get_default_copy(),
      sunrise: 0,
      sunset: 0,
    },
    name: get_default_copy(),
  }
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

pub fn get_default_nod() -> NOD {
  NOD {
    articles: vec![
      NewsArticle {
        source: NewsArticleSource {
          id: None,
          name: None
        },
        author: None,
        title: None,
        description: None,
        content: None,
        url: None,
        publishedAt: None
      }
    ]
  }
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

pub fn get_default_hod() -> HOD {
  HOD {
    date: None,
    url: None,
    data: HODData {
      Events: vec![
        HODItem {
          year: None,
          text: None,
          links: vec![HODLink {
            title: None,
            link: None
          }]
        }
      ], 
      Births: vec![
        HODItem {
          year: None,
          text: None,
          links: vec![HODLink {
            title: None,
            link: None
          }]
        }
      ],
      Deaths: vec![
        HODItem {
          year: None,
          text: None,
          links: vec![HODLink {
            title: None,
            link: None
          }]
        }
      ],
    }
  }
}

/* Song of the day API */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SOD {
  pub artist_name: Option<String>,
  pub track_name: Option<String>,
  pub uri: Option<String>,
  pub source: Option<String>
}

// impl From<String> for SOD {
//   fn from(item: String) -> Self {
//       SOD {

//       }
//   }
// }

pub fn get_default_sod() -> SOD {
  SOD {
    artist_name: Some("The Weeknd".to_string()),
    track_name: Some("Blinding Lights".to_string()),
    uri: Some("https://open.spotify.com/track/0VjIjW4GlUZAMYd2vXMi3b".to_string()),
    source: None
  }
}

/* Download options here: https://spotifycharts.com/regional */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpotifyChartCsvRecord {
  pub position: String,
  pub track_name: String,
  pub artist: String,
  pub num_streams: String,
  pub url: String
}

impl From<csv::StringRecord> for SpotifyChartCsvRecord {
  fn from(record: csv::StringRecord) -> Self {
    fn transform(data: Option<&str>) -> String {
       match data {
        Some(d) => d.to_owned(),
        None => "".to_owned()
      }
    }
    SpotifyChartCsvRecord {
      position: transform(record.get(0)),
      track_name: transform(record.get(1)),
      artist: transform(record.get(2)),
      num_streams: transform(record.get(3)),
      url: transform(record.get(4))
    }
  }
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

pub type GenericError = Box<dyn std::error::Error>;
pub type GenericResult<T, E = GenericError> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum HttpVerb {
  HEAD,
  GET,
  POST,
  PUT,
  PATCH,
  DELETE
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum HttpResponseType {
  JSON, // application/json
  TEXT, // text/plain
  CSV // text/csv
}

#[derive(Debug, Clone)]
pub struct HttpRequestParams {
  pub id: Option<String>,
  pub url: String,
  pub method: HttpVerb,
  pub response_type: Option<HttpResponseType>,
  pub query_params: Option<HashMap<String, String>>,
  pub body: Option<String>
}

pub struct RequestSeqWithSuccessFallbackParams<T> {
  pub requests: Vec<HttpRequestParams>,
  pub default_value: T
}
