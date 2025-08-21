// 7.1.5 => Module System - The `use` keyword

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // direct call to hosting
    hosting::add_to_waitlist(); // no need of `front_of_house::`
    hosting::add_to_waitlist();
}

/*
Notes:
- `use` allows to bring a path into scope
- we can furthur make it concise by bringing the fn `add_to_waitlist` in the scope by
    - `use crate::front_of_house::hosting::add_to_waitlist` but it's not idiomatic in Rust
    - Idiomatic way is to bring the parent mod of the fn in scope
    - Idiomatic to have full path if the type is structs, enums or other items
*/
