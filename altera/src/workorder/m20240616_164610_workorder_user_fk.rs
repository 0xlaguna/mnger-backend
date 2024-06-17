use sea_orm_migration::prelude::*;

use crate::workorder::model::WorkOrder;
use crate::user::model::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration { 
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> { 
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_work_order_created_by_user_id")
                    .from(WorkOrder::Table, WorkOrder::CreatedBy)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> { 
        
        manager.drop_foreign_key(
            ForeignKey::drop()
                .name("FK_work_order_created_by_user_id")
                .table(WorkOrder::Table)
                .to_owned()
        ).await?;

        Ok(())
    }
}
