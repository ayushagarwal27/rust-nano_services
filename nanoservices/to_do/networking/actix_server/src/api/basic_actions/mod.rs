use actix_web::web::{self, scope, ServiceConfig};
use auth_kernel::user_session::descriptors::RedisSessionDescriptor;
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.route(
        "/get/all",
        web::get().to(get::get_all::<SqlxPostGresDescriptor, RedisSessionDescriptor>),
    )
    .route(
        "/create",
        web::post().to(create::create::<SqlxPostGresDescriptor, RedisSessionDescriptor>),
    )
    .route(
        "/delete/{name}",
        web::delete().to(delete::delete_by_name::<SqlxPostGresDescriptor, RedisSessionDescriptor>),
    )
    .route(
        "/update",
        web::put().to(update::update::<SqlxPostGresDescriptor, RedisSessionDescriptor>),
    );
}
