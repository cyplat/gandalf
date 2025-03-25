/*
 This module holds user endpoints.

 created modules must be registered in routes.rs
*/
use actix_web::{HttpResponse, Responder, get, web};

use uuid::Uuid;

use crate::app_modules::app_state::AppState;

use super::schemas::UserResponse;

#[get("/{user_id}")]
pub async fn get_user(app_state: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let user_id = user_id.into_inner();

    match app_state.user_service.get_user(user_id).await {
        Ok(Some(user)) => {
            let user_response = UserResponse::from(user);
            HttpResponse::Ok().json(user_response)
        }
        Ok(None) => HttpResponse::NotFound().json("User not found"),
        Err(e) => {
            println!("Error getting user: {}", e);
            HttpResponse::InternalServerError().json("Failed to get user")
        }
    }
}
