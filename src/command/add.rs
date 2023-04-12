use chrono::NaiveDate;
use clap::Parser;

use crate::csv::*;
use crate::domain::contact::Contact;
use crate::domain::contact::Gender;
use crate::validation::*;

#[derive(Debug, Parser)]
pub struct AddArguments {
    #[clap(long, validator = validate_name)]
    name: String,
    #[clap(long, validator = validate_birthday)]
    birthday: String,
    #[clap(long, validator = validate_phone)]
    phone: String,
    #[clap(long, validator = validate_email)]
    email: String,
    #[clap(long)]
    location: String,
    #[clap(long)]
    gender: Gender,
}

impl From<AddArguments> for Contact {
    fn from(args: AddArguments) -> Self {
        Contact {
            name: args.name,
            birthday: NaiveDate::parse_from_str(args.birthday.as_str(), "%Y-%m-%d").unwrap(),
            phone: args.phone,
            email: args.email,
            location: args.location,
            gender: args.gender,
            active: true,
        }
    }
}

pub fn add(args: AddArguments) {
    let contact: Contact = args.into();
    write(&contact);
    println!("<<< Added a new contact >>>");
    contact.print();
}
