use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create contact table
        manager
            .create_table(
                Table::create()
                    .table(Contact::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Contact::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .comment("Contact unique identifier"),
                    )
                    .col(
                        ColumnDef::new(Contact::FirstName)
                            .text()
                            .not_null()
                            .comment("First name"),
                    )
                    .col(
                        ColumnDef::new(Contact::LastName)
                            .text()
                            .not_null()
                            .comment("Last name"),
                    )
                    .col(
                        ColumnDef::new(Contact::Email)
                            .text()
                            .comment("Primary email address"),
                    )
                    .col(
                        ColumnDef::new(Contact::Phone)
                            .text()
                            .comment("Primary phone number"),
                    )
                    .col(
                        ColumnDef::new(Contact::Mobile)
                            .text()
                            .comment("Mobile phone number"),
                    )
                    .col(
                        ColumnDef::new(Contact::IsActive)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("Is contact active"),
                    )
                    .col(
                        ColumnDef::new(Contact::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .comment("Creation timestamp"),
                    )
                    .col(
                        ColumnDef::new(Contact::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .comment("Last update timestamp"),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for email lookup
        manager
            .create_index(
                Index::create()
                    .name("idx_contact_email")
                    .table(Contact::Table)
                    .col(Contact::Email)
                    .to_owned(),
            )
            .await?;

        // Index for name search
        manager
            .create_index(
                Index::create()
                    .name("idx_contact_name")
                    .table(Contact::Table)
                    .col(Contact::FirstName)
                    .col(Contact::LastName)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contact::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Contact {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Phone,
    Mobile,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
