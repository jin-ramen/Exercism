use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|candidate| {
            is_anagram(word, candidate)
        })
        .copied()
        .collect()
}

pub fn is_anagram(word1: &str, word2: &str) -> bool {
    let w1 = word1.to_lowercase();
    let w2 = word2.to_lowercase();
    
    if w1 == w2 {
        return false;
    }

    let mut w1_chars: Vec<char> = w1.chars().collect();
    w1_chars.sort_unstable();

    let mut w2_chars: Vec<char> = w2.chars().collect();
    w2_chars.sort_unstable();

    w1_chars == w2_chars
}