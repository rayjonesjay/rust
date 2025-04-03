pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let f : f64 = c as f64;
    (c,f.exp(),f.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut res: String = String::new();
    for c in a.chars(){
        if let Some(digit) = c.to_digit(10){
            let f = (digit as f64).exp();
            res.push_str(&f.to_string());
        }else{
            res.push(c);
        }
    }
    (a,res)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res : Vec<f64> = Vec::new();
    for n in b.clone() {
        let d = (n as f64);
        res.push(d.abs().ln());
    }
    (b,res)
}