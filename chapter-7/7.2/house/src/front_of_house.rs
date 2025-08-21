// represent a file located in the directory with the same name as the parent module
pub mod hosting; // valid => `src/front_of_house/hosting.rs`

// pub mod front_door; // error => searches for `src/front_of_house/front_door/mod.rs` or `src/front_of_house/front_door.rs`

/*
- Only at root level like `lib.rs` or `main.rs` does it look for `src/front_door/mod.rs` if declared
*/
