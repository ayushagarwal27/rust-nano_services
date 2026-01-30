pub mod login;
pub mod logout;

use actix_web::web::{get, scope, ServiceConfig};
use auth_dal::users::descriptors::SqlxPostGresDescriptor;

pub fn auth_factory(app: &mut ServiceConfig) {
    println!("Registering auth routes");
    app.route(
        "/auth/login",
        get().to(login::login::<SqlxPostGresDescriptor>),
    );
}
