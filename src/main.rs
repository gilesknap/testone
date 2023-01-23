// some experiments with functions et al

mod chapt02;
mod chapt03;
mod chapt04;
mod chapt05;
mod chapt06;
mod chapt10;
pub mod generics;

fn main() {
    generics::test_duck();

    // remove me to run everything else
    if true {
        return;
    }
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
