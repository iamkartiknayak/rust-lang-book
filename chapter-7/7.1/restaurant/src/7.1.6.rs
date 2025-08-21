// 7.1.6 => Module System - The `use` keyword (Resolving conflicts)

use std::fmt;
use std::io;

fn function_one() -> fmt::Result {
    Ok(())
}
fn function_two() -> io::Result<()> {
    Ok(())
}

// OR

use std::fmt::Result;
use std::io::Result as IoResult;

fn function_three() -> Result {
    Ok(())
}
fn function_four() -> IoResult<()> {
    Ok(())
}

/*
Notes:
- Resolving conflicts by either
    - having parent module
    - renaming one while bringing into scope using `as` keyword
*/
