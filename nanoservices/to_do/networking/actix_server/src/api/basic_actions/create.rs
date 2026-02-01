use auth_kernel::api::users::get::get_user_by_unique_id;
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};

use actix_web::{web::Json, HttpResponse};
use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_dal::to_do_items::{
    schema::NewToDoItem,
    transactions::{create::SaveOne, get::GetAll},
};

pub async fn create<T: SaveOne + GetAll>(
    token: HeaderToken,
    body: Json<NewToDoItem>,
) -> Result<HttpResponse, NanoServiceError> {
    let user = get_user_by_unique_id(token.unique_id).await?;
    let _ = create_core::<T>(body.into_inner(), user.id).await?;
    Ok(HttpResponse::Created().json(get_all_core::<T>(user.id).await?))
}
