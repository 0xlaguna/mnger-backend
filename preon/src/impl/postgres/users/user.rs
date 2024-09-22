use std::io::Write;

use crate::{
    dto::users::{DataCreateAccount, DataEditUser},
    models::{
        user::{
            self, ActiveModel as UserActiveModel, Entity as UserEntity, Model as UserModel, UserId,
        },
        verficiation_type::{self, Entity as VerificationTypeEntity},
        verification::{ActiveModel as VerificationActiveModel, Entity as VerificationEntity},
    },
    r#impl::storage::s3::{Opt, S3},
    util::time::Time,
};
use sea_orm::*;

use strong_id::StrongUuid;
use tempfile::NamedTempFile;
use ulid::Ulid;

use crate::{auth::util::hash_password, Error, Result};

pub struct AbstractUser;

impl AbstractUser {
    pub async fn fetch_user(db: &DbConn, id: &str) -> Result<UserModel> {
        let user = UserEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| Error::DatabaseError {
                operation: "find_one",
                with: "sessions",
                info: e.to_string(),
            })?
            .ok_or(Error::NotFound)?;

        Ok(user)
    }

    pub async fn fetch_me(db: &DbConn, id: &str) -> Result<UserModel> {
        let user = UserEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| Error::DatabaseError {
                operation: "find_one",
                with: "sessions",
                info: e.to_string(),
            })?
            .ok_or(Error::NotFound)?;

        Ok(user)
    }

    /// Create a new user
    pub async fn create_user(db: &DbConn, data: DataCreateAccount) -> Result<UserModel> {
        let password = hash_password(data.password)?;

        let user_id = UserId::now_v7().to_string();

        let user = UserActiveModel {
            id: Set(user_id),
            username: NotSet,
            email: Set(data.email),
            password: Set(password),
            first_name: Set(data.first_name),
            middle_name: Set(data.middle_name),
            last_name: Set(data.last_name),
            enabled: Set(true),
            avatar: NotSet,
            company_id: NotSet,
            created_at: Set(Time::now_with_offset()),
            dob: NotSet,
            timezone: NotSet,
            updated_at: NotSet,
        };

        let user = user.insert(db).await.map_err(|e| Error::DatabaseError {
            operation: "create_user",
            with: "sessions",
            info: e.to_string(),
        })?;

        let verification_types = VerificationTypeEntity::find()
            .filter(verficiation_type::Column::Enabled.eq(true))
            .all(db)
            .await
            .map_err(|e| Error::DatabaseError {
                operation: "create_user",
                with: "sessions",
                info: e.to_string(),
            })?;

        let i_verification_types: Vec<VerificationActiveModel> = verification_types
            .into_iter()
            .map(|i| VerificationActiveModel {
                id: NotSet,
                user_id: Set(user.id.clone()),
                type_id: Set(i.id),
                token: NotSet,
                pending: Set(Some(true)),
                expires_at: Set(Some(Time::now_plus_days(3))),
                enabled: Set(Some(true)),
            })
            .collect();

        VerificationEntity::insert_many(i_verification_types)
            .exec(db)
            .await
            .map_err(|e| Error::DatabaseError {
                operation: "create_user",
                with: "sessions",
                info: e.to_string(),
            })?;

        Ok(user)
    }

    pub async fn update_user(
        db: &DbConn,
        user_id: &str,
        data: DataEditUser<'_>,
    ) -> Result<UserModel> {
        let user = UserEntity::find_by_id(user_id)
            .one(db)
            .await
            .map_err(|e: DbErr| Error::DatabaseError {
                operation: "find_user",
                with: "sessions",
                info: e.to_string(),
            })?;

        // Delete previous avatar if any
        if let Some(old_avatar) = user.clone().unwrap().avatar {
            S3::delete_object(old_avatar).await?;
        }

        let mut user: UserActiveModel = user.ok_or(Error::NotFound)?.into();

        if let Some(first_name) = data.first_name {
            user.first_name = Set(first_name)
        }

        if let Some(middle_name) = data.middle_name {
            user.first_name = Set(middle_name)
        }

        if let Some(last_name) = data.last_name {
            user.first_name = Set(last_name)
        }

        // Update user file avatar
        if let Some(avatar) = data.avatar {
            let mut avatar_temp_file = NamedTempFile::new().map_err(|e| Error::InternalError {
                info: e.to_string(),
            })?;

            let _ = avatar_temp_file.write_all(avatar.data);

            let mut buf = [0; ulid::ULID_LEN];
            let filename = Ulid::new().array_to_str(&mut buf);

            let file_key = format!("user-images/{}.{}", filename, avatar.extension);

            // Upload image to s3
            S3::put_object(&Opt {
                key: file_key.clone(),
                source: avatar_temp_file.path().to_path_buf(),
            })
            .await?;

            user.avatar = Set(Some(file_key));
        };

        let user = user
            .update(db)
            .await
            .map_err(|e: DbErr| Error::DatabaseError {
                operation: "update_user",
                with: "sessions",
                info: e.to_string(),
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
                info: e.to_string(),
            })?;

        let users = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| Error::DatabaseError {
                operation: "find_users_in_page",
                with: "sessions",
                info: e.to_string(),
            })?;

        Ok((users, num_pages))
    }
}
