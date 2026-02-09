pub mod login;
pub mod logout;

use actix_web::web::{get, ServiceConfig};
use auth_dal::users::descriptors::SqlxPostGresDescriptor;
use auth_kernel::user_session::descriptors::RedisSessionDescriptor;

pub fn auth_factory(app: &mut ServiceConfig) {
    println!("Registering auth routes");
    app.route(
        "/auth/login",
        get().to(login::login::<SqlxPostGresDescriptor, RedisSessionDescriptor>),
    )
    .route("/auth/logout", get().to(logout::logout));
}
