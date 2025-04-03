
// adds a val at the end of vec
pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

// insert a value at index position
pub fn at_index(slice: &[String], index: usize) -> &str {
    slice[index].as_str()
}
