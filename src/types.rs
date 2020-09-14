use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppCache {
  pub qod: Option<Vec<Quote>>
}

impl AppCache {
  pub fn qod_exists(&self) -> bool {
    !self.qod.is_none()
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
