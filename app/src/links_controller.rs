use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};


use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct LinkObj {
    pub link: String,
}

#[get("/links")]
pub async fn get_all(app_state: web::Data<AppState>) -> HttpResponse {
    let result = app_state.link_repository.get_links().await;

    match result {
        Ok(links_vec) => HttpResponse::Ok().json(links_vec),
        Err(err) => {
            let error_message = format!("Database error: {}", err);
            HttpResponse::InternalServerError().body(error_message)
        }
    }
}

#[post("/links")]
pub async fn save_link(
    app_state: web::Data<AppState>,
    payload: web::Json<LinkObj>,
) -> HttpResponse {
    let result = app_state.link_repository.save_link(payload).await;
    match result {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(err) => {
            let error_message = format!("Database error: {}", err);
            HttpResponse::InternalServerError().body(error_message)
        }
    }
}
