use sea_orm_migration::prelude::*;

use crate::user::model::{User, Session};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .get_connection()
            .execute_unprepared(
                "
                    CREATE TABLE company
                    (
                        id              typeid                      default typeid_generate('company')      not null    primary key,
                        name            varchar                                                             not null,
                        address         text,
                        phone_number    varchar,
                        email           varchar,
                        logo            varchar,
                        website         varchar,
                        created_at      timestamp with time zone    default now(),
                        updated_at      timestamp with time zone
                    )
                "
            ).await?;

        manager
            .get_connection()
            .execute_unprepared(
                "
                    CREATE TABLE \"user\"
                    (
                        id              typeid                      default typeid_generate('user')     not null    primary key,
                        username        varchar,
                        email           varchar                                                         not null    unique,
                        first_name      varchar                                                         not null,
                        middle_name     varchar,
                        last_name       varchar,
                        dob             date,
                        avatar          varchar,
                        timezone        varchar,
                        password        varchar                                                         not null,
                        enabled         bool                                                            not null,
                        company_id      typeid
                            references company
                            on delete set null,
                        created_at      timestamp with time zone    default now()                       not null,
                        updated_at      timestamp with time zone
                    )
                "
            ).await?;

        manager
            .get_connection()
            .execute_unprepared(
                "
                    CREATE TABLE session
                    (
                        id              typeid                      default typeid_generate('session')      not null    primary key,
                        token           varchar                                                             not null,
                        name            varchar,
                        expires_at      timestamp with time zone                                            not null,
                        user_id         typeid                                                              not null
                            references \"user\"
                            on delete cascade
                    )
                "
            ).await?;

        manager
            .get_connection()
            .execute_unprepared(
                "
                    CREATE TABLE team
                    (
                        id              typeid                      default typeid_generate('team')     not null   primary key,
                        name            varchar                                                         not null   unique,
                        description     text,
                        created_by      typeid
                            references \"user\",
                        created_at      timestamp with time zone    default now()                       not null,
                        updated_at      timestamp with time zone
                    )
                "
            ).await?;

        manager
            .get_connection()
            .execute_unprepared(
                "
                    CREATE TABLE team_participant
                    (
                        id              bigserial                                                                       primary key,
                        team_id         typeid                                                          not null
                            references team,
                        user_id         typeid                                                          not null
                            references \"user\",
                        joined_at       timestamp with time zone    default now()                       not null
                    )
                "
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
