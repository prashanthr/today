extern crate lru;

use lru::LruCache;

pub struct AppCacheItem<T> {
  key: String,
  value: Option<T>
}

fn build_cache_item<T>(key: String, value: Option<T>) -> AppCacheItem<T> {
  AppCacheItem::<T> {
    key,
    value
  }
}

pub fn create<T>(size: usize) -> LruCache<String, AppCacheItem<T>> {
  let cache = LruCache::new(size);
  cache
}

pub fn exists<T>(mut cache: LruCache<String, AppCacheItem<T>>, key: String) -> bool {
  match cache.get(&key) {
    Some(_v) => true,
    None => false
  }
}

pub fn get<T>(mut cache: LruCache<String, AppCacheItem<T>>, key: String) -> AppCacheItem<T> {
  *cache.get(&key).unwrap()
}

pub fn put<T>(mut cache: LruCache<String, AppCacheItem<T>>, key: String, value: Option<T>) {
  cache.put(key, build_cache_item(key, value));
}