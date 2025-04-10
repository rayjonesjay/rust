/*
Create a function named arrange_phrase, that takes a string literal,
sorts the words and returns it. Each word will contain a
number that indicates the position of that word.
 */
pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by_key(|word| word.chars().find(|c| c.is_digit(10)).unwrap_or('0').to_digit(10).unwrap());

    words.iter()
        .map(|word| word.chars().filter(|c| !c.is_digit(10)).collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

