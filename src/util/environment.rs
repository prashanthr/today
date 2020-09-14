use std::env;
/*
Fetches the value of any environment variable
*/
pub fn get_env (key: &str, default_value: Option<&str>) -> String {
  let empty: &str = "";
  match env::var(key) {
    Ok(val) => {
      println!("Fetched env data: {}: {}", key, val);
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
