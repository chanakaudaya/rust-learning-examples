fn main() {
    println!("Hello, ownership!");

    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("Chanaka");
    // Cloning the data here because RUST does not allow accessing s1 after moving the ownership to s2
    let s2 = s1.clone();

    println!("s1 is {} and s2 is {}", s1, s2);
    int_copy();

    let a = String::from("Hello");
    takes_ownership(a);
    // The below line does not work because the ownership of "a" has been transferred to takes_ownership function
    //println!("{} World!", a);

    let b = 5;
    makes_copy(b);
    println!("{} World!", b);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    let n = calculate_length(&s1);
    println!("The length of string {} is {}", s1, n);

    let mut s = String::from("hello");
    change(&mut s);
    println!("Mutated string is {}", s);

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    println!("first word is {}", word);

}

fn int_copy() {
    println!("Starting the integer copy function");
    let x = 5; 
    // Here we don't use clone since types stored in stack does not need the clone method
    let y = x;
    println!("The value of x is {}, the value of y is {}", x, y);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    println!("from gives ownership {}", some_string);
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("from takes and gives back {}", a_string);
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
