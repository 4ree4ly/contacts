pub mod add;
pub mod delete;
pub mod edit;
pub mod find;

use clap::Parser;

use add::AddArguments;
use delete::DeleteArguments;
use edit::EditArguments;
use find::FindArguments;

use add::add;
use delete::delete;
use edit::edit;
use find::find;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[clap(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    Add(AddArguments),
    Delete(DeleteArguments),
    Edit(EditArguments),
    Find(FindArguments),
}

pub fn process() {
    match Arguments::parse().cmd {
        Subcommands::Add(args) => add(args),
        Subcommands::Delete(args) => delete(args),
        Subcommands::Edit(args) => edit(args),
        Subcommands::Find(args) => find(args),
    }
}
