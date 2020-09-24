use md5;

pub fn compute_hash(input: String) -> String {
  String::from(format!("{:x}", md5::compute(input)))
} 
