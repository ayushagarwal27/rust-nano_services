use actix_web::HttpResponse;
use auth_kernel::user_session::transactions::get::GetUserSession;
use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::get::get_all as get_all_core;
use to_do_dal::to_do_items::transactions::get::GetAll;

pub async fn get_all<T: GetAll, X: GetUserSession>(
    token: HeaderToken,
) -> Result<HttpResponse, NanoServiceError> {
    let session = X::get_user_session(token.unique_id).await?;
    Ok(HttpResponse::Ok().json(get_all_core::<T>(session.user_id).await?))
}
