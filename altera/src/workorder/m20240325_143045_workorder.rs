use sea_orm_migration::prelude::*;

use crate::workorder::model::{WorkOrder, WorkOrderAssignment};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "
                    CREATE TABLE work_order
                    (
                        id              text                      default typeid_generate_text('workorder')    not null    primary key,
                        title           varchar                                                             not null,
                        description     varchar,
                        status          smallint,
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

        Ok(())
    }
}
