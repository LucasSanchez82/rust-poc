use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{integer, pk_auto, string};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table("user")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("name"))
                    .col(string("email").unique_key())
                    .col(string("password"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("post")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("title"))
                    .col(string("text"))
                    .col(integer("user_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .from("post", "user_id")
                            .to("user", "id"),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table("post").to_owned())
            .await?;
        Ok(())
    }
}
