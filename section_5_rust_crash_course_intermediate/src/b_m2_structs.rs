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
        // Structs are also mutable
        let mut superhero = Superhero {
            name: String::from("Batman"),
            human_name: String::from("Bruce Wayne"),
            enemies_killed: 10_000,
            retired: false,
        };
        dbg!(&superhero);
        // Let's create a function where it can change the items in the struct
        fn change_superhero_name(hero: &mut Superhero, new_superhero: &str) {
            hero.name = new_superhero.to_string();
        }

        change_superhero_name(&mut superhero, "Redhood");

        dbg!(&superhero);

        // You can also create so many functions using impl referencing the struct itself using the world self
        impl Superhero {
            fn increment_enemies(&mut self) {
                self.enemies_killed += 1;
            }

            fn change_name(&mut self, new_name: &str) {
                self.name = new_name.to_string();
            }

            fn change_human_name(&mut self, new_name: &str) {
                self.human_name = new_name.to_string();
            }

            fn change_retired(&mut self) {
                self.retired = !self.retired;
            }
        }

        // Now you can use them as follows

        superhero.change_human_name("Clark kent");
        superhero.change_name("Superman");
        superhero.change_retired();
        superhero.increment_enemies();
        dbg!(&superhero);
    }
}
