use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder, HttpResponse
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::AppState;

#[derive(Serialize, FromRow)]
struct Producto {
    id: i32,
    nombre: String,
    categoria: String,
    precio: f32,
    cantidad: i32,
}

#[derive(Serialize, FromRow)]
struct Cliente {
    id: i32,
    nombre: String,
    telefono: String,
    presupuesto: f32,
}

#[derive(Deserialize)]
pub struct CreateProductoBody {
    pub nombre: String,
    pub categoria: String,
    pub precio: f32,
    pub cantidad: i32,
}

#[derive(Deserialize)]
pub struct CreateClienteBody {
    pub nombre: String,
    pub telefono: String,
    pub presupuesto: f32,
}

pub struct UpdateProductoBody {
    pub nombre: Option<String>,
    pub categoria: Option<String>,
    pub precio: Option<f32>,
    pub cantidad: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateClienteBody {
    pub nombre: Option<String>,
    pub telefono: Option<String>,
    pub presupuesto: Option<f32>,
}

#[get("/productos")]
pub async fn fetch_productos(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Producto>("SELECT id, nombre, categoria, precio, cantidad FROM productos")
        .fetch_all(&state.db)
        .await
    {
        Ok(productos) => HttpResponse::Ok().json(productos),
        Err(_) => HttpResponse::NotFound().json("No productos found"),
    }
}

#[get("/clientes")]
pub async fn fetch_clientes(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Cliente>("SELECT id, nombre, telefono, presupuesto FROM clientes")
        .fetch_all(&state.db)
        .await
    {
        Ok(clientes) => HttpResponse::Ok().json(clientes),
        Err(_) => HttpResponse::NotFound().json("No clientes found"),
    }
}

#[post("/productos")]
pub async fn create_producto(state: Data<AppState>, body: Json<CreateProductoBody>) -> impl Responder {
    match sqlx::query_as::<_, Producto>(
        "INSERT INTO productos (nombre, categoria, precio, cantidad) VALUES ($1, $2, $3, $4) RETURNING id, nombre, categoria, precio, cantidad"
    )
        .bind(body.nombre.to_string())
        .bind(body.categoria.to_string())
        .bind(body.precio)
        .bind(body.cantidad)
        .fetch_one(&state.db)
        .await
    {
        Ok(producto) => HttpResponse::Ok().json(producto),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create producto"),
    }
}

#[post("/clientes")]
pub async fn create_cliente(state: Data<AppState>, body: Json<CreateClienteBody>) -> impl Responder {
    match sqlx::query_as::<_, Cliente>(
        "INSERT INTO clientes (nombre, telefono, presupuesto) VALUES ($1, $2, $3) RETURNING id, nombre, telefono, presupuesto"
    )
        .bind(body.nombre.to_string())
        .bind(body.telefono.to_string())
        .bind(body.presupuesto)
        .fetch_one(&state.db)
        .await
    {
        Ok(cliente) => HttpResponse::Ok().json(cliente),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create cliente"),
    }
}
#[put("/productos/{id}")]
pub async fn update_producto(
    state: Data<AppState>,
    id: Path<i32>,
    body: Json<UpdateProductoBody>
) -> impl Responder {
    let mut query = "UPDATE productos SET".to_string();
    let mut params: Vec<&(dyn sqlx::Encode<'_, Postgres> + Sync)> = Vec::new();
    let mut set_clauses: Vec<String> = Vec::new();

    if let Some(nombre) = &body.nombre {
        set_clauses.push("nombre = $1".to_string());
        params.push(&nombre);
    }
    if let Some(categoria) = &body.categoria {
        set_clauses.push("categoria = $2".to_string());
        params.push(&categoria);
    }
    if let Some(precio) = body.precio {
        set_clauses.push("precio = $3".to_string());
        params.push(&precio);
    }
    if let Some(cantidad) = body.cantidad {
        set_clauses.push("cantidad = $4".to_string());
        params.push(&cantidad);
    }

    if set_clauses.is_empty() {
        return HttpResponse::BadRequest().json("No fields to update");
    }

    query.push_str(&set_clauses.join(", "));
    query.push_str(" WHERE id = $5 RETURNING id, nombre, categoria, precio, cantidad");

    params.push(&id);

    match sqlx::query_as::<_, Producto>(&query)
        .bind(params)
        .fetch_one(&state.db)
        .await
    {
        Ok(producto) => HttpResponse::Ok().json(producto),
        Err(_) => HttpResponse::InternalServerError().json("Failed to update producto"),
    }
}

#[put("/clientes/{id}")]
pub async fn update_cliente(
    state: Data<AppState>,
    id: Path<i32>,
    body: Json<UpdateClienteBody>
) -> impl Responder {
    let mut query = "UPDATE clientes SET".to_string();
    let mut params: Vec<&(dyn sqlx::Encode<'_, Postgres> + Sync)> = Vec::new();
    let mut set_clauses: Vec<String> = Vec::new();

    if let Some(nombre) = &body.nombre {
        set_clauses.push("nombre = $1".to_string());
        params.push(&nombre);
    }
    if let Some(telefono) = &body.telefono {
        set_clauses.push("telefono = $2".to_string());
        params.push(&telefono);
    }
    if let Some(presupuesto) = body.presupuesto {
        set_clauses.push("presupuesto = $3".to_string());
        params.push(&presupuesto);
    }

    if set_clauses.is_empty() {
        return HttpResponse::BadRequest().json("No fields to update");
    }

    query.push_str(&set_clauses.join(", "));
    query.push_str(" WHERE id = $4 RETURNING id, nombre, telefono, presupuesto");

    params.push(&id);

    match sqlx::query_as::<_, Cliente>(&query)
        .bind(params)
        .fetch_one(&state.db)
        .await
    {
        Ok(cliente) => HttpResponse::Ok().json(cliente),
        Err(_) => HttpResponse::InternalServerError().json("Failed to update cliente"),
    }
}

#[delete("/productos/{id}")]
pub async fn delete_producto(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    match sqlx::query("DELETE FROM productos WHERE id = $1")
        .bind(id.into_inner())
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Producto deleted"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete producto"),
    }
}

#[delete("/clientes/{id}")]
pub async fn delete_cliente(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    match sqlx::query("DELETE FROM clientes WHERE id = $1")
        .bind(id.into_inner())
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Cliente deleted"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete cliente"),
    }
}
