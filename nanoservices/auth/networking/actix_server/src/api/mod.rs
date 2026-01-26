use actix_web::web::ServiceConfig;

pub mod auth;
pub mod users;

pub fn views_factory(app: &mut ServiceConfig) {
    users::users_factory(app);
    auth::auth_factory(app);
}
