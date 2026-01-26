use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::{schema::ToDoItem, transactions::update::UpdateOne};

pub async fn update<T: UpdateOne>(item: ToDoItem) -> Result<(), NanoServiceError> {
    let _ = T::update_one(item).await?;
    Ok(())
}
