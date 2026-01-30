pub mod create;
use actix_web::web::{post, scope, ServiceConfig};
use auth_dal::users::descriptors::SqlxPostGresDescriptor;

pub fn users_factory(app: &mut ServiceConfig) {
    println!("✓ Users factory called - registering /api/v1/users/create");
    app.route(
        "/users/create",
        post().to(create::create::<SqlxPostGresDescriptor>),
    );
}
