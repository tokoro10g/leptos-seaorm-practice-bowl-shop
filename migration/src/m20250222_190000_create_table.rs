use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Bowl table
        manager
            .create_table(
                Table::create()
                    .table(Bowl::Table)
                    .if_not_exists()
                    .col(pk_auto(Bowl::Id))
                    .col(string(Bowl::Name).not_null())
                    .col(decimal(Bowl::Price).not_null())
                    .to_owned(),
            )
            .await?;

        // Create Customization table with foreign key reference to Bowl
        manager
            .create_table(
                Table::create()
                    .table(Customization::Table)
                    .if_not_exists()
                    .col(pk_auto(Customization::Id))
                    .col(integer(Customization::BowlId).not_null())
                    .col(string(Customization::Name).not_null())
                    .col(decimal(Customization::Price).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_customization_bowl")
                            .from(Customization::Table, Customization::BowlId)
                            .to(Bowl::Table, Bowl::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager
            .drop_table(Table::drop().table(Customization::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Bowl::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Bowl {
    Table,
    Id,
    Name,
    Price,
}

#[derive(DeriveIden)]
enum Customization {
    Table,
    Id,
    BowlId,
    Name,
    Price,
}
