use crate::domain::contact::Contact;

use crate::configs;
use csv;
use std::fs::File;
use std::fs::OpenOptions;
use std::vec::Vec;

use itertools::Itertools;

pub fn write(contact: &Contact) {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(get_file());
    wtr.serialize(&contact).unwrap();
    wtr.flush().unwrap();
}

pub fn find_by_name(name: &String) -> Option<Contact> {
    find_all()
        .into_iter()
        .find(|contact| contact.name.starts_with(name))
}

pub fn find_all() -> Vec<Contact> {
    let mut rdr = csv::Reader::from_reader(get_file());
    let mut all_contacts: Vec<Result<Contact, csv::Error>> = rdr.deserialize::<Contact>().collect();
    all_contacts.reverse();
    all_contacts
        .into_iter()
        .map(|result| result.unwrap())
        .unique_by(|contact| contact.name.clone())
        .filter(|contact| contact.active)
        .collect()
}

pub fn delete_by_name(name: &String) {
    if let Some(mut contact) = find_by_name(name) {
        contact.active = false;
        write(&contact);
    }
}

fn get_file() -> File {
    OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(false)
        .append(true)
        .open(configs::get_config().contacts_path)
        .unwrap()
}
