use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{integer, pk_auto, string, timestamp_with_time_zone, uuid};

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
                    .table("session")
                    .if_not_exists()
                    .col(uuid("token").primary_key())
                    .col(integer("user_id"))
                    .col(timestamp_with_time_zone("expire_at"))
                    .col(timestamp_with_time_zone("created_at"))
                    .foreign_key(
                        ForeignKey::create()
                            .from("session", "user_id")
                            .to("user", "id"),
                    )
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
        manager
            .drop_table(Table::drop().table("post").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("session").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("user").to_owned())
            .await?;
        Ok(())
    }
}
