mod routes;

use actix_cors::Cors;
use actix_web::{HttpServer, middleware::Logger, App, http::header};
use sqlx::{Postgres, Pool, postgres::PgPoolOptions};
use dotenv::dotenv;
use routes::{ health_route::health_checker_handler };

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url: String = std::env::var("DATABASE_URL").expect("DATA_URL must be set");
    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to database!");
            pool
        }
        Err(e) => {
            println!("Error connecting to database: {:?}", e);
            std::process::exit(1);
        }
    };

    println!("Server started successfully!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH"])
            .allowed_origin("http://localhost:3000")
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::CONTENT_ENCODING
            ])
            .supports_credentials();

        App::new()
            .app_data(actix_web::web::Data::new(AppState { db: pool.clone() }))
            .service(health_checker_handler)
            // .configure(config)
            .wrap(cors)
            .wrap(Logger::default())

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
