use crate::models::user::{
    self, 
    Entity as UserEntity, 
    Model as UserModel,
    ActiveModel as UserActiveModel
};
use sea_orm::*;

use crate::{Result, Error, auth::util::hash_password};

pub struct AbstractUser;

impl AbstractUser {
    pub async fn fetch_user(db: &DbConn, id: i32) -> Result<UserModel> {
        let user = UserEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "find_one", 
                with: "sessions",
                info: e.to_string()
            })?
            .ok_or(Error::NotFound)?;

        Ok(user)
    }

    /// Create a new user
    pub async fn create_user(
        db: &DbConn,
        email: String,
        password: String,
        first_name: String,
        middle_name: String,
        last_name: String
    ) -> Result<UserModel> {

        let password = hash_password(password)?;

        let user = UserActiveModel {
            id: NotSet,
            username: NotSet,
            email: Set(email),
            password: Set(password),
            first_name: Set(first_name),
            middle_name: Set(middle_name),
            last_name: Set(last_name),
            disabled: Set(false)
        };

        let user = user
            .insert(db)
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "create_user", 
                with: "sessions",
                info: e.to_string()
            })?;

        Ok(user)
    }

    pub async fn find_users_in_page(
        db: &DbConn,
        page: u64,
        users_per_page: u64,
    ) -> Result<(Vec<UserModel>, u64)> {
        // Setup paginator
        let paginator = UserEntity::find()
            .order_by_asc(user::Column::Id)
            .paginate(db, users_per_page);

        let num_pages = paginator
            .num_pages()
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "find_users_num_pages", 
                with: "sessions",
                info: e.to_string()
            })?;

        let users = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "find_users_in_page", 
                with: "sessions",
                info: e.to_string()
            })?;
        
        Ok((users, num_pages))
    }

}

// use ::entity::{user, user::Entity as User};
// use sea_orm::*;

// pub struct Mutation;

// impl Mutation {
//     pub async fn create_user(
//         db: &DbConn,
//         form_data: user::Model,
//     ) -> Result<user::ActiveModel, DbErr> {
//         user::ActiveModel {
//             username: Set(form_data.username.to_owned()),
//             ..Default::default()
//         }
//         .save(db)
//         .await
//     }

//     pub async fn update_user_by_id(
//         db: &DbConn,
//         id: i32,
//         form_data: user::Model,
//     ) -> Result<user::Model, DbErr> {
//         let user: user::ActiveModel = User::find_by_id(id)
//             .one(db)
//             .await?
//             .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
//             .map(Into::into)?;

//         user::ActiveModel {
//             id: user.id,
//             username: Set(form_data.username.to_owned()),
//         }
//         .update(db)
//         .await
//     }

//     pub async fn delete_user(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
//         let user: user::ActiveModel = User::find_by_id(id)
//             .one(db)
//             .await?
//             .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
//             .map(Into::into)?;

//         user.delete(db).await
//     }

//     pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
//         User::delete_many().exec(db).await
//     }
// }