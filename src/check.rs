//! Check if strings are or contain palindromes and anagrams
use std::collections::HashMap;

lazy_static! {
    static ref PRIMES: HashMap<char, u64> = {
        let mut m = HashMap::new();
        m.insert('a', 2);
        m.insert('b', 3);
        m.insert('c', 5);
        m.insert('d', 7);
        m.insert('e', 11);
        m.insert('f', 13);
        m.insert('g', 17);
        m.insert('h', 19);
        m.insert('i', 23);
        m.insert('j', 29);
        m.insert('k', 32);
        m.insert('l', 37);
        m.insert('m', 41);
        m.insert('n', 43);
        m.insert('o', 47);
        m.insert('p', 53);
        m.insert('q', 59);
        m.insert('r', 61);
        m.insert('s', 67);
        m.insert('t', 71);
        m.insert('u', 73);
        m.insert('v', 79);
        m.insert('w', 83);
        m.insert('x', 89);
        m.insert('y', 97);
        m.insert('z', 101);
        m
    };
}

/// Maps each string to the product of primes
/// -> Encodes letter frequency for every word using Fundamental Theorem of Arithmetic
pub fn prime_product(s: &str) -> u64 {
    s.to_string()
        .to_lowercase()
        .chars()
        .filter_map(|c| PRIMES.get(&c))
        .product()
}
/// Checks if two strings are anagrams of each other
pub fn is_ana(s: &str, t: &str) -> bool {
    let (s, t) = (
        s.to_string()
            .to_lowercase()
            .chars()
            .filter(|&c| c.is_alphanumeric())
            .collect::<String>(),
        t.to_string()
            .to_lowercase()
            .chars()
            .filter(|&c| c.is_alphanumeric())
            .collect::<String>(),
    );
    if s.len() != t.len() || s.len() <= 1 {
        false
    } else {
        prime_product(&s) == prime_product(&t)
    }
}
/// Checks if an input string is a palindrome
pub fn is_pal(s: &str) -> bool {
    let s = s
        .to_string()
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_alphanumeric())
        .collect::<String>();
    s.len() > 1 && s == s.chars().rev().collect::<String>()
}
/// Checks if a word in the input string is a palindrome
pub fn has_pal(s: &str) -> bool {
    s.to_string()
        .to_lowercase()
        .split_whitespace()
        .any(|w| is_pal(w))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ana_works() {
        assert!(is_ana("color", "roloc"));
        assert!(is_ana("hash", "shah"));
        assert!(is_ana("cali", "lica"));
        assert!(is_ana("under", "erund"));
        assert!(is_ana("highlight", "lighthigh"));
        assert!(!is_ana("should", "fail"));
    }
    #[test]
    fn pal_works() {
        assert!(is_pal("pull up if I pull up"));
        assert!(!is_pal("code or die"));
        assert!(has_pal("adam and eve"));
        assert!(!has_pal("adam and"));
    }
}
