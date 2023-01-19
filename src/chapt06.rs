// Chapter 6 - Enums, Pattern Matching

// enum with 4 variants storing different amounts and types
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    OptionalVariant(Option<i32>),
    Unloved1,
    Unloved2(i32),
}

// add methods to the enum
impl Message {
    fn call(&self) {
        // use the match flow control construct !! also note 100 characters per line as per style guide (but comments are exempt!!!)
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to {}, {}", x, y),
            Message::Write(s) => println!("Write {}", s),
            Message::ChangeColor(r, g, b) => {
                println!(
                    "Change color (and make a really long line)  {}, {}, {}",
                    r, g, b
                )
            }
            Message::OptionalVariant(i) => match i {
                Some(i) => println!("Optional variant {}", i),
                None => println!("Optional variant with no parameter"),
            },
            // cath all other variants and assign to variable other (or use _ to ignore)
            other => println!("Unloved variant (catchall) {:?}", other),
        }
    }
    fn _call2(&self) {
        // use the if let flow control construct (I suppose this works because Quit is a unit variant)
        if let Message::Quit = self {
            println!("call2 Quit");
        }
    }
}

pub fn test_message() {
    let m = Message::Write("hello".to_string());
    m.call();
    let m = Message::Quit;
    m.call();
    m._call2();
    let m = Message::Move { x: 1, y: 2 };
    m.call();
    let m = Message::ChangeColor(1, 2, 3);
    m.call();
    let m = Message::OptionalVariant(Some(1));
    m.call();
    let m = Message::OptionalVariant(None);
    m.call();
    m._call2();
    let m = Message::Unloved1;
    m.call();
    m._call2();
    let m = Message::Unloved2(42);
    m.call();
    m._call2();
}
