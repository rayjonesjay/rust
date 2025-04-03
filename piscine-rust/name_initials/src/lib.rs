// Harry Potter -> H. P.
pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for name in names {
        // split at white space
        let words: Vec<&str> = name.split(' ').collect();
        let mut tmp: String = String::new();
        for (index,w) in words.iter().enumerate() {
            tmp.push(w.chars().next().unwrap());
            tmp.push('.');
            if index < words.len()-1 {
                tmp.push(' ');
            }
        }
        res.push(tmp.to_owned())
    }
    res
}