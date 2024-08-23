use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240821_141300_create_profile::Profile;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id))
                    .col(string(Post::Title))
                    .col(string(Post::Text))
                    .col(integer(Post::ProfileId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-profile")
                            .from(Post::Table, Post::ProfileId)
                            .to(Profile::Table, Profile::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Post {
    Table,
    Id,
    Title,
    Text,
    ProfileId
}
