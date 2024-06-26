# Oxidetalis database migrations

This crate contains the database migrations for the Oxidetalis homeserver, using
SeaORM.

## How to run the migrations
The migrations are run when the server starts. The server will check if the
database is up-to-date and run the migrations if needed. So, you don't need to
run the migrations manually.

## How to create a new migration
The migrations will saved in the database, so SeaORM will track the migrations,
and you don't need to worry about the migration files, just write the migration
and SeaORM will take care of the rest.

To create a new migration, you need to create a new migration file in the `src`
directory. You can name the file anything you want, for example,
`create_users_table.rs`. The file should contain the migration code, you can
take this as a template:
```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Here you can write the migration code, the `manager` can do anything you want.
        
        // When the homeserver starts, it will run the `up` function for each migration that is not run yet.
    }
}

#[derive(DeriveIden)]
enum TableName {
    Table, // Required for the table name
    Id, // Required for the primary key
    // Add more columns here
d}
```

> [!NOTE] Don't write the `down` function, I prefer to do each migration in a
> separate migration file, so you don't need to write the `down` function. If you
> want to delete a table later, you can create a new migration file that deletes
> the table.

After you write the migration code, you need to add the migration to the
`src/lib.rs` file.

## License
This crate is licensed under the MIT license.
