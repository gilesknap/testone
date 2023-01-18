// Chapt 3. Common programming concepts

use std::io;

pub fn expressions() {
    expression();
    call_five();

    let condition = true;
    let number = if condition { 5 } else { 6 };
    let x = plus_one(number);

    println!("The value of plus_one({number}) is: {x}");

    // these functions are scoped to the expressions() function
    // (not really a chapter 3 thing - I just wanted to try it)
    fn expression() {
        // the final line in this block is the return value
        // it is an expression rather than a statement
        let y = {
            let x = 3;
            x + 1
        };

        // the ! indicates a macro
        println!("The value of 'let x=3; x+1' is: {y}");
    }

    fn call_five() {
        let x = five();

        println!("The value of five() is: {x}");
    }

    // a function that returns a value ends with an expression
    fn five() -> i32 {
        5
    }

    // a function that returns a value ends with an expression
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
}

pub fn if_else(number: i32) {
    if number % 4 == 0 {
        println!("number {number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("number {number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("number {number} is divisible by 2");
    } else {
        println!("number {number} is not divisible by 4, 3, or 2");
    }
}

pub fn break_with_return() {
    let mut counter = 0;

    // loops can be labeled - allowing you to specify which loop to break
    let result = 'break_loop_label: loop {
        counter += 1;

        if counter == 10 {
            // break can be followed by an expression to return from the loop
            break 'break_loop_label counter * 2;
        }
    };

    println!("The break result is {result}");
}

pub fn while_demo() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

pub fn for_demo() {
    // integer iterators can be specified with .. notation
    for element in 1..4 {
        println!("the value is: {element}");
    }
    // iterator mutator functions step_by() and rev() can be chained
    for element in (1..6).step_by(2).rev() {
        println!("the value is: {element}");
    }
}

pub fn array_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
