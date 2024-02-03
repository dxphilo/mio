mod env;
mod links_repo;
mod links_controller;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{middleware::Logger, App, HttpServer};
use migration::Migrator;
use sea_orm_migration::MigratorTrait;

use crate::env::{db_url, load_config};
use crate::links_repo::LinksRepository;
use crate::links_controller::{ save_link, get_all};

#[derive(Debug, Clone)]
pub struct AppState {
    link_repository: LinksRepository,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_config();

    let db_url = db_url();
    let db_conn = sea_orm::Database::connect(&db_url)
        .await
        .expect("Database connection failed");
    
    // TODO: check migration status and handle appropriately
    Migrator::up(&db_conn, None)
        .await
        .expect("Migrations failed");

    let link_repository = LinksRepository {
        db_connection: db_conn.clone(),
    };

    let state = AppState { link_repository };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allowed_headers(vec!["content-type", "authorization"])
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .max_age(3600);

        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(get_all)
            .service(save_link)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
