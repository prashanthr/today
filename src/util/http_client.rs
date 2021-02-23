extern crate reqwest;
extern crate serde_json;
extern crate csv;
use reqwest::{header, ClientBuilder};
use std::time::Duration;
use serde_json::{Result};
use crate::types::{
  GenericResult,
  HttpRequestParams,
  RequestSeqWithSuccessFallbackParams,
  HttpResponseType,
  HttpVerb
};

/*
 Makes a raw HTTP GET request and gets the result or error
*/

pub async fn make_request_raw(request_params: HttpRequestParams) -> GenericResult<reqwest::Response> {
  let mut headers = header::HeaderMap::new();
  headers.insert(
    header::ACCEPT, 
    header::HeaderValue::from_static("application/json, text/plain, text/csv"
  ));
  let client = ClientBuilder::new()
    .default_headers(headers)
    .user_agent(concat!(
      env!("CARGO_PKG_NAME"),
      "/",
      env!("CARGO_PKG_VERSION")))
    .timeout(Duration::from_secs(10))
    .build().unwrap();
  println!("Making request to {}", request_params.url);
  match client
    .get(&request_params.url) // verb switch here
    .send()
    .await {
      Ok(data) =>  {
        println!("Made a successful HTTP request to {}", request_params.url);
        Ok(data)
      },
      Err(err) => {
        let err_msg = format!("Error: HTTP Request to {} failed", request_params.url);
        eprintln!("{} with err {}", err_msg, err);
        Err(err_msg)?
      }
    }
}

/*
 Makes a HTTP GET request and gets the result based on the passed in generic type or fails a non 2XX response
*/

pub async fn make_request<T: for<'de> serde::Deserialize<'de>> (request_params: HttpRequestParams) -> GenericResult<T> {
  let params = request_params.clone();
  match make_request_raw(request_params)
  .await {
      Ok(data) =>  {
        match data.status().is_success() {
          true => {
            println!("data received {:?}",data);
            match params.response_type {
              Some(res_type) => {
                match res_type {
                  HttpResponseType::JSON => Ok(data.json::<T>().await.unwrap()),
                  _ => {
                    let err_msg = "Error: No handler provided for unsupported response type passed in as argument";
                    eprintln!("{}", err_msg);
                    Err(err_msg)?
                  }
                }
              },
              None => Ok(data.json::<T>().await.unwrap())
            }
          },
          false => {
            let err_msg = format!("Error: Received non OK [2XX] response: {:?}", data);
            eprintln!("{}", err_msg);
            Err(err_msg)?
          }
        }    
      },
      Err(err) => {
        let err_msg = format!("Error: Error occurred when trying to make request to {}: {}", params.url, err);
        eprintln!("{}", err_msg);
        Err(err_msg)?
      }
  }
}

/*
 Makes a HTTP GET request with a fallback value so it never errors on any result
*/
pub async fn make_request_with_fallback<T: for<'de> serde::Deserialize<'de>> (url: &str, default_value: T) -> Result<T> {
  match make_request_raw(
    HttpRequestParams {
      id: None,
      url: url.to_owned(),
      method: HttpVerb::GET,
      response_type: None,
      query_params: None,
      body: None
    }
  ).await {
      Ok(data) =>  {
        match data.status().is_success() {
          true => Ok(data.json::<T>().await.unwrap()),
          false => {
            eprintln!("Received non OK response: {:?}", data);
            serde_json::Result::Ok(default_value)
          }
        }    
      },
      Err(err) => {
        eprintln!("Error occurred when trying to make request to {}: {}", url, err);
        serde_json::Result::Ok(default_value)
      }
  }
}


/*
  Runs a set of requests in sequence and returns after the first successful request or proceeds to the next
  It will error out if no request was successful
  References:
  https://stackoverflow.com/questions/50850309/how-do-i-iterate-over-a-vec-of-functions-returning-futures-in-rust
 */
pub async fn requests_in_sequence<T: for<'de> serde::Deserialize<'de>>(requests: Vec<HttpRequestParams>) -> GenericResult<T> {
  println!("Running {} request(s) in sequence", requests.len());
  
  let mut reqs = requests.into_iter().peekable();

  while let Some(r) = reqs.next() {
      println!("Running request with id {:?} - {:?}:{:?}...", r.id, r.method, r.url);
      match make_request::<T>(r).await {
          Ok(res) => {
            println!("Got successful response for req");
            return Ok(res)
          },
          Err(err) if reqs.peek().is_none() => {
            eprintln!("Error: Got an error and there is nothing more to iterate on {:?}", err);
            return Err(err)
          },
          Err(_) => { 
            /* Do nothing and try the next source */ 
            eprintln!("Warn: Got an error, trying next source...")
          }
      }
  }

  Err("Error: Unexpected err - ran out of requests".into())
}

/*
  Run a sequence of http requests and return the first successful value or a default value
  It simulates the JS promise.any() feature but in sequence
  References:
  https://blog.yoshuawuyts.com/futures-concurrency/
  https://stackoverflow.com/questions/50850309/how-do-i-iterate-over-a-vec-of-functions-returning-futures-in-rust
*/
pub async fn requests_seq_with_success_or_fallback<T: for<'de> serde::Deserialize<'de>>(opts: RequestSeqWithSuccessFallbackParams<T>) -> T {
  async {
    match requests_in_sequence::<T>(opts.requests)
      .await {
        Ok(result) => {
          println!("Success: Received at least one successful response in request chain");
          result
        },
        Err(err) => {
          eprintln!("Error: No successful requests in future chain: {}", err);
          opts.default_value
        }
    }
    
  }.await
}
