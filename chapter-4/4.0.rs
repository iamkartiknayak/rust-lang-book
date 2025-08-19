// 4.0 => Ownership & Borrowing -

fn main() {
    fn a() {
        let x = "Hello Rust!"; // stored in stack
        let y = 22;
        b();
    }

    fn b() {
        let x = String::from("Rust is awesome!"); // stored in heap
    }
}

/*
Notes:
- Stack frames are created for each function which stores it's local var
- Accessing stack data (i32, &str) is faster than heap data (String, vec!)
- Stack variables are static in size whereas heap var are dynamic in size
- Stack allocations are contagious and cache friendly, heap allocations require de-referencing a pointer
- Heap allocates memory & passes back a pointer which gets stored in the stack
- Pushing to stack is faster than asking heap to allocate memory
- Accessing data on stack is faster than the heap cuz it has to follow the pointer (memory is scattered in RAM)
*/
