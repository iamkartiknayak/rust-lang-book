// 7.1.2 => Module System - Paths, Modules Privacy Rule

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order(); // defined in same module
        super::serve_order(); // `super::` allows to access parent module using relative path
    }

    fn cook_order() {}
}

/*
Notes:
- `super::` allows to access parent module using relative path
*/
