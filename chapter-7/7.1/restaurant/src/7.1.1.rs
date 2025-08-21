// 7.1.1 => Module System - Paths, Modules Privacy Rule

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {} // pub => public

        fn seat_at_table() {} // private
    }

    // serving::take_order();  error => arbitrary statement
}

pub fn eat_at_restruant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // front_of_house::hosting::seat_at_table(); // error => seat_at_table is private
}

// **********************************

/*
Notes:
- `pub` is used to make fn and mod public, without which it can't be used in outside module
*/
