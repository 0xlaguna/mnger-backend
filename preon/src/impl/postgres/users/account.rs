use crate::models::user::{
    self, 
    Entity as UserEntity, 
    Model as UserModel,
};
use crate::models::session::{
    self, ActiveModel as SessionActiveModel, Entity as SessonEntity, Model as SessionModel
};
use sea_orm::*;
use nanoid::nanoid;

use crate::{Result, Error};

pub struct AbstractAccount;

impl AbstractAccount {
    /// Create a new user Session
    pub async fn create_session(db: &DbConn, name: &str, user_id: i32) -> Result<SessionModel> {
        let session = SessionActiveModel {
            id: NotSet,
            name: Set(name.to_string()),
            token: Set(nanoid!(64)),
            user_id: Set(user_id)
        };

        let session = session
            .insert(db)
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "create_session", 
                with: "sessions",
                info: e.to_string()
            })?;
        
        Ok(session)
    }

    /// Find session by token
    pub async fn find_session_by_token(db: &DbConn, token: String) -> Result<SessionModel> {
        let session = SessonEntity::find()
            .filter(session::Column::Token.eq(token))
            .one(db)
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "find_one", 
                with: "sessions",
                info: e.to_string()
            })?
            .ok_or(Error::InvalidSession)?;
        
        Ok(session)
    }

    pub async fn find_by_email(db: &DbConn, email: &str) -> Result<UserModel> {
        let account = UserEntity
            ::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "find_one", 
                with: "sessions",
                info: e.to_string()
            })?
            .ok_or(Error::NotFound)?;
        
        Ok(account)
    }

    /// Login account
    pub async fn login(db: &DbConn, email: &str, password: &str, name: Option<&str>) -> Result<SessionModel> {
        let account = AbstractAccount::find_by_email(db, email).await?;
        
        let is_valid_password = argon2::verify_encoded(&account.password, password.as_bytes())
            .map_err(|_| Error::InvalidCredentials)?;

        if !is_valid_password {
            return Err(Error::InvalidCredentials);
        }

        let name = name.unwrap_or("Unknown");

        let session = AbstractAccount
            ::create_session(db, name, account.id).await?;

        Ok(session)
    }
}
