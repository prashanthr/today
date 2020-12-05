use celes::Country;
use std::str::FromStr;

fn get_default_country() -> Country {
  Country::the_united_states_of_america()
}

pub fn get_country(country: &str) -> Country {
  Country::from_str(country).unwrap_or(get_default_country())
}

pub fn get_country_code(country: &str) -> String {
  get_country(country).alpha2
}
