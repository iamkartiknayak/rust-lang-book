// 3.5 => Common concepts - `loop`, `while`

fn main() {
    // infinite loop
    // loop {
    //     println!("I'm an infinite loop!")
    // }

    // `loop` with `break` statement
    let mut count_down = 3;
    loop {
        println!("{}..", count_down);
        count_down -= 1;

        if count_down == 0 {
            println!("Launch...");
            break;
        }
    }

    // Returning a value from `loop`
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter;
        }
    };
    println!("The result is {}", result);

    // `while` loop
    let mut counter = 5;
    while counter > -1 {
        if counter != 0 {
            print!("{}, ", counter);
        } else {
            print!("{}\n", counter);
        }
        counter -= 1;
    }
}

/*
Notes:
- `loop` is infinite in nature & needs a break statement to exit
*/
