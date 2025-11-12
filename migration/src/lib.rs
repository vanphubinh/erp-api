pub use sea_orm_migration::prelude::*;

mod m20251112_091615_create_organization_table;
mod m20251112_091616_create_contact_table;
mod m20251112_091617_create_organization_contact_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251112_091615_create_organization_table::Migration),
            Box::new(m20251112_091616_create_contact_table::Migration),
            Box::new(m20251112_091617_create_organization_contact_table::Migration),
        ]
    }
}
