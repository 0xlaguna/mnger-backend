use sea_orm_migration::prelude::*;

use crate::user::model::{User, Session};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string())
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Disabled).boolean().not_null())
                    .to_owned(),
            ).await?;
        
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Session::Token).string())
                    .col(ColumnDef::new(Session::Name).string())
                    .col(ColumnDef::new(Session::UserId).integer())
                    .to_owned(),
            ).await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_session_user_id")
                    .from(Session::Table, Session::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;

        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await?;
        
        Ok(())
    }
}
