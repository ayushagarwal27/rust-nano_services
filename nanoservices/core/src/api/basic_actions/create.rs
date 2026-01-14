use to_do_dal::json_file::save_one;

use crate::{enums::TaskStatus, structs::ToDoItem};
use glue::errors::NanoServiceError;

pub async fn create(item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let _ = save_one(&item.title.to_string(), &item)?;
    Ok(item)
}
