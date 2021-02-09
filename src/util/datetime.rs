use chrono::{DateTime, Duration, Utc};
use chrono::prelude::Datelike;

pub fn now () -> DateTime<Utc> {
  Utc::now()
}

pub fn in_range(start: DateTime<Utc>, duration: Duration) -> bool {
  let max = start.checked_add_signed(duration).unwrap();
  let now = now();
  let is_same_day = now.date() == start.date();
  let is_in_range = is_same_day && now <= max;
  println!("now is {}", if is_in_range { "in range" } else { "not in range" });
  is_in_range
}

/*
  Gets the current day (day of the month)
*/
pub fn get_current_day() -> u32 {
  let now = now();
  now.day()
}

/*
  Gets the current month (month of the year) 
*/
pub fn get_current_month() -> u32 {
  let now = now();
  now.month()
}

