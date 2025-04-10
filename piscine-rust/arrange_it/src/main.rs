use arrange_it::arrange_phrase;

fn main() {
    let phrase = "is2 This1 a4 test3";
    println!("{}", arrange_phrase(phrase)); // Output: "This is test a"
}