use actix_web::web::{self, ServiceConfig, scope};

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(scope("/api/v1").route("/get/all", web::get().to(get::get_all)));
}
