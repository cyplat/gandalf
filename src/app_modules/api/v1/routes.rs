use actix_web::web;

use super::user_endpoints;

// Grouped routes for users
pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(user_endpoints::get_user));
}
