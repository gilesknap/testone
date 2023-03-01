// some experiments with functions et al

mod chapt02;
mod chapt03;
mod chapt04;
mod chapt05;
mod chapt06;
mod chapt10;
pub mod generics;
pub mod polymorph;

fn main() {
    polymorph::test_poly();
    // remove me to run everything else
    if true {
        return;
    }
    mutate();
    generics::test_duck();

    chapt10::test_trait();
    chapt10::test_mixup();

    chapt06::test_message();

    chapt05::rectangle();
    chapt05::structures();

    chapt04::mutable_string();
    chapt04::data_copy();
    chapt04::copy_functions();
    chapt04::borrowing();
    chapt04::mutable_ref();
    chapt04::mixed_refs();
    chapt04::slices();

    chapt03::expressions();
    chapt03::if_else(3);
    chapt03::break_with_return();
    chapt03::while_demo();
    chapt03::for_demo();

    chapt03::array_index();

    chapt02::guessing_game();
}

fn mutate() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    v[0] = 3;

    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("a", 1);
    hash_map.insert("b", 2);
}
