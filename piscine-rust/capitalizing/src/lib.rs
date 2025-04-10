pub fn capitalize_first(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars();

    if let Some(first) = chars.next() {
        result.push(first.to_ascii_uppercase());
    }

    result.extend(chars); // push the rest of the characters
    result
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true; // Capitalize the next non-whitespace character
        } else {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();

    for c in input.trim().chars() {
        if c.is_uppercase() {
            for lc in c.to_lowercase() {
                result.push(lc);
            }
        } else if c.is_lowercase() {
            for uc in c.to_uppercase() {
                result.push(uc);
            }
        } else {
            result.push(c); // non-letter characters like space, punctuation
        }
    }

    result
}