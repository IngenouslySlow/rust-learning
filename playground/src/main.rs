/*---------------------------- NOTES ---------------- */
// Docs in Rust with Cargo
// You can create docs in rust by using ///
// # Heading
// /** Multiline block */
// Once done use -- cargo doc to create the docs
// To open docs in the browser use -- cargo doc --open
// See Crates.io to find all rust libraries

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
