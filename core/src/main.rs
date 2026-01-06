use crate::api::basic_actions::create::create;

mod api;
mod enums;
mod structs;

fn main() {
    let to_do_item = create("washing", enums::TaskStatus::PENDING);
    println!("{}", to_do_item);
}
