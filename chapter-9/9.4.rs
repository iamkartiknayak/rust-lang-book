// 9.4 => - Error Handling - Error Propagation

pub struct Guess {
    value: u8,
}

impl Guess {
    pub fn new(value: u8) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 & 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

fn main() {
    Guess::new(55); // valid
    Guess::new(101); // error => invalid input
}
