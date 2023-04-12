use clap::Parser;

use crate::csv::*;

#[derive(Debug, Parser)]
pub struct DeleteArguments {
    #[clap(long)]
    name: String,
}

pub fn delete(args: DeleteArguments) {
    delete_by_name(&args.name);
    println!("<<< Deleted contact with name `{}` >>>", args.name);
}
