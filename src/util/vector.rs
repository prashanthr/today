use rand::seq::SliceRandom;

pub fn get_slice<T: Clone>(data: Vec<T>, start: usize, end: usize) -> Vec<T> {
  let max_len = data.len();
  let slice_end = if end <= max_len { end }  else { max_len };
  data[start..slice_end].to_vec()
}

/*
  Get a random element from a list of items
  Usage:
  let items = vec![1,2,3];
  let random_el = get_random(&items);
*/
pub fn get_random<T: Clone + std::fmt::Debug>(data: &Vec<T>) -> Option<&T> {
  let chosen_one = data.choose(&mut rand::thread_rng());
  match chosen_one {
    Some(el) => Some(el),
    None => None
  }
}