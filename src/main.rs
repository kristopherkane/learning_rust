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
    //Use unwrap if you know the key exists
    let review = book_reviews.get("key1").unwrap();

    println!("Review: for key1 is {}", review);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}
