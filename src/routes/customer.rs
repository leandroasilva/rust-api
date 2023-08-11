use crate::models::customer::{CustomerModel, CreateCustomerSchema, UpdateCustomerSchema};
use crate::AppState;

use actix_web::{get, post, put, web, HttpResponse, Responder, delete};
use chrono::Utc;
use serde_json::json;

#[get("/customers")]
pub async fn get_customers(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        CustomerModel,
        "select * from customers"
    )
    .fetch_all(&data.db)
    .await;
    
    if query_result.is_err(){
        let message: &str = "Something bad happened while fetching the customers";
        return HttpResponse::InternalServerError()
        .json(json!({"status": "error", "message": message}))
    }
    
    let customers = query_result.unwrap();
    
    let json_response = serde_json::json!({
        "status": "success",
        "no. customers": customers.len(),
        "customers": customers
    })
    
    HttpResponse::Ok().json(json_response)
}

#[post("/customers/customer")]
pub async fn create_customer(body: web::Json<CreateCustomerSchema>, data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        CustomerModel,
        "INSERT into customers (name, email, password, birthday) values ($1, $2, $3, $4) returning *",
        body.name.to_string(),
        body.email.to_string(),
        body.password.to_string(),
        body.birthday.to_string()
    ).fetch_one(&data.db)
    .await;

    match query_result {
        Ok(customer) => {
            let customer_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "customer": customer
            })});
            return HttpResponse::Ok().json(customer_response);
        }
        Err(e) => {
            if e.to_string().contains("duplicate key value violates unique constraint") {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail", "message": "Duplicate Key"}))
            }
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)}));
        }
    }
}

#[get("/customers/customer/{id}")]
pub async fn get_customer_by_id(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let customer_id = path.into_inner();
    let query_result = sqlx::query_as!(CustomerModel, "SELECT * FROM customers WHERE id = $1", customer_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(customer) => {
            let customer_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "customer": customer
            })});
            return HttpResponse::Ok().json(customer_response);
        }
        Err(_) => {
            let message = format!("Customer with ID: {} not found", customer_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }
}

#[put("/customers/customer/{id}")]
pub async fn update_customer(path: web::Path<uuid::Uuid>, data: web::Data<AppState>, body: web::Json<UpdateCustomerSchema>) -> impl Responder {
    let customer_id = path.into_inner();
    // make sure customer exists before updating
    let query_result = sqlx::query_as!(CustomerModel, "SELECT * FROM customers where id = $1", customer_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("Customer with ID: {} not found", customer_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let now = Utc::now();
    let name = query_result.unwrap();

    let query_result = sqlx::query_as!(
        CustomerModel,
        "UPDATE customers set name = $1, email = $2, passwrod = $3, birthday = $4, updated_at = $5 where id = $6 returning *",
        body.name.to_owned().unwrap_or(customer.name),
        body.email.to_owned().unwrap_or(customer.email),
        body.password.to_owned().unwrap_or(customer.password),
        body.birthday.to_owned().unwrap_or(customer.birthday),
        now,
        customer_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(customer) => {
            let customer_response = serde_json::json!({"state": "success", "data": serde_json::json!({
                "customer": customer
            })});
            return HttpResponse::Ok().json(customer_response);
        }
        Err (_) => {
            let message = format!("Customer with ID: {} not found", customer_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}))
        }
    }
}

#[delete("/customers/customer/{id}")]
pub async fn delete_customer(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let customer_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE from customers WHERE id = $1", customer_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("Customer with ID: {} not found", customer_id);
        return HttpResponse::NotFound().json(json!({"status": "fail", "message": message}))
    }
    HttpResponse::NoContent().finish()
}