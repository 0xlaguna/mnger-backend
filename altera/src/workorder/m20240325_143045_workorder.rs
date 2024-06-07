use sea_orm_migration::prelude::*;

use crate::workorder::model::WorkOrder;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(WorkOrder::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkOrder::Id)
                            .string()
                            .extra("DEFAULT generate_ulid()")
                            .primary_key()
                            .not_null()
                    )
                    .col(ColumnDef::new(WorkOrder::Title).string().not_null())
                    .col(ColumnDef::new(WorkOrder::Description).string())
                    .col(ColumnDef::new(WorkOrder::Status).small_integer().not_null())
                    .col(ColumnDef::new(WorkOrder::StartDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(WorkOrder::EndDate).timestamp_with_time_zone())
                    .col(ColumnDef::new(WorkOrder::CreatedBy).integer().not_null())
                    .col(
                        ColumnDef::new(WorkOrder::CreatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT now()")
                            .not_null()
                    )
                    .col(ColumnDef::new(WorkOrder::UpdatedAt).timestamp_with_time_zone())
                    .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop().table(WorkOrder::Table).to_owned()
            )
            .await?;

        Ok(())
    }
}
