use sea_orm_migration::prelude::*;

use crate::user::model::{User, Verification, VerificationType};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::Avatar).string())
                    .to_owned()
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Verification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Verification::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                    )
                    .col(ColumnDef::new(Verification::UserId).integer().not_null())
                    .col(ColumnDef::new(Verification::TypeId).integer().not_null())
                    .col(ColumnDef::new(Verification::Token).string())
                    .col(ColumnDef::new(Verification::Pending).boolean())
                    .col(ColumnDef::new(Verification::ExpiresAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Verification::Enabled).boolean())
                    .to_owned()
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(VerificationType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(VerificationType::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                    )
                    .col(ColumnDef::new(VerificationType::Name).string().not_null())
                    .col(ColumnDef::new(VerificationType::Description).string())
                    .col(ColumnDef::new(VerificationType::Enabled).boolean())
                    .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(Alias::new("avatar"))
                    .to_owned()
            ).await?;

        manager
            .drop_table(
                Table::drop().table(Verification::Table).to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop().table(VerificationType::Table).to_owned()
            )
            .await?;

        Ok(())
    }
}
