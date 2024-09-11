use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240821_141200_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(pk_auto(Profile::Id))
                    .col(string(Profile::Name))
                    .col(string(Profile::Bio))
                    .col(integer(Profile::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-profile-user")
                            .from(Profile::Table, Profile::UserId)
                            .to(User::Table, User::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Profile {
    Table,
    Id,
    Name,
    Bio,
    UserId,
}
