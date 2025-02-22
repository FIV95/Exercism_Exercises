use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    // we are given
    // target word
    // array of string references
    // we are returning a HashSet

    let mut frequency_table: HashMap<char, i64> = HashMap::new();
    
    // define a HashTable

    // gonna add each character of the word string to a hashtable to check character frequency <key = char || value = freq>
    for char in word.chars() {
        *frequency_table.entry(char).or_insert(0) += 1;
    }

    // for each string in the array slice
    let filtered_anagrams: Vec<&str> = possible_anagrams
        .iter()
        .filter(|&&word| word != word)
        .clone()
        .collect();
    // first we compare equality directly - exact same words are not anagrams
    possible_anagrams.retain(|&word| word != word);
}
