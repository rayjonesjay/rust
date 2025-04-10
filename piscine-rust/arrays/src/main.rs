use arrays::*;

fn main() {
    let a: [i32; 10] = (1..=10).collect::<Vec<i32>>().try_into().unwrap();
    let b: [i32; 5] = [1, 2, 3, 4, 5];
//     let b: [i32; 32] = [0; 32];
//     println!("The sum of the elements in {:?} is {}", a, sum(a));
    println!("The sum of the elements in {:?} is {}", b, sum(&b));
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}
