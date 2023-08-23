pub fn minus_five(num: i32) -> i32 {
    num - 5
}

#[cfg(test)]

mod test {
    use super::minus_five;

    #[test]
    fn subracts_five_test() {
        let y = minus_five(10);
        print!("Value of y is: {}", y);
        assert_eq!(y, 5);
    }
}
