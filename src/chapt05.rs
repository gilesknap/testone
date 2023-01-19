// Chapter 5: Structs

//
// Simple structs --------------------------------------------------------------
//

pub fn structures() {
    struct User {
        _active: bool,
        _username: String,
        email: String,
        _sign_in_count: u64,
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        _username: String::from("some_username123"),
        _active: true,
        _sign_in_count: 1,
    };

    user1.email = String::from("another_email@example.com");

    println!("user1 email is {:#?}", user1.email);

    // shortcut to create a struct with some fields from another struct
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user2 email is {}", user2.email);

    // a unit struct is a struct with no fields (like a tuple with no elements)
    // but you can apply traits to it
    struct AlwaysEqual;

    let _subject = AlwaysEqual;
}

//
// structs with methods --------------------------------------------------------
//

#[derive(Debug)]
// this derives the struct from the Debug trait and allows us to print it
// with :? (print) or :#? (pretty print)
struct Rectangle {
    width: u32,
    height: u32,
}

// add a methods to the struct - know as associated functions
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // shadow the width field with a method
    fn width(&self) -> bool {
        self.width > 0
    }

    // a method that takes another Rectangle as an argument
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// A second impl block for the same struct (no good reason to do this, yet!
// see Chapt 10)
impl Rectangle {
    // this is an associated function but NOT a method as it does not take
    // &self as an argument. This one is a constructor.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn rectangle() {
    let scale = 2;

    // use debug output to show info about the struct
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("sq is {:#?}", sq);
}
