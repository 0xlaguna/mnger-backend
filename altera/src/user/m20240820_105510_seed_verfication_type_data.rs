use sea_orm_migration::prelude::*;

use crate::user::model::VerificationType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(VerificationType::Table)
            .columns([
                VerificationType::Id,
                VerificationType::Name,
                VerificationType::Enabled,
            ])
            .values_panic([1.into(), "Email".into(), true.into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(VerificationType::Table)
            .and_where(Expr::col(VerificationType::Id).eq(1))
            .to_owned();

        manager.exec_stmt(delete).await?;

        Ok(())
    }
}
