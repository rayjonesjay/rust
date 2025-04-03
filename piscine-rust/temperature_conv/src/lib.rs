use std::f64;

pub fn fahrenheit_to_celsius(a: f64) -> f64 {
    (a - 32 as f64) / (9 as f64 / 5 as f64)
}

pub fn celsius_to_fahrenheit(a: f64) -> f64 {
     a * (9 as f64 / 5 as f64)  + 32 as f64
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_f_to_c() {
//         let temp_f = 20.0;
//         let expected_c = -6.666666666666666;
//
//         // Allow a small error margin for floating-point precision
//         let tolerance = 1e-9;
//         assert!((fahrenheit_to_celsius(temp_f) - expected_c).abs() < tolerance);
//     }
//
//     #[test]
//     fn test_c_to_f() {
//         let temp_c = -6.666666666666667;
//         let expected_f = 20.0;
//         assert!((celsius_to_fahrenheit(temp_c) - expected_f).abs() < 1e-9);
//     }
// }
