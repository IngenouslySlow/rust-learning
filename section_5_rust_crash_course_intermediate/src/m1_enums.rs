#[derive(Debug)]

// ------- Option Enum with Some or None --------
enum BatmanCannotBeDefeated<T> {
    None,
    Some(T),
}

fn can_anyone_defeat_batman(answer: String) -> BatmanCannotBeDefeated<String> {
    if answer == "Yes" {
        BatmanCannotBeDefeated::Some("Cannot be defeated".to_string())
    } else {
        BatmanCannotBeDefeated::None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let not_defeated: BatmanCannotBeDefeated<String> =
            can_anyone_defeat_batman("Yes".to_string());
        let defeated: BatmanCannotBeDefeated<String> = can_anyone_defeat_batman("No".to_string());

        println!("{:?}", not_defeated);
        println!("{:?}", defeated);
    }
}
