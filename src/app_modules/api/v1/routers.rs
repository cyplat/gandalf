// use uuid::Uuid;
// use actix_web::{get, web, Responder};

// use super::user_handlers::UserHandler;

// // #[get("/users/{user_id}")]
// // async fn index(path: web::Path<Uuid>) -> impl Responder {
// //     let user_id = path.into_inner();

// //     let hhandler = UserHandler::new().await;
// //     hhandler.get_user(user_id).await
// // }

// fn configure_routes(cfg: &mut ServiceConfig) {
//     cfg.service(
//         web::scope("/api/v1")
//             // .route("/users", web::post().to(UserController::create_user))
//             .route("/users/{user_id}", web::get().to(UserController::get_user))
//     );
// }
