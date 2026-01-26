use actix_web::web::ServiceConfig;
pub mod basic_actions;

pub fn views_factory(app: &mut ServiceConfig) {
    basic_actions::basic_actions_factory(app);
}
