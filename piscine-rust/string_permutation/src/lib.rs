use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut s1_freq: HashMap<char, i32> = HashMap::new();
    let mut s2_freq: HashMap<char, i32> = HashMap::new();

    for c in s1.chars() {
        *s1_freq.entry(c).or_insert(0) += 1;
    }
    for c in s2.chars() {
        *s2_freq.entry(c).or_insert(0) += 1;
    }

    for (key, value) in &s1_freq {
        match s2_freq.get(key) {
            Some(&v) => {
                if v != *value {
                    return false;
                }
            }
            None => return false, // Key from s1 is not in s2
        }
    }

    true
}
