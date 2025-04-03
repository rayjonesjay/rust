pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let chars : Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len(){
        if chars[i] == '-'{
            result.pop();
            i += 1;

        }else if chars[i] == '+'{
            let mut plus_count = 0;
            while i < chars.len()  && chars[i] == '+' {
                i += 1;
                plus_count += 1;
            }
            i = i + plus_count;
        }else{
            result.push(chars[i]);
            i += 1;

        }
    }
    *s = result;
}


pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        if let Some(result) = evaluate_expression(s) {
            *s = result.to_string();
        }
    }
}

fn evaluate_expression(expr: &str) -> Option<i32> {
    let tokens: Vec<&str> = expr.split(|c| c == '+' || c == '-').collect();

    if tokens.len() != 2 {
        return None; // Invalid format
    }

    let left = tokens[0].trim().parse::<i32>().ok()?;
    let right = tokens[1].trim().parse::<i32>().ok()?;

    if expr.contains('+') {
        Some(left + right)
    } else if expr.contains('-') {
        Some(left - right)
    } else {
        None
    }
}