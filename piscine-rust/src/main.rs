
fn main() {
    println!("hello this is the best way to dance");
    let v = "hello world";
    let r = v.as_bytes().windows(v.len());
    for (index,char) in r.enumerate() {
        println!("{} {:?}",index,char);
    }
    // println!("{:?}",r);
}
