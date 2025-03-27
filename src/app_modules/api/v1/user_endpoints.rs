/*
 This module holds user endpoints.

 created modules must be registered in routes.rs
*/
use actix_web::{HttpResponse, Responder, get, post, web};

use uuid::Uuid;

use crate::app_modules::app_state::AppState;

use super::schemas::RegistrationRequestLocal;
use super::schemas::UserResponse;
use crate::adapters::dtos::RegistrationDto;
use crate::app_modules::auth::AuthMethod;
use crate::domain::errors::UserError;

use serde_json::json;

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

// Registration Endpoint
#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    registration_request: web::Json<RegistrationRequestLocal>,
) -> impl Responder {
    let user_data = registration_request.into_inner();

    // Select appropriate strategy based on registration method
    let strategy = match app_state
        .auth_service
        .strategies
        .get(&AuthMethod::EmailPassword)
    {
        Some(strategy) => strategy,
        None => {
            return HttpResponse::InternalServerError().json(json!({
                "error": "Authentication method not supported"
            }));
        }
    };

    // Attempt registration
    match strategy
        .register(RegistrationDto {
            email: user_data.email,
            password: Some(user_data.password),
        })
        .await
    {
        Ok(registered_user) => HttpResponse::Created().json(json!({
            "user": registered_user,
            "message": "Registration successful. Please verify your email."
        })),
        Err(e) => match e {
            UserError::UserAlreadyExists => HttpResponse::Conflict().json(json!({
                "error": "User already exists",
                "code": "USER_EXISTS"
            })),
            UserError::ValidationError(validation_errors) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "Validation failed",
                    "details": validation_errors.to_string()
                }))
            }
            UserError::InvalidEmail => HttpResponse::BadRequest().json(json!({
                "error": "Invalid email format",
                "code": "INVALID_EMAIL"
            })),
            _ => HttpResponse::InternalServerError().json(json!({
                "error": "Registration failed",
                "code": "REGISTRATION_ERROR"
            })),
        },
    }
}
