use std::{io::Write, path};

use crate::{dto::users::DataEditUser, r#impl::storage::s3::{Opt, S3}, models::user::{
    self, ActiveModel as UserActiveModel, Entity as UserEntity, Model as UserModel
}};
use sea_orm::*;

use ulid::Ulid;
use tempfile::NamedTempFile;

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

    pub async fn update_user(
        db: &DbConn,
        user_id: i32,
        data: DataEditUser<'_>
    ) -> Result<()> {
        
        // Update use file avatar
        let mut temp_file = NamedTempFile::new()
            .map_err(|e| Error::InternalError { info: e.to_string()})?;

        let _ = temp_file.write_all(data.avatar.data);
        let temp_file_path = temp_file.path();

        let mut buf = [0; ulid::ULID_LEN];
        let filename = Ulid::new().array_to_str(&mut buf);

        let file_key = format!(
            "user-images/{}.{}", 
            filename, 
            data.avatar.extension
        );

        S3::put_object(&Opt { key: file_key, source: temp_file_path.to_path_buf() }).await?;

        Ok(())
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