use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


mod services;
use services::{create_producto, create_cliente, fetch_productos, fetch_clientes, delete_producto, delete_cliente};//agregar las que faltan

pub struct AppState {
    db: Pool<Postgres>
}

#[tokio::main] // Usar Tokio como el runtime
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(fetch_productos)
            .service(fetch_clientes)
            .service(create_producto)
            .service(create_cliente)
            /*.service(update_producto)
            .service(update_cliente)*/
            .service(delete_producto)
            .service(delete_cliente)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}







