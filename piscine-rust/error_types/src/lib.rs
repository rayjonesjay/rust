use chrono::prelude::*;
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String,String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        FormError {
            form_values:(field_name.parse().unwrap(), field_value),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err.parse().unwrap(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name : String,
    pub password: String
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        let mut has_chars = false;
        let mut is_not_greater_8 = false;
        let mut  username_empty = false;

        if self.name.is_empty() {
           username_empty = true;
        };
        
        if self.password.len() < 8 {
            is_not_greater_8 = true;
        }

        for c in self.password.chars(){
            if c.is_ascii_punctuation() {
                has_chars=true;
                // break we only need at least one punctuation mark
                break;
            };
        }

        const MESSAGES: &[&str; 3] = &["Username is empty","Password should be at least 8 characters long","Password should be a combination of ASCII numbers, letters and symbols"];
        let mut msg = "";
        let mut name = "";

        if username_empty {
            msg = MESSAGES[0];
            name = "name";
        }

        if !has_chars {
            msg = MESSAGES[2];
            name = "password";
        }

        if is_not_greater_8 {
            msg = MESSAGES[1];
            name = "password";
        }

        if !msg.is_empty(){
            Err(
                if name == "name"{
                    FormError::new(name,self.name.clone(), msg)
                }else {
                    FormError::new(name,self.password.clone(), msg)
                }
            )
        }else{
            Ok(())
        }

    }
}