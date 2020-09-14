// use core::fmt::Debug;
use std::collections::HashMap;
use std::fmt::Debug;

extern crate lru;

use lru::LruCache;

#[derive(Debug)]
pub struct AppCache<T> {
  cache: HashMap<String, T>
}

impl<T: Copy + std::fmt::Debug> AppCache<T> {
  pub fn new() -> AppCache<T> {
    AppCache {
      cache: HashMap::new()
    }
  }
  
  pub fn exists(&mut self, key: String) -> bool {
    match self.cache.get(&key) {
      Some(_v) => true,
      None => false
    }
  }
  
  pub fn get(self, key: String) -> T {
    *self.cache.get(&key).unwrap()
  }
  
  pub fn put(&mut self, key: String, value: T) {
    self.cache.insert(key, value);
  }
  
  pub fn print(&self) {
    println!("Printing cache values");
    for (key, val) in self.cache.iter() {
      println!("key: {} val: {:?}", key, val);
    }
  }
}

pub fn create<T>(size: usize) -> LruCache<String, T> {
  let cache: LruCache<String, T> = LruCache::new(size);
  cache
}

pub fn exists<T>(mut cache: LruCache<String, T>, key: String) -> bool {
  match cache.get(&key) {
    Some(_v) => true,
    None => false
  }
}

pub fn get<T: Copy>(mut cache: LruCache<String, T>, key: String) -> T {
  *cache.get(&key).unwrap()
}

pub fn put<T>(mut cache: LruCache<String, T>, key: String, value: T) {
  cache.put(key, value);
}

pub fn print<T: Debug>(mut cache: LruCache<String, T>) {
  println!("Printing cache values");
  for (key, val) in cache.iter() {
    println!("key: {} val: {:?}", key, val);
  }
}