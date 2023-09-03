#[derive(Debug)]
enum GuessTheCity<T, E> {
    Ok(T),
    Err(E),
}

fn city_guesser(name: String) -> GuessTheCity<String, String> {
    if name == "Batman" {
        GuessTheCity::Ok("Gotham City".to_string())
    } else {
        GuessTheCity::Err("Metropolis".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let batman_city: GuessTheCity<String, String> = city_guesser("Batman".to_string());
        let superman_city: GuessTheCity<String, String> = city_guesser("Superman".to_string());
        dbg!(batman_city);
        dbg!(superman_city);
    }
}
