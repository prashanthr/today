extern crate reqwest;
extern crate serde_json;
use reqwest::{header, ClientBuilder};
use std::time::Duration;
use serde_json::{Result};
/*
 Makes a HTTP GET request
*/
pub async fn make_request<T: for<'de> serde::Deserialize<'de>> (url: &str) -> Result<T> {
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
