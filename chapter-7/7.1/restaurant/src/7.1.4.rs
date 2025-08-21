// 7.1.4 => Module System - Paths, Modules Privacy Rule

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

/*
Notes:
- if enum is set as public all it's variants are public too
*/
