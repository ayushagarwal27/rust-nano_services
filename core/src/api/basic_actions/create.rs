use to_do_dal::json_file::save_one;

use crate::{enums::TaskStatus, structs::ToDoItem};

pub fn create(title: &str, status: TaskStatus) -> Result<ToDoItem, String> {
    let item = ToDoItem {
        title: title.to_string(),
        status,
    };
    let _ = save_one(&title.to_string(), &item)?;
    Ok(item)
}
