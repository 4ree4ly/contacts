use clap::Parser;

use crate::csv::*;

#[derive(Debug, Parser)]
pub struct FindArguments {
    #[clap(long)]
    name: Option<String>,
}

// TODO: add highlitning for matched text
pub fn find(args: FindArguments) {
    if let Some(name) = args.name {
        if let Some(cnt) = find_by_name(&name) {
            cnt.print();
        }
    } else {
        let contacts = find_all();
        contacts.into_iter().for_each(|contact| contact.print());
    }
}
