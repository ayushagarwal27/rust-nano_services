use actix_web::web::ServiceConfig;

pub mod auth;
pub mod users;

pub fn views_factory(app: &mut ServiceConfig) {
    app.configure(users::users_factory)
        .configure(auth::auth_factory);
}
