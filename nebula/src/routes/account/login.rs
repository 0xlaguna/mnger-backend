use sea_orm_rocket::Connection;
use rocket::serde::{Serialize, Deserialize, json::Json};

use mnger_preon::Result;

use mnger_preon::r#impl::postgres::users::account::AbstractAccount;
use mnger_preon::models::Session;

use mnger_preon::r#impl::postgres::pool::Db;

/// # Account Data
#[derive(Serialize, Deserialize)]
pub struct DataLoginAccount {
    /// Valid email address
    pub email: String,

    /// Password
    pub password: String,

    /// Session name
    pub name: Option<String>,

}

/// # Login user account
///
///
#[post("/login", data = "<data>")]
pub async fn req(conn: Connection<'_, Db>, data: Json<DataLoginAccount>) -> Result<Json<Session>> {
    let db = conn.into_inner();

    let data = data.into_inner();

    let session = AbstractAccount
        ::login(
            db, 
            data.email, 
            data.password, 
            data.name
        ).await?;
    
    Ok(Json(session))
}
