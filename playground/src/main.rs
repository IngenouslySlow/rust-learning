/*---------------------------- NOTES ---------------- */
// There are two ways to remove squiggly lines for the functions or variables that are unused

// Procedure 1 - Add an underscore
fn _add_five(num: u32) -> u32 {
    let _x = 5;
    let _y = 10;
    num + 5
}

// Procedure 2 - Use macros and allow them
#[allow(dead_code, unused_variables)]
fn minus_five(num: u32) -> u32 {
    let x = 5;
    let y = 10;
    num + 5
}

fn main() {}
