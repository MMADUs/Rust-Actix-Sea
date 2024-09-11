pub use sea_orm_migration::prelude::*;

mod m20240821_141400_post;
mod m20240821_141300_profile;
mod m20240821_141200_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240821_141200_user::Migration),
            Box::new(m20240821_141300_profile::Migration),
            Box::new(m20240821_141400_post::Migration),
        ]
    }
}
