use rand::distributions::{Distribution, Uniform};

lazy_static! {
    // source: https://www.ef.edu/english-resources/english-vocabulary/top-3000-words/
    static ref WORDS: Vec<String> = include_str!("../assets/words.txt")
        .split_whitespace()
        .map(String::from)
        .collect();
}

pub fn generate_words(count: usize) -> Vec<String> {
  let mut rng = rand::thread_rng();
  let dist = Uniform::from(0..WORDS.len());

  (0..count)
    .map(|_| WORDS.get(dist.sample(&mut rng)).unwrap().clone())
    .collect()
}
