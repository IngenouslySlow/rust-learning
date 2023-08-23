pub fn add_five(num: i32) -> i32 {
    num + 5
}

pub fn other_func() {
    println!("This function returns nothing");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_five_test() {
        let x = add_five(5);
        println!("Value of x is: {}", x); // By default running cargo test doesn't print anything to the terminal.. To print this line use -- cargo test -- --nocapture
        assert_eq!(x, 10);
    }
}
