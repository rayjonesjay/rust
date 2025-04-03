// find factorial
pub fn factorial(num: u64) -> u64 {
    let mut sum = 1;
    for i in 1..=num {
        sum *= i
    }
    sum
}