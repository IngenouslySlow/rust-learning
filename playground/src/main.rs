const PLAYERS: u8 = 11; // Can't be mutable and global (available everywhere)

fn main() {
    // Stack
    let x: u8 = 18;
    println!("Value of x is: {}", x);

    // Vector - Heap
    let vec: Vec<u32> = vec![1, 2, 3, 4, 5];
    println!("Value of vec is: {:?}", vec);

    // Mutable Vector - Heap
    let mut vec_2: Vec<u32> = vec![1, 2, 3, 4, 5];
    vec_2.push(6);
    println!("Value of vec_2 is: {:?}", vec_2);

    // String - Heap
    let str: String = String::from("Bruce Wayne");
    println!("Value of str is: {:?}", str);

    // Mutable String - Heap
    let mut str_2: String = String::from("Bruce Wayne");
    str_2.push(' ');
    str_2.push('🦇');
    println!("Value of str_2 is: {:?}", str_2);

    // Reference string - Stack pointing the address
    let str_3 = &str_2[0..5];
    println!("Value of str_3 is: {:?}", str_3);

    // Const
    println!("No. of players are: {}", PLAYERS);
}
