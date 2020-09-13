use core::fmt::Debug;
extern crate lru;

use lru::LruCache;

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