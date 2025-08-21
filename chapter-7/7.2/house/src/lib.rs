mod front_of_house; // represents a file `front_of_house.rs` or looks for `front_of_house/mod.rs`

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // no need of `front_of_house::`
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

/*
Notes:
- We can declared modules in Rust using `mod` keyword
- We can define the contents inline using  `{}` (eg: mod front_of_house {})
- Use `;` if we want to have content in different file (eg: mod front_of_house;)
*/
