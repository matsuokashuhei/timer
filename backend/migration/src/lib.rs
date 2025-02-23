pub use sea_orm_migration::prelude::*;

mod m20250222_164410_create_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250222_164410_create_users::Migration)]
    }
}

pub fn define_created_at() -> ColumnDef {
    ColumnDef::new(Alias::new("created_at"))
        .date_time()
        .not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".to_owned())
        .clone()
}

pub fn define_updated_at() -> ColumnDef {
    ColumnDef::new(Alias::new("updated_at"))
        .date_time()
        .not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned())
        .clone()
}
