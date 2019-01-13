use std::collections::HashMap;

fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces i: {}", spaces);

    //Calling Functions
    print_number(7);
    print_number(-1);


    // HashMaps
    let mut book_reviews = HashMap::new();
    book_reviews.insert("key1".to_string(), "value1".to_string());
    //Use unwrap if you know the key exists - bad practice - may panic
    let review = book_reviews.get("key1").unwrap();
    println!("'get' version Review: for key1 is {}", review);

    //proper, safe access to the value
    match book_reviews.get("key1") {
        Some(review) => println!("Pattern version review for key1: {}", review),
        None => println!("Pattern version key1 is unreviewed.")
    }


}

fn print_number(x: i32) {
    println!("x is: {}", x);
}
