fn main() {
    println!("Hello, variables!");
    let mut x = 5;
    println!("Value of x is {}", x);
    x = 6;
    println!("Value of x is now {}", x);

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    toupleLearn();
}

fn toupleLearn() {

    println!("Learning tuples in Rust");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
