/*---------------------------- NOTES ---------------- */
/*Create  a function */
/* Every variable in rust is by default immutable */
/* To make it mutable you need to explicitly declare it as a mutable variable ex: let mut x: u32 = 50; */

fn add_five(num: u32) -> u32 {
    num + 5 // If we don't write return and don't put a semi-colon rust will return the line by default
}

fn main() {
    let mut x: u32 = 50;
    println!("x is {}", x);

    let y: u32 = add_five(x);
    println!("y is {}", y);

    // As we declared explicitly as a mutable variable, we can change x as follows
    x = 70;
    println!("x is now {}", x);
}
