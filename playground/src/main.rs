/*---------------------------- NOTES ---------------- */
/* Unit Testing with Rust */

/* See  funcs.rs and other_functs/minus.rs to see how to add testing*/

mod funcs;
mod other_funcs;

use crate::funcs::{add_five, other_func};
use crate::other_funcs::minus::minus_five;

fn main() {
    let x = add_five(5);
    println!("x: {}", x);

    let print_random = other_func();
    println!("{:?}", print_random);

    let y = minus_five(10);
    println!("y: {}", y);
}
