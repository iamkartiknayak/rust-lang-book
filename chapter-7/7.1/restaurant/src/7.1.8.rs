// 7.1.8 => Module System - Nested Paths

use std::io::{self, Write};
/*
   same as `use std::io;`
           `use std::io::Write;`
*/

use std::io::*; // glob operator

use std::io::{ErrorKind::AddrInUse, stdin, stdout};

/*
Notes:
- using `*` ensures everything under a specific module that's publicy available is in the scope
*/
