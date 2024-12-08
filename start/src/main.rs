mod math;

fn main() {
	let x = 5; // this is immutable
	println!("x is: {}",x);	
	let mut y = 10; // this is mutable
	println!("y is: {}",y);
	y = 16; // changing the value of y
	println!("y is now: {}",y);
	println!("Hello, world!");

	let integer: i32 = 10; // explicit declaration
	let inferred_float = 4.2; // implicit defaults to f64
	println!("integer {}, float: {}",integer,inferred_float);


	// ownership
	let s1 = String::from("hello");
	let s2 = s1.clone(); // s1 moved to s2, so s1 is no longer valid, no garbage collection needed
	println!("{}",s2);
//	println!("{}",s1);

	let s3 = s1.clone();
	println!("{}",s3);

	// borrowing

	let mut s = String::from("gnx");
	let r1 = &s;
	println!("r1: {}",r1);
	
	let r2 = &mut s;
	r2.push_str(", kdot");
	println!("r2: {}",r2);
	
	let result = add(2,3);
	println!("result={}",result);


	// loops
	for i in 1..5 {
		println!("i is {}",i);
	}

	let mut v = vec![1,2,3];
	v.push(4);
	println!("vector: {:?}",v);
	

	println!("************");
	
	let (quotient,reminder) = divide(10,4);
	println!("quotient {}\nreminder {}",quotient,reminder);

	let m = math::ml(2,3);
	println!("{}",m);
}

// function add
fn add(a: i32, b: i32) -> i32 {
	a + b
}

// dont put semi-colon to indicate its a return value
// if you put semi-colon it turns to a statement
// this causes an error
fn divide(a: i32, b: i32) -> (i32, i32) {
	(a/b,a%b)	
}
