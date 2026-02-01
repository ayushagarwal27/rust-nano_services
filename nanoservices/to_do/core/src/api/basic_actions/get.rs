use glue::errors::NanoServiceError;

use to_do_dal::to_do_items::{schema::AllToDoItems, transactions::get::GetAll};

pub async fn get_all<T: GetAll>(user_id: i32) -> Result<AllToDoItems, NanoServiceError> {
    let all_items = T::get_all(user_id).await?;
    AllToDoItems::from_vec(all_items)
}
