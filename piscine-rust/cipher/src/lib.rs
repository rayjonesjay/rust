#[derive(Debug, PartialEq)]
pub struct CipherError {
    // expected public fields
    pub expected: String
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    // I only need to convert one then compare with the other, either way the result is same since it's a mirror.

    let mut converted : String = String::new();

    for c in original.chars() {
        converted.push(converter(c));
    }

    if converted == ciphered {
        Ok(())
    }else{
        Err(CipherError{expected:converted.to_string()})
    }

}
fn converter(c: char) -> char {
    const LOW_FIRST_LETTER: char = 'a';
    const LOW_LAST_LETTER: char = 'z';
    const UP_FIRST_LETTER: char = 'A';
    const UP_LAST_LETTER:char = 'Z';

    // check if char is upper or lower than do calculations
    if c.is_ascii_lowercase() {
        let res = char::from_u32(LOW_LAST_LETTER as u32 - (c as u32 - LOW_FIRST_LETTER as u32));
        match res {
            Some(result) => result,
            // None => None,
            // None => {0}
            _ => c
        }
    }else if c.is_ascii_uppercase(){
        let options = char::from_u32(UP_LAST_LETTER as u32 - (c as u32 - UP_FIRST_LETTER as u32));
        match options {
            Some(res ) => res,
            _ => c
        }
    }else{
        c
    }
}