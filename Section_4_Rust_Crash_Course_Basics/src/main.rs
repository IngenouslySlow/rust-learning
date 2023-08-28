const OUR_COURSE: &str = "Rust With AutoGPT";

fn main() {
    println!("Welocme to this course on {}", OUR_COURSE);

    // Stack -- Immutable (By default)
    let x: i32;
    x = 2;
    println!("x is {}", x);

    let y: i32 = 4;
    println!("y is {}", y);

    // Stack -- Mutable (Use mut)
    let mut z: i32 = 5;
    print!("Z was {}\n", z);
    z = 10;
    println!("Now z is {}", z);

    // Floats
    let reefer_temp: f64 = 15.0;
    println!("Reefer temp. is now set to {}", reefer_temp);

    // Booleans
    let genset_required: bool = reefer_temp > 20.0;
    println!("Genset required: {}", genset_required);

    // Chars - use '' single quotes while working with chars and that's how rust knows that it's a char
    let word_char: char = 'K';
    println!("Word Char is now set to: {}", word_char);

    let emoji_char: char = '🦇';
    println!("Bat signal activated: {}", emoji_char);

    // Arrays - Fixed -- Stored in stack -- Use {:?} while printing the result
    let fixed_array: [f32; 10] = [2.0; 10];
    println!("Fixed array: {:?}", fixed_array);

    // Looping through arrays -- map
    let looped_fixed_array: [f32; 10] = fixed_array.map(|n: f32| n + 2.0);
    println!("Looped Fixed Array: {:?}", looped_fixed_array);

    // For Loop - Loops
    for i in 0..=y {
        if i != 4 {
            print!("{}, ", i);
        } else {
            println!("{}", i);
        }
    }
}
