#[derive(Debug)]
enum CarColors {
    Red,
    Blue,
    Black,
    Silver,
}

fn select_car_color() -> CarColors {
    CarColors::Black
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_color: CarColors = select_car_color();
        dbg!(car_color);
    }
}
