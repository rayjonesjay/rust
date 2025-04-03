// to_url
pub fn to_url(s: &str) -> String {
    let mut new_str = String::new();
    for c in  s.chars() {
        if c == ' '{
            new_str.push_str("%20")
        }else{
            new_str.push(c)
        }
    }
    new_str
}