#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CustomerModel {
    pub id:Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub birthday: String,
    pub created_at: Option<chrono::Datetime<chrono::Utc>>,
    pub updated_at: Option<chrono::Datetime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCustomerSchema {
    pub name: String,
    pub email: String,
    pub password: String,
    pub birthday: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCustomerSchema {
    pub name: String,
    pub email: String,
    pub password: String,
    pub birthday: String,
}