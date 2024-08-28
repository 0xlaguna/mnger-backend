use std::io::Write;

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
        middle_name: Option<String>,
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
            disabled: Set(false),
            avatar: NotSet
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
    ) -> Result<UserModel> {
        
        let user = UserEntity::find_by_id(user_id)
            .one(db)
            .await
            .map_err(|e: DbErr| Error::DatabaseError { 
                operation: "find_user", 
                with: "sessions",
                info: e.to_string()
            })?;

        // Delete previous avatar if any
        if let Some(old_avatar) = user.clone().unwrap().avatar {
            S3::delete_object(old_avatar).await?;
        }

        let mut user: UserActiveModel = user.ok_or(Error::NotFound)?.into();
        
        user.first_name = Set(data.first_name);
        user.middle_name = Set(data.middle_name);
        user.last_name = Set(data.last_name);

        // Update user file avatar
        if let Some(avatar) = data.avatar {
            let mut avatar_temp_file = NamedTempFile::new()
                .map_err(|e| Error::InternalError { info: e.to_string()})?;

            let _ = avatar_temp_file.write_all(avatar.data);

            let mut buf = [0; ulid::ULID_LEN];
            let filename = Ulid::new().array_to_str(&mut buf);

            let file_key = format!(
                "user-images/{}.{}", 
                filename, 
                avatar.extension
            );

            // Upload image to s3
            S3::put_object(&Opt { key: file_key.clone(), source: avatar_temp_file.path().to_path_buf() }).await?;

            user.avatar = Set(Some(file_key));
        };

        let user = user.update(db)
            .await
            .map_err(|e: DbErr| Error::DatabaseError { 
                operation: "update_user", 
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
