#[macro_use]
extern crate lazy_static;

mod check;

use check::*;
use std::{
    collections::HashMap,
    fmt::{self, Display},
};

pub struct Anagrams<K, V>(pub HashMap<K, V>);

impl Anagrams<(u64, usize), Vec<String>> {
    pub fn new(s: &str) -> Self {
        let mut h: HashMap<(u64, usize), Vec<String>> = HashMap::new();
        s.to_string()
            .to_lowercase()
            .split_whitespace()
            .for_each(|w| {
                let unique_product = prime_product(&w);
                if let Some(mut v) = h.get_mut(&(unique_product, w.len())) {
                    v.push(w.to_string());
                } else {
                    h.insert((unique_product, w.len()), vec![w.to_string()]);
                }
            });
        Self(h)
    }
}

impl fmt::Display for Anagrams<(u64, usize), Vec<String>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut total = String::new();
        let mut ouc = 0u64;
        for (key, values) in &self.0 {
            ouc += 1u64;
            let mut sf = format!("Unique Product {} with Len {}: {} \n ", ouc, key.0, key.1);
            let mut inc = 0u64;
            for v in values {
                inc += 1u64;
                let val = format!("Word {}: {} \n", inc, v);
                sf.push_str(&val);
            }
            total.push_str(&sf);
        }
        write!(f, "{}", total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn anagram_discovery_works_on_sentences() {
        let anagram_map = Anagrams::new("color and roloc under erund");
        let mut hmap: HashMap<(u64, usize), Vec<String>> = HashMap::new();
        hmap.insert(
            (prime_product("color"), 5usize),
            vec!["color".to_string(), "roloc".to_string()],
        );
        hmap.insert(
            (prime_product("erund"), 5usize),
            vec!["under".to_string(), "erund".to_string()],
        );
        hmap.insert((prime_product("and"), 3usize), vec!["and".to_string()]);
        assert!(anagram_map.0 == hmap);
    }
}
