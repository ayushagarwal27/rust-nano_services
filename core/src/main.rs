use crate::{api::basic_actions::create::create, enums::TaskStatus};

use clap::Parser;

mod api;
mod enums;
mod structs;

#[derive(Parser, Debug)]
#[command(version,about, long_about=None)]
struct Args {
    #[arg(short, long)]
    title: String,
    #[arg(short, long)]
    status: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let status_enum = TaskStatus::from_string(&args.status)?;

    let to_do_item = create(&args.title, status_enum)?;

    println!("{}", to_do_item);
    Ok(())
}
