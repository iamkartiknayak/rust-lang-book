// 7.1.0 => Module System - Defining Modules

mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

/*
Notes:
- Inside a mod body, one can only declare items such as (functions, structs, enums, traits, impl blocks, type aliases, constants, etc.).
- Inside a mod body, one cannot declare items such as function calls
*/
