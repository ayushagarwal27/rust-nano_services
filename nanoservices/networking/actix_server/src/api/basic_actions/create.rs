use actix_web::{HttpResponse, web::Json};
use glue::errors::NanoServiceError;
use glue::token::HeaderToken;
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};
use to_do_core::structs::ToDoItem;

pub async fn create(
    token: HeaderToken,
    body: Json<ToDoItem>,
) -> Result<HttpResponse, NanoServiceError> {
    println!("Token: {}", token.message);
    let _ = create_core(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
