#[macro_use]
extern crate lazy_static;

mod check;

use check::*;
use std::{
    collections::HashMap,
    fmt::{self, Display},
};

pub struct Anagrams<K, V>(HashMap<K, V>);

impl Anagrams<u64, Vec<String>> {
    pub fn new(s: &str) -> Self {
        let mut h: HashMap<u64, Vec<String>> = HashMap::new();
        s.to_string()
            .to_lowercase()
            .split_whitespace()
            .for_each(|w| {
                let unique_product = prime_product(&w);
                if let Some(mut v) = h.get_mut(&unique_product) {
                    v.push(w.to_string());
                } else {
                    h.insert(unique_product, vec![w.to_string()]);
                }
            });
        Self(h)
    }
}

impl fmt::Display for Anagrams<u64, Vec<String>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut total = String::new();
        let mut ouc = 0u64;
        for (key, values) in &self.0 {
            ouc += 1u64;
            let mut sf = format!("Unique Product {}: {} \n ", ouc, key);
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
