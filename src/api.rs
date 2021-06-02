
use actix_web::{web, HttpResponse, Responder, http};
use std::sync::Mutex;
use http::StatusCode;
use std::collections::HashMap;
use crate::util;
use crate::types::{
  AppCache, 
  QOD, Quote, get_default_qod,
  WODRequest, WOD, get_default_wod,
  NODRequest, NOD, get_default_nod,
  HODRequest, HOD, get_default_hod,
  SOD,
  TodayRequest, TodayResponse,
  HttpRequestParams, HttpVerb, 
  RequestSeqWithSuccessFallbackParams
};

/* Route Handlers */

/*
 Health Check
*/
pub async fn health() -> impl Responder {
  let status_data: HashMap<&str, &str> = [("status", "healthy")].iter().cloned().collect();
  HttpResponse::Ok().json(status_data)
}

/*
  Debug/Test Route
*/
pub async fn debug(data: web::Data<Mutex<AppCache>>) -> impl Responder {
  let app_cache = data.lock().unwrap();
  app_cache.print();
  HttpResponse::Ok()
    .json::<HashMap<&str, &str>>(
      [("debug", "ok")].iter().cloned().collect()
    )
}

/*
  Quote of day
*/
pub async fn get_qod(data: web::Data<Mutex<AppCache>>) -> Option<Vec<Quote>> {
  let mut app_cache = data.lock().unwrap();
  let qod_url: &str = "http://quotes.rest/qod.json?category=inspire&language=en";
  if app_cache.qod_exists() {
    app_cache.qod.clone()
  } else {
    match util::http_client::make_request_with_fallback::<QOD>(qod_url, get_default_qod()).await {
      Ok(data) => {
        app_cache.qod = Some(data.clone().contents.quotes);
        app_cache.qod_dt = Some(util::datetime::now());
        Some(data.contents.quotes)
      },
      Err(_err) => None
    }
  }
}

pub async fn quote_of_day(data: web::Data<Mutex<AppCache>>) -> impl Responder {
  match get_qod(data).await {
    Some(result) => HttpResponse::Ok().json(result),
    None => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}

/*
  Weather of day
*/

pub async fn get_wod(data: web::Data<Mutex<AppCache>>, params: WODRequest) -> Option<WOD> {
  let mut app_cache = data.lock().unwrap();
  let resolved_location: String = match &params.location {
    None => String::from("san francisco,usa"),
    Some(loc) => loc.to_string(),
  };
  let unit = match params.unit {
    None => String::from("metric"),
    Some(u) => u.to_string()
  };
  let cache_key = util::hash::compute_hash(resolved_location.clone() + &unit.clone());
  let get_cache_key = cache_key.clone();
  let set_cache_key = cache_key.clone();
  let base_url: &str = "https://api.openweathermap.org/data/2.5/weather";
  let api_key: String = util::environment::get_env("TODAY_WEATHER_API_KEY", None);
  let wod_url: &str = &(base_url.to_owned() + "?q=" + &resolved_location.to_owned() + "&units=" + &unit.to_owned() + "&APPID=" + &api_key.to_owned());
  
  if app_cache.wod_exists(cache_key) {
    Some(
      app_cache
        .wod.as_ref().unwrap().get(&get_cache_key)
        .unwrap().clone()
    )
  } else {
    match util::http_client::make_request_with_fallback::<WOD>(wod_url, get_default_wod()).await {
      Ok(data) => {
        let mut mut_data = data.clone();
        let mut new_cache: HashMap<String, WOD> = 
          if !app_cache.wod.as_ref().is_none() {
            app_cache.wod.clone().unwrap()
          } else {
            HashMap::new()
          };
        // Update icon url
        mut_data.weather[0].icon = String::from(format!("https://openweathermap.org/img/wn/{}.png", mut_data.weather[0].icon));
        new_cache.insert(set_cache_key.clone(), mut_data.clone());
        app_cache.wod = Some(
          new_cache.clone()
        );
        app_cache.wod_dt = Some(util::datetime::now());
        Some(mut_data)
      },
      Err(_err) => None
    }
  }
}

pub async fn weather_of_day(data: web::Data<Mutex<AppCache>>, info: web::Query<WODRequest>) -> impl Responder {
  match get_wod(data, WODRequest { location: info.location.to_owned(), unit: info.unit.to_owned() }).await {
    Some(result) => HttpResponse::Ok().json(result),
    None => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}
/*
  News of day
*/

pub async fn get_nod(data: web::Data<Mutex<AppCache>>, params: NODRequest) -> Option<NOD> {
  let mut app_cache = data.lock().unwrap();
  let base_url: &str = "http://newsapi.org/v2/top-headlines";
  let resolved_country_code: String = match &params.country {
    None => util::country::get_country_code("America"),
    Some(country) => util::country::get_country_code(country),
  };
  let cache_key = util::hash::compute_hash(resolved_country_code.clone());
  let get_cache_key = cache_key.clone();
  let set_cache_key = cache_key.clone();
  let api_key: String = util::environment::get_env("TODAY_NEWS_API_KEY", None);
  let nod_url: &str = &(base_url.to_owned() + "?country=" + &resolved_country_code.to_owned() + "&pageSize=100" + "&apiKey=" + &api_key.to_owned());
  
  let result = if app_cache.nod_exists(cache_key) {
    Some(
      app_cache.nod
      .as_ref().unwrap().get(&get_cache_key)
      .unwrap().clone()
    )
  } else {
      match util::http_client::make_request_with_fallback::<NOD>(nod_url, get_default_nod()).await {
        Ok(data) => {
          let mut new_cache: HashMap<String, NOD> = 
          if !app_cache.nod.as_ref().is_none() {
            app_cache.nod.clone().unwrap()
          } else {
            HashMap::new()
          };
          new_cache.insert(set_cache_key, data.clone());
          app_cache.nod = Some(
            new_cache.clone()
          );
          app_cache.nod_dt = Some(util::datetime::now());
          Some(data)
        },
        Err(_err) => None
      }
  };

  match result {
    Some(data) => {
      let limit: u32 =  match params.limit {
        None => 20,
        Some(l) => l
      };
      let mut mut_data = data.clone();
      mut_data.articles = util::vector::get_slice(data.articles, 0, limit as usize);
      Some(mut_data)
    },
    None => None
  }
}

pub async fn news_of_day(data: web::Data<Mutex<AppCache>>, info: web::Query<NODRequest>) -> impl Responder {
  match get_nod(data, NODRequest { country: info.country.to_owned(), limit: info.limit.to_owned() }).await {
    Some(result) => HttpResponse::Ok().json(result),
    None => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}

/*
  History of day
*/

pub async fn get_hod (data: web::Data<Mutex<AppCache>>, params: HODRequest) -> Option<HOD> {
  let mut app_cache = data.lock().unwrap();
  let result = if app_cache.hod_exists() {
    app_cache.hod.clone()
  } else {
    let live_result = util::http_client::requests_seq_with_success_or_fallback::<HOD>(
        RequestSeqWithSuccessFallbackParams {
          requests: vec![
            HttpRequestParams { 
              id: Some("secure-hod-date".to_owned()),
              url: "https://history.muffinlabs.com/date".to_owned(),
              method: HttpVerb::GET,
              response_type: None,
              query_params: None,
              body: None
            },
            HttpRequestParams { 
              id: Some("secure-hod-date-month-day".to_owned()),
              url: format!(
                "https://history.muffinlabs.com/date/{}/{}",
                util::datetime::get_current_month(), 
                util::datetime::get_current_day()
              ).to_owned(),
              method: HttpVerb::GET,
              response_type: None,
              query_params: None,
              body: None
            },
            HttpRequestParams { 
              id: Some("insecure-hod-date".to_owned()),
              url: "http://history.muffinlabs.com/date".to_owned(),
              method: HttpVerb::GET,
              response_type: None,
              query_params: None,
              body: None
            },
            HttpRequestParams { 
              id: Some("insecure-hod-date-month-day".to_owned()),
              url: format!(
                "http://history.muffinlabs.com/date/{}/{}",
                util::datetime::get_current_month(), 
                util::datetime::get_current_day()
              ).to_owned(),
              method: HttpVerb::GET,
              response_type: None,
              query_params: None,
              body: None
            }
          ],
          default_value: get_default_hod()
        }
      ).await;
    
    app_cache.hod = Some(live_result.clone());
    app_cache.hod_dt = Some(util::datetime::now());
    Some(live_result)
  };
  match result {
    Some(data) => {
      match params.limit {
        None => Some(data),
        Some(limit) => {
          let mut mut_result = data.clone();          
          mut_result.data.Events = util::vector::get_slice(data.data.Events, 0, limit as usize);
          mut_result.data.Births = util::vector::get_slice(data.data.Births, 0, limit as usize);
          mut_result.data.Deaths = util::vector::get_slice(data.data.Deaths, 0, limit as usize);
          Some(mut_result)
        }
      }
    },
    None => None
  }
}

pub async fn history_of_day(data: web::Data<Mutex<AppCache>>, info: web::Query<HODRequest>) -> impl Responder {
  match get_hod(data, HODRequest { limit: info.limit.to_owned() }).await {
    Some(result) => HttpResponse::Ok().json(result),
    None => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}

/* Song of the day */
pub async fn get_sod(data: web::Data<Mutex<AppCache>>) -> Option<SOD> {
  // Temporary return to find new alt for SpotifyCharts
  return None;
  let mut app_cache = data.lock().unwrap();
  let sod_sources = vec![
    ("https://spotifycharts.com/regional/global/daily/latest/download", "spotify-regional"),
    ("https://spotifycharts.com/viral/global/daily/latest/download", "spotify-global"),
    // ("https://itunes.apple.com/us/rss/topsongs/limit=200/json", "itunes-topsongs")
  ];
  let (sod_url, sod_source) = sod_sources[util::vector::get_random_in_range(sod_sources.len())];
  let records = if app_cache.sod_exists() {
    app_cache.sod.clone()
  } else {
    match util::http_client::make_request_raw(
      HttpRequestParams {
        id: Some(sod_source.to_string()),
        url: sod_url.to_string(),
        method: HttpVerb::GET,
        response_type: None,
        query_params: None,
        body: None
      }
    ).await {
      Ok(data) => {
        let records = util::spotify_csv::response_to_records(data).await?;
        println!("Successfully parsed {} record(s)", records.len());
        app_cache.sod = Some(records.clone());
        app_cache.sod_dt = Some(util::datetime::now());
        Some(records)
      },
      Err(_err) => None
    }
  };
  match records {
    Some(recs) => {
      let random_ptr = util::vector::get_random_in_range(recs.len());
      let chosen_song = &recs[
        random_ptr
      ];
      println!("Picking record# {} - {:?}", random_ptr, chosen_song);
      let result = util::spotify_csv::record_to_sod(Some(chosen_song));
      Some(result)
    },
    None => None
  }
}

pub async fn song_of_day(data: web::Data<Mutex<AppCache>>) -> impl Responder {
  match get_sod(data).await {
    Some(result) => HttpResponse::Ok().json(result),
    None => HttpResponse::new(StatusCode::from_u16(500).unwrap())
  }
}

/* Today - Unified API */
pub async fn today(data: web::Data<Mutex<AppCache>>, info: web::Query<TodayRequest>) -> impl Responder {
  let qod = get_qod(data.clone()).await;
  let wod = get_wod(data.clone(), WODRequest { location: info.location.to_owned(), unit: info.wod_unit.to_owned() }).await;
  let nod = get_nod(data.clone(), NODRequest { country: info.country.to_owned(), limit: info.nod_limit.to_owned() }).await;
  let hod = get_hod(data.clone(), HODRequest { limit: info.hod_limit.to_owned() }).await;
  let sod = get_sod(data.clone()).await;
  HttpResponse::Ok().json::<TodayResponse>(
    TodayResponse { 
      qod,
      wod,
      nod,
      hod,
      sod
    }
  )
}


