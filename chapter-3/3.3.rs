// 3.3 => Common concepts - Functions

fn main() {
    print_something();
    greet_user("Kartik!");

    let num1 = 12;
    let num2 = 22;
    let sum = get_sum(num1, num2);
    println!("The sum of {} & {} is {}", num1, num2, sum);
}

fn print_something() {
    println!("Rust is awesome!");
}

fn greet_user(user_name: &str) {
    println!("Welcome {}", user_name);
}

fn get_sum(num1: i32, num2: i32) -> i32 {
    // return num1 + num2; OR
    num1 + num2
}

/*
Notes:
- To return a value from fn one can use one of the following as last line of a fn
    - `return <expr/var>;`
    - `<expr/var>` // expr or var with no semicolon
- `-> i32` denotes the type of value being returned from the fn
*/
