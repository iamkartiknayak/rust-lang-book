// 7.1.3 => Module System - Paths, Modules Privacy Rule

mod back_of_house {
    pub struct Breakfast {
        pub toast: String, // pub needed to change value to Wheat
        seasonal_fruit: String,
    }

    impl Breakfast {
        // Builds a summer breakfast meal
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"), // pre-defined value as it's private and cannot be set
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    // `seasonal_fruit` field is private so one cannot create Breakfast directly
    // let meal2 = back_of_house::Breakfast {
    //     toast: String::from("Wheat"),
    //     seasonal_fruit: String::from("peaches");
    // };
}

/*
Notes:
- By default fields within the struct are private, in this case `seasonal_fruit`
*/
