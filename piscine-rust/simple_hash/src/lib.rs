use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut freq_map = HashMap::new();
    for &word in words {
        *freq_map.entry(word).or_insert(0) += 1;
    }
    freq_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}