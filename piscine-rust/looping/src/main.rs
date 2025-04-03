use std::io;

fn main(){
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let mut trials = 0;
    let answer = "The letter e";

    loop {
        println!("{}",riddle);
        // mutable string since it will be changing
        let mut in_put = String::new();

        io::stdin().read_line(&mut in_put).expect("Failed to read line");

        // mutate
        let in_put =  in_put.trim();

        trials = trials+1;
        if answer == in_put {
            break
        }
    }
    println!("Number of trials: {}",trials);
}