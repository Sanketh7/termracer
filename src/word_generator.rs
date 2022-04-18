use rand::distributions::{Distribution, Uniform};

use super::constants::WORDS_STRING;

pub struct WordGenerator {
    words: Vec<String>,
}

impl WordGenerator {
    pub fn new() -> WordGenerator {
        let words = WORDS_STRING
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        WordGenerator { words }
    }

    pub fn get_random_words(&self, count: usize) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let dist = Uniform::from(0..self.words.len());
        let mut result: Vec<String> = Vec::new();

        for _ in 0..count {
            let idx = dist.sample(&mut rng);
            match self.words.get(idx) {
                Some(word) => result.push(word.to_string()),
                None => (),
            }
        }

        result
    }
}
