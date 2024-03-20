use crate::{models::Session as Session, Error};
use rocket::{http::Status, outcome::Outcome, request::{self, FromRequest}, Request};

use crate::r#impl::postgres::users::account::AbstractAccount;
use crate::r#impl::postgres::pool::Db;
use sea_orm_rocket::Connection;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> { 
        let header_session_token = request
            .headers()
            .get("x-session-token")
            .next()
            .map(|x| x.to_string());

        let db_conn = request.guard::<Connection<'_, Db>>()
            .await
            .expect("Database connection").into_inner();

        if let Some(session_token) = header_session_token {
            match AbstractAccount::find_session_by_token(db_conn, session_token).await {
                Ok(session) => Outcome::Success(session),
                Err(_) => Outcome::Error((Status::Unauthorized, Error::InvalidSession)),
            }
        } else {
            Outcome::Error((Status::Unauthorized, Error::MissingHeaders))
        }
    }
}
