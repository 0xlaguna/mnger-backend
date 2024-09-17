use sea_orm_migration::prelude::*;

use crate::authorization::model::{
    Role, 
    Permission, 
    RolePermission,
    Module,
    RoleModule
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Roles table
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Role::Name).string())
                    .col(ColumnDef::new(Role::Description).string())
                    .to_owned()
            ).await?;
        
        // Permission table
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Permission::Name).string())
                    .col(ColumnDef::new(Permission::Description).string())
                    .to_owned()
            ).await?;
        
        // Role-Permission table
        manager
            .create_table(
                Table::create()
                    .table(RolePermission::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RolePermission::RoleId).integer())
                    .col(ColumnDef::new(RolePermission::PermissionId).integer())
                    .to_owned()
            ).await?;
        
        // Composite primary key RolePermission(RoleId, PermissionId)
        manager
            .get_connection()
            .execute_unprepared(
                "
                ALTER TABLE role_permission
                ADD PRIMARY KEY (role_id, permission_id)
                "
            ).await?;

        // Role-Permission FK_role_permission_role_id
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_role_permission_role_id")
                    .from(RolePermission::Table, RolePermission::RoleId)
                    .to(Role::Table, Role::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;

        // Role-Permission FK_role_permission_permission_id
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_role_permission_permission_id")
                    .from(RolePermission::Table, RolePermission::PermissionId)
                    .to(Permission::Table, Permission::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;
        
        // Module table
        manager
            .create_table(
                Table::create()
                    .table(Module::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Module::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Module::Name).string().not_null())
                    .col(ColumnDef::new(Module::Description).string())
                    .to_owned()
            ).await?;

        // RoleModule table
        manager
            .create_table(
                Table::create()
                    .table(RoleModule::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RoleModule::RoleId).integer())
                    .col(ColumnDef::new(RoleModule::ModuleId).integer())
                    .to_owned()
            ).await?;

        // Composite primary key RoleModule(RoleId, ModuleId)
        manager
            .get_connection()
            .execute_unprepared(
                "
                ALTER TABLE role_module
                ADD PRIMARY KEY (role_id, module_id)
                "
            ).await?;

        // Role-Module FK_role_module_role_id
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_role_module_role_id")
                    .from(RoleModule::Table, RoleModule::RoleId)
                    .to(Role::Table, Role::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;

        // Role-Module FK_role_module_module_id
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_role_module_module_id")
                    .from(RoleModule::Table, RoleModule::ModuleId)
                    .to(Module::Table, Module::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop Role table
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;
        
        // Drop permission table
        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await?;

        // Drop RolePermission table
        manager.drop_foreign_key(
            ForeignKey::drop()
                .name("FK_role_permission_role_id")
                .table(RolePermission::Table)
                .to_owned()
        ).await?;

        manager.drop_foreign_key(
            ForeignKey::drop()
                .name("FK_role_permission_permission_id")
                .table(RolePermission::Table)
                .to_owned()
        ).await?;
        
        manager
            .drop_table(Table::drop().table(RolePermission::Table).to_owned())
            .await?;

        // Drop module table

        manager
            .drop_table(Table::drop().table(Module::Table).to_owned())
            .await?;

        // Drop RoleModule tabl
        manager.drop_foreign_key(
            ForeignKey::drop()
                .name("FK_role_module_role_id")
                .table(RoleModule::Table)
                .to_owned()
        ).await?;

        manager.drop_foreign_key(
            ForeignKey::drop()
                .name("FK_role_module_module_id")
                .table(RoleModule::Table)
                .to_owned()
        ).await?;

        manager
            .drop_table(Table::drop().table(RoleModule::Table).to_owned())
            .await?;

        Ok(())
    }
}
