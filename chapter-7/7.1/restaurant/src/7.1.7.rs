// 7.1.7 => Module System - The `use` keyword (`pub` for `use`)

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // no need of `front_of_house::`
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

/*
Notes:
- using `pub` for `use` to re-export, making sure external code can use it as well
*/
