pub mod create;
pub mod get;

use actix_web::web::{self, post, ServiceConfig};
use auth_dal::users::descriptors::SqlxPostGresDescriptor;

pub fn users_factory(app: &mut ServiceConfig) {
    println!("✓ Users factory called - registering /api/v1/users/create");
    app.route(
        "/users/create",
        post().to(create::create::<SqlxPostGresDescriptor>),
    )
    .route(
        "/users/get",
        web::get().to(get::get_by_unique_id::<SqlxPostGresDescriptor>),
    );
}
