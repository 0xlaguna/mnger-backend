use crate::models::{user, user::Entity as User};
use sea_orm::*;

use crate::{Result, Error};

pub struct AbstractUser;

impl AbstractUser {
    pub async fn fetch_user(db: &DbConn, id: i32) -> Result<user::Model> {
        let user = User::find_by_id(id)
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

    /// If ok, returns (user models, num pages).
    pub async fn find_users_in_page(
        db: &DbConn,
        page: u64,
        users_per_page: u64,
    ) -> Result<(Vec<user::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = User::find()
            .order_by_asc(user::Column::Id)
            .paginate(db, users_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated users
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
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