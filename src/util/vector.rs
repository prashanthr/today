pub fn get_slice<T: Clone>(data: Vec<T>, start: usize, end: usize) -> Vec<T> {
  let max_len = data.len();
  let slice_end = if end <= max_len { end }  else { max_len };
  data[start..slice_end].to_vec()
}
