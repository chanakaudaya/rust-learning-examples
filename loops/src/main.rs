fn main() {
    println!("Hello, loops!");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 5;
        }
    };

    println!("Value returned from loop is {}", result);
    while_test();
    loop_collection();
    for_loop();
    for_loop_range();

}

fn while_test() {
    println!("Staring the while loop");
    let mut x = 3;
    while x != 0 {
        println!("{}!", x);
        x -= 1;
    }
    println!("LIFTOFF!!!");
}

fn loop_collection() {
    println!("Starting the looping through the collection");
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop() {
    println!("Staring the for loop test");
    let a = [1,2,3,4,5];
    
    for element in a.iter() {
        println!("the value is {}!", element);
    }
    println!("LIFTOFF!!!");
}

fn for_loop_range() {
    println!("Staring the for loop with range test");    
    for number in (1..4).rev() {
        println!("the value is {}!", number);
    }
    println!("LIFTOFF!!!");
}


