use auth_kernel::user_session::transactions::get::GetUserSession;
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};

use actix_web::{web::Json, HttpResponse};
use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_dal::to_do_items::{
    schema::NewToDoItem,
    transactions::{create::SaveOne, get::GetAll},
};

pub async fn create<T: SaveOne + GetAll, X: GetUserSession>(
    token: HeaderToken,
    body: Json<NewToDoItem>,
) -> Result<HttpResponse, NanoServiceError> {
    let session = X::get_user_session(token.unique_id).await?;
    let _ = create_core::<T>(body.into_inner(), session.user_id).await?;
    Ok(HttpResponse::Created().json(get_all_core::<T>(session.user_id).await?))
}
