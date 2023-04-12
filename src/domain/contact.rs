use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub birthday: NaiveDate,
    pub phone: String,
    pub email: String,
    pub location: String,
    pub gender: Gender,
    pub active: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GenderParseError;

impl Error for GenderParseError {}

impl Display for GenderParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error, cannot parse a gender variant from string slice.")
    }
}

impl FromStr for Gender {
    type Err = GenderParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            _ => Err(GenderParseError),
        }
    }
}

impl Contact {
    pub fn print(&self) {
        match self.gender {
            Gender::Male => println!(" 👨 {}", self.name),
            Gender::Female => println!(" 👩 {}", self.name),
        }
        println!(" 🎂 {}", self.birthday);
        println!(" ☎️  {}", self.phone);
        println!(" 📧 {}", self.email);
        println!(" 🏠 {}", self.location);
        println!("+-------------------------+");
    }
}
