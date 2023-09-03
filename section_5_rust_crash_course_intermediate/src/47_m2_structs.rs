// ------- Structs --------
#[derive(Debug)]
// Structs are just key value pairs
struct Superhero {
    name: String,
    human_name: String,
    enemies_killed: u64,
    retired: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs() {
        let superhero = Superhero {
            name: String::from("Batman"),
            human_name: String::from("Bruce Wayne"),
            enemies_killed: 10_000,
            retired: false,
        };

        dbg!(superhero);
    }
}
