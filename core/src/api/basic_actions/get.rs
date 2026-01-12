use crate::structs::{AllToDoItems, ToDoItem};
use to_do_dal::json_file::get_all as get_all_handle;

pub async fn get_all() -> Result<AllToDoItems, String> {
    let all_items = get_all_handle::<ToDoItem>()?;
    Ok(AllToDoItems::from_hashmap(all_items))
}
