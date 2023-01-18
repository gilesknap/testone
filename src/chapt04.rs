// Chapt 5. Ownership

// references rules
// - At any given time, you can have either one mutable reference or any number
//   of immutable references.
// - References must always be valid. (e.g. can't return pointer to local variable)

pub fn mutable_string() {
    {
        // 'from' literal copies the literal to the heap and returns a String
        let mut s = String::from("hello");
        // push_str() appends a literal to a String (ugh!)
        s.push_str(", world!");
        println!("{}", s);
    }
    // s is out of scope here so its heap is deallocated for us
    // Rust uses the 'drop' function to deallocate heap memory
    // implement drop to do something when a variable goes out of scope
}

pub fn data_copy() {
    // scalars go on the stack and are copied when assigned
    // The Copy trait implies this behaviour
    // Types with the Copy trait must NOT implement drop
    // Tuples containing Copy Trait types are also Copy
    let x = 5;
    // NOTE: underscore prefix avoids warning for unused variables
    let _y = x;

    // complex types go on the heap
    // assignment moves the reference, invalidating the original variable
    let s1 = String::from("hello");
    let _s2 = s1;

    // this would not compile because s1 is no longer valid, its heap memory
    // ownership has been moved to s2
    // println!("{}, world!", s1);

    // clone makes a deep copy so the original string is still valid
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

pub fn copy_functions() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    fn takes_ownership(some_string: String) {
        // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.

    fn makes_copy(some_integer: i32) {
        // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

pub fn borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    // s1 is still valid here because it was borrowed and returned
    println!("The length of '{}' is {}.", s1, len);

    // this function takes a reference to a String and borrows ownership
    // while it is running. Ownership returns to the caller when the function
    // returns
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
}

pub fn mutable_ref() {
    let mut s = String::from("hello");

    change(&mut s);

    // modified s is still valid here
    println!("{}", s);

    // this reference parameter is mutable so the function can change the
    // value of the String ONLY ONE MUTABLE REF allowed at a time
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // double mutable ref is allowed here as the first ref goes out of scope
    // before the second ref is created
    fn _double_mut_ref() {
        let mut s = String::from("hello");

        {
            let _r1 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference with no
          // problems.

        let _r2 = &mut s;
    }
}

pub fn mixed_refs() {
    // you cannot have a mutable reference and an immutable reference at the
    // same time as the mutable one may affect the data being read by the
    // immutable one
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // HOWEVER - this is OK because r1 and r2 go out of scope at their last
    // use (WTF???)

    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem ***
    println!("{}", r3);

    // this would be a compile error at *** above
    // println!("{}", r1);
}

pub fn slices() {
    let s = String::from("hello world");

    // &str is a slice of a String
    let hello = &s[0..5];
    let world = &s[6..11];

    // Note that a ref to a String is a slice of the entire String
    if &s[..] == &s {
        println!("slices are equal");
    }

    println!("slices: {} {}", hello, world);

    // slices can be used on arrays too
    // BTW this is a paste of
    // https://doc.rust-lang.org/book/ch04-03-slices.html#other-slices
    // vscode rust-analyzer added the type annotations (which are not
    // strictly necessary since the compiler can infer them)
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
