use actix_web::web::{self, scope, ServiceConfig};
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.route(
        "/get/all",
        web::get().to(get::get_all::<SqlxPostGresDescriptor>),
    )
    .route(
        "/create",
        web::post().to(create::create::<SqlxPostGresDescriptor>),
    )
    .route(
        "/delete/{name}",
        web::delete().to(delete::delete_by_name::<SqlxPostGresDescriptor>),
    )
    .route(
        "/update",
        web::put().to(update::update::<SqlxPostGresDescriptor>),
    );
}
