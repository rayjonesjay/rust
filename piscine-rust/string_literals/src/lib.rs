pub fn is_empty(v: &str) -> bool {
    v.len()==0
}

pub fn is_ascii(v: &str) -> bool {
    // chars() returns an iterator
    // all takes a closure that returns either true or false
    v.chars().all(|c| c.is_ascii())
}

pub fn contains(v: &str, pat: &str) -> bool {
    // // check if pat is empty
    // if pat.is_empty(){
    //     // empty string is always present as a substring in any string
    //     return true;
    // }
    //
    // let size = pat.len();
    // // if length of pat is greater than size
    // // then it is not a substring of v
    // if size > v.len() {
    //     return false;
    // }
    //
    // for i in 0..=v.len()-size{
    //     if &v[i..i+size] == pat {
    //         return true;
    //     }
    // }


    // any takes a closure that returns either true or false
    v.as_bytes().windows(pat.len()).any(|substring| substring == pat.as_bytes())
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index],&v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    v.char_indices()
        .find(|(_, c)| *c == pat)
        .map(|(i, _)| i)
        .unwrap_or(usize::MAX)
}
