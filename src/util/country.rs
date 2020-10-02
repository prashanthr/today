use celes::Country;
use std::str::FromStr;

pub fn get_country(country: &str) -> Country {
  Country::from_str(country).unwrap()
}

pub fn get_country_code(country: &str) -> String {
  get_country(country).alpha2
}
