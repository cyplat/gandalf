// Server builder

use actix_web::{
    App, HttpResponse, HttpServer,
    middleware::{Logger, NormalizePath},
    web::{self, ServiceConfig},
};

use actix_cors::Cors;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use serde_json::json;
use tracing_subscriber;

use crate::app_modules::app_state::AppState;
use crate::config::database::PgPool;

use crate::app_modules::api::v1::user_handlers::UserHandler;

// Configuration function for routes
fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/v1").route("/users/{user_id}", web::get().to(UserHandler::get_user)),
    );
}

pub struct WebServer {
    listener: TcpListener,
    db_pool: PgPool,
}

impl WebServer {
    pub fn new(listener: TcpListener, db_pool: PgPool) -> Self {
        Self { listener, db_pool }
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        // Create application state
        let app_state = web::Data::new(AppState::new(self.db_pool.clone()));

        // Initialize tracing/logging
        tracing_subscriber::fmt::init();

        // Start HTTP server
        HttpServer::new(move || {
            App::new()
                // Middleware
                .wrap(Logger::default())
                .wrap(TracingLogger::default())
                .wrap(Cors::default())
                .wrap(NormalizePath::trim())
                // Application state
                .app_data(app_state.clone())
                // Configure routes
                .configure(configure_routes)
                // Fallback handler
                .default_service(web::route().to(|| async {
                    HttpResponse::NotFound().json(json!({
                        "error": "Not Found",
                        "message": "The requested resource could not be found"
                    }))
                }))
        })
        .listen(self.listener)?
        .run()
        .await
    }
}
