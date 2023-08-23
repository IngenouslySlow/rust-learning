/*---------------------------- NOTES ---------------- */
/* File structure in rust */
/* You can import a function or anything into rust's main.rs file */
/* You need to say rust that the file exists with mod filename (You can escape .rs from a filename)*/
/* Then you need to import the crate by using :: like use crate::filename::functionname */
/* See below how add_five is imported */

/* Importing an other folder */
/* Create another folder */
/* Add a file and make a public function */
/* You need to create another mod.rs file in that folder and add the file you just created like so -- pub mod filename */
/* Now, you can use that function in main.rs file like how we used before */
mod funcs;
mod other_funcs;
use crate::funcs::{add_five, other_func};
use crate::other_funcs::minus::minus_five;

fn main() {
    let x: i32 = add_five(5);
    println!("x: {}", x);

    let print_func = other_func();
    println!("{:?}", print_func);

    let y = minus_five(10);
    println!("y: {}", y);
}
