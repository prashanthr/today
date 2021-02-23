use rand::Rng;

pub fn get_slice<T: Clone>(data: Vec<T>, start: usize, end: usize) -> Vec<T> {
  let max_len = data.len();
  let slice_end = if end <= max_len { end }  else { max_len };
  data[start..slice_end].to_vec()
}

pub fn get_random_in_range(max: usize) -> usize {
  rand::thread_rng().gen_range(0..max)
}
