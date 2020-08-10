mod basics;
mod advanced;

fn main() {
    // Basics
    println!("Hello, Rust!");
    println!("Variables and types ....... ");
    println!("Sum of 5 and 6 is {}", basics::vars::add(5, 6));
    println!("Is abba a palindrome? {}", basics::vars::is_palindrome(String::from("abba")));
    println!("Is 12 an even? {:?}", basics::vars::get_type(12));
    println!("Stack and heap ....... ");
    basics::stack_and_heap::run();
    println!("Control structures ....... ");
    basics::control_structures::run_while(10);
    basics::control_structures::run_for();
    basics::control_structures::run_if_else(10);
    basics::control_structures::run_ternary_if_else(10);
    basics::control_structures::run_match(String::from("Basketball"));
    println!("Data structures ....... ");
    basics::data_structures::set_default_task();
    basics::data_structures::update_status(basics::data_structures::TaskStatus::InProgress);
    basics::data_structures::update_status(basics::data_structures::TaskStatus::Completed);
    basics::data_structures::array_ops();
    basics::data_structures::tuple_ops();
    basics::data_structures::slice_ops(&[12, 13, 14]);

    // Advanced
    println!("Collections ....... ");
    advanced::collections::generic_ops();
    advanced::collections::vector_ops();
    advanced::collections::hashmap_ops();
    advanced::collections::hashset_ops();

}