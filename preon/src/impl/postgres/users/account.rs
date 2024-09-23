use crate::models::session::{
    self, ActiveModel as SessionActiveModel, Entity as SessonEntity, Model as SessionModel,
};
use crate::models::user::{self, Entity as UserEntity, Model as UserModel};
use chrono::{Duration, FixedOffset, Utc};
use nanoid::nanoid;
use sea_orm::*;

use crate::{Error, Result};

pub struct AbstractAccount;

impl AbstractAccount {
    /// Create a new user Session
    pub async fn create_session(
        db: &DbConn,
        _name: Option<&str>,
        user_id: String,
    ) -> Result<SessionModel> {
        let fixed_now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
        let expires_at = fixed_now + Duration::days(3);

        let session = SessionActiveModel {
            id: NotSet,
            name: NotSet,
            token: Set(nanoid!(64)),
            user_id: Set(user_id),
            expires_at: Set(expires_at),
        };

        let session = session.insert(db).await?;

        Ok(session)
    }

    /// Find session by token
    pub async fn find_session_by_token(db: &DbConn, token: String) -> Result<SessionModel> {
        let session = SessonEntity::find()
            .filter(session::Column::Token.eq(token))
            .one(db)
            .await?
            .ok_or(Error::InvalidSession)?;

        Ok(session)
    }

    pub async fn find_by_email(db: &DbConn, email: &str) -> Result<UserModel> {
        let account = UserEntity::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await?
            .ok_or(Error::NotFound)?;

        Ok(account)
    }

    /// Login account
    pub async fn login(
        db: &DbConn,
        email: &str,
        password: &str,
        name: Option<&str>,
    ) -> Result<SessionModel> {
        let account = AbstractAccount::find_by_email(db, email).await?;

        let is_valid_password = argon2::verify_encoded(&account.password, password.as_bytes())
            .map_err(|_| Error::InvalidCredentials)?;

        if !is_valid_password {
            return Err(Error::InvalidCredentials);
        }

        let session = AbstractAccount::create_session(db, name, account.id).await?;

        Ok(session)
    }
}
