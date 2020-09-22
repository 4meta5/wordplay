//! Palindrome checks and generation

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
/// Generates palindromes using the input string as a dictionary
pub fn gen_pals(s: &str) -> Vec<String> {
    todo!()
}
