use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppCache {
  pub qod: Option<Vec<Quote>>
}

impl AppCache {
  pub fn qod_exists(&self) -> bool {
    let is_full = !self.qod.is_none();
    println!("QOD cache is {}", if is_full { "full"  } else { "empty" });
    is_full
  }
  pub fn print(&self) {
    println!("AppCache data:");
    println!("QOD: {:?}", self.qod);
    // println!("WOD: {:?}", self.qod);
    // println!("WOD: {:?}", self.qod);
  }
}

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
