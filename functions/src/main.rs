fn main() {
    println!("Hello, world!");

    another_function(50);

    println!("Value from five() function is {}", five());

    let x = plus_one(5);

    println!("The value of x is: {}", x);

    // If condition check
    let x = 3;
    if x < 5 {
        println!("Condition is true!")
    } else {
        println!("Condition is false!");
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x :i32) {
    println!("Another function. value of x is {}", x);
}

fn five() -> i32 {
    5
}


