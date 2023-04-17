use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NAME_REGEX: Regex =
        Regex::new("^[a-zA-Z]+(([',. -][a-zA-Z ])?[a-zA-Z]*)*$").unwrap();
    static ref PHONE_REGEX: Regex = Regex::new(r"^\+?[1-9][0-9]{7,14}$").unwrap();
    static ref EMAIL_REGEX: Regex = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
    static ref BIRTHDAY_REGEX: Regex = Regex::new(r"^[0-9]{4}-[0-9]{1,2}-[0-9]{1,2}$").unwrap();
}

pub fn validate_name(name: &str) -> Result<(), String> {
    if NAME_REGEX.is_match(name) {
        return Ok(());
    }
    Err(String::from("Error, invalid name!"))
}

pub fn validate_phone(phone: &str) -> Result<(), String> {
    if PHONE_REGEX.is_match(phone) {
        return Ok(());
    }
    Err(String::from("Error, invalid phone!"))
}

pub fn validate_email(email: &str) -> Result<(), String> {
    if EMAIL_REGEX.is_match(email) {
        return Ok(());
    }
    Err(String::from("Error, invalid email!"))
}

pub fn validate_birthday(birthday: &str) -> Result<(), String> {
    if BIRTHDAY_REGEX.is_match(birthday) {
        return Ok(());
    }
    Err(String::from("Error, invalid birthday!"))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_be_valid_name() {
        assert_eq!(super::validate_name("Beent Nice"), Ok(()));
    }

    #[test]
    fn should_be_invalid_name() {
        assert_eq!(
            validate_name("invalid_name"),
            Err(String::from("Error, invalid name!"))
        );
    }
}
