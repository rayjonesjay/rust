use string_permutation::*;

fn main() {
    let word = "taca";
    let word1 = "caat";

    println!(
        "Is {:?} a permutation of {:?}? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );
}