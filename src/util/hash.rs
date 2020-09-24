use md5;

pub fn compute_hash(input: String) -> String {
  let cl = input.clone();
  let hash = String::from(format!("{:x}", md5::compute(input)));
  println!(">> input {:?} hash {:?}", cl, hash);
  hash
} 
