use crate::app::api::users;
use actix_web::web;

use super::api::healthcheck;

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    cfg.service((users::list, healthcheck::healthcheck))
}
