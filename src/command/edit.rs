use chrono::NaiveDate;
use clap::Parser;

use crate::csv::*;
use crate::domain::contact::Gender;

#[derive(Debug, Parser)]
pub struct EditArguments {
    #[clap(long)]
    name: String,
    #[clap(long)]
    birthday: Option<String>,
    #[clap(long)]
    phone: Option<String>,
    #[clap(long)]
    email: Option<String>,
    #[clap(long)]
    location: Option<String>,
    #[clap(long)]
    gender: Option<Gender>,
}

pub fn edit(args: EditArguments) {
    if let Some(mut contact) = find_by_name(&args.name) {
        let mut modified = false;
        if let Some(birthday) = args.birthday {
            let birthday = NaiveDate::parse_from_str(birthday.as_str(), "%Y-%m-%d").unwrap();
            modified = modified || contact.birthday != birthday;
            contact.birthday = birthday;
        }
        if let Some(phone) = args.phone {
            modified = modified || contact.phone != phone;
            contact.phone = phone;
        }
        if let Some(email) = args.email {
            modified = modified || contact.email != email;
            contact.email = email;
        }
        if let Some(location) = args.location {
            modified = modified || contact.location != location;
            contact.location = location;
        }
        if let Some(gender) = args.gender {
            modified = modified || contact.gender != gender;
            contact.gender = gender;
        }

        if modified {
            write(&contact);
            println!("<<< Edeted contact with name `{}` >>>", args.name);
        }
    }
}
