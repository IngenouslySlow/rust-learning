// ------- Result and Option -- Same thing we've implemented before--------

// Result
fn city(name: String) -> Result<String, String> {
    if name == "Gotham" {
        Result::Ok("I'm batman".to_string())
        // Or you can also do this as this is built-in
        // Ok("I'm batman".to_string())
    } else {
        Result::Err("I'm Superman".to_string())
        // Or you can also do this as this is built-in
        // Err("I'm Superman".to_string())
    }
}

// Option
fn bat_signal(crime: bool) -> Option<String> {
    if crime == true {
        Option::Some("Batman's here".to_string())
        // Or you can also do this as this is built-in
        // Some("Batman's here");
    } else {
        Option::None
        // Or you can also do this as this is built-in
        // None
    }
}

#[cfg(test)]
mod test {
    use std::string;

    use super::*;

    #[test]
    fn tests_enums() {
        let batman: Result<String, String> = city("Gotham".to_string());
        let superman: Result<String, String> = city("Metropolis".to_string());
        dbg!(batman);
        dbg!(superman);

        let bat_signal_on: Option<String> = bat_signal(true);
        let bat_signal_off: Option<String> = bat_signal(false);
        dbg!(bat_signal_on);
        dbg!(bat_signal_off);
    }
}
