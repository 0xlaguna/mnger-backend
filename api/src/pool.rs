use mnger_api_service::sea_orm;

use async_trait::async_trait;
use revolt_rocket_okapi::gen::OpenApiGenerator;
use sea_orm::ConnectOptions;
use sea_orm_rocket::{rocket::figment::Figment, Config, Database, Connection};
use std::time::Duration;

use revolt_rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

#[derive(Database, Debug)]
#[database("sea_orm")]
pub struct Db(SeaOrmPool);

#[derive(Debug, Clone)]
pub struct SeaOrmPool {
    pub conn: sea_orm::DatabaseConnection,
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config = figment.extract::<Config>().unwrap();
        let mut options: ConnectOptions = config.url.into();
        options
            .max_connections(config.max_connections as u32)
            .min_connections(config.min_connections.unwrap_or_default())
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .sqlx_logging(config.sqlx_logging);
        if let Some(idle_timeout) = config.idle_timeout {
            options.idle_timeout(Duration::from_secs(idle_timeout));
        }
        let conn = sea_orm::Database::connect(options).await?;

        Ok(SeaOrmPool { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}


impl<'r, D: Database> OpenApiFromRequest<'r> for Connection<'r, D> {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> revolt_rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }

    fn get_responses(_gen: &mut revolt_rocket_okapi::gen::OpenApiGenerator) -> revolt_rocket_okapi::Result<revolt_okapi::openapi3::Responses> {
        Ok(revolt_okapi::openapi3::Responses::default())
    }
}
