use sea_orm_migration::prelude::*;

use crate::workorder::model::{WorkOrder, WorkOrderAssignment, WorkOrderPriority, WorkOrderStatus};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkOrderStatus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkOrderStatus::Id)
                            .small_integer()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WorkOrderStatus::Name).string().not_null())
                    .col(ColumnDef::new(WorkOrderStatus::Icon).string())
                    .col(ColumnDef::new(WorkOrderStatus::Color).string().not_null())
                    .col(ColumnDef::new(WorkOrderStatus::Description).text())
                    .col(ColumnDef::new(WorkOrderStatus::Enabled).boolean())
                    .to_owned(),
            )
            .await?;

        // Seed WorkOrder Status
        let insert_wo_statuses = Query::insert()
            .into_table(WorkOrderStatus::Table)
            .columns([
                WorkOrderStatus::Id,
                WorkOrderStatus::Name,
                WorkOrderStatus::Color,
                WorkOrderStatus::Enabled,
            ])
            .values_panic([1.into(), "To Do".into(), "#6C757D".into(), true.into()])
            .values_panic([2.into(), "On Hold".into(), "#FFC107".into(), true.into()])
            .values_panic([
                3.into(),
                "In Progress".into(),
                "#17A2B8".into(),
                true.into(),
            ])
            .values_panic([4.into(), "Canceled".into(), "#DC3545".into(), true.into()])
            .values_panic([5.into(), "Complete".into(), "#28A745".into(), true.into()])
            .values_panic([6.into(), "Scheduled".into(), "#007BFF".into(), true.into()])
            .to_owned();

        manager.exec_stmt(insert_wo_statuses).await?;

        manager
            .create_table(
                Table::create()
                    .table(WorkOrderPriority::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkOrderPriority::Id)
                            .small_integer()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WorkOrderPriority::Name).string().not_null())
                    .col(ColumnDef::new(WorkOrderPriority::Icon).string())
                    .col(ColumnDef::new(WorkOrderPriority::Color).string().not_null())
                    .col(ColumnDef::new(WorkOrderPriority::Description).text())
                    .col(ColumnDef::new(WorkOrderPriority::Enabled).boolean())
                    .to_owned(),
            )
            .await?;

        // Seed WorkOrder Priority
        let insert_wo_priorities = Query::insert()
            .into_table(WorkOrderPriority::Table)
            .columns([
                WorkOrderPriority::Id,
                WorkOrderPriority::Name,
                WorkOrderPriority::Color,
                WorkOrderPriority::Enabled,
            ])
            .values_panic([1.into(), "Low".into(), "#28A745".into(), true.into()])
            .values_panic([2.into(), "Medium".into(), "#FFC107".into(), true.into()])
            .values_panic([3.into(), "High".into(), "#DC3545".into(), true.into()])
            .to_owned();

        manager.exec_stmt(insert_wo_priorities).await?;

        manager
            .get_connection()
            .execute_unprepared(
                "
                    CREATE TABLE work_order
                    (
                        id              text                      default typeid_generate_text('workorder')    not null    primary key,
                        title           varchar                                                             not null,
                        description     varchar,
                        status_id          smallint
                            references work_order_status,
                        priority_id        smallint
                            references work_order_priority,
                        start_date      timestamp with time zone,
                        end_date        timestamp with time zone,
                        created_by      text
                            references \"user\"
                            on delete set null,
                        created_at      timestamp with time zone    default now()                           not null,
                        updated_at      timestamp with time zone
                    )
                "
            ).await?;

        manager
            .get_connection()
            .execute_unprepared(
                "
                    CREATE TABLE work_order_assignment
                    (
                        id              bigserial                                                                       primary key,
                        work_order_id   text                                                              not null
                            references work_order,
                        user_id         text
                            references \"user\",
                        team_id         text
                            references team,
                        assigned_at     timestamp with time zone    default now()
                    )
                "
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorkOrder::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(WorkOrderAssignment::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(WorkOrderStatus::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(WorkOrderPriority::Table).to_owned())
            .await?;

        Ok(())
    }
}
