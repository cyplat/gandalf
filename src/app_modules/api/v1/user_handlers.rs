use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::app_modules::app_state::AppState;
use crate::domain::services::UserService;

use super::schemas::UserResponse;

pub struct UserHandler {
    user_service: UserService,
}

impl UserHandler {
    pub async fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    pub async fn get_user(
        app_state: web::Data<AppState>,
        user_id: web::Path<Uuid>,
    ) -> impl Responder {
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
}
