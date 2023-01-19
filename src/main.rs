// some experiments with functions et al

mod chapt03;
mod chapt04;
mod chapt05;

fn main() {
    chapt05::rectangle();

    // remove me to run everything else
    if true {
        return;
    }

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
}
