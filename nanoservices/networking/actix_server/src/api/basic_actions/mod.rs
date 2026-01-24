use actix_web::web::{self, ServiceConfig, scope};
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
            .route(
                "/get/all",
                web::get().to(get::get_all::<SqlxPostGresDescriptor>),
            )
            // .route("/get/{name}", web::get().to(get::get_by_name))
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
            ),
    );
}
