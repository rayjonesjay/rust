pub fn first_subword(mut s: String) -> String {
    let mut res : String = String::new();
    for (i,c) in s.char_indices(){
        if c == '_'{
            break
        }
        if c.is_uppercase() && i != 0 {
            break
        }
        res.push(c);
    }
    res
}