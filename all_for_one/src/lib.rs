pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250730_221445_create_user::Migration),
            Box::new(m20250805_230357_add_idp_column_in_user_table::Migration),
        ]
    }
}
mod m20250730_221445_create_user;
mod m20250805_230357_add_idp_column_in_user_table;
