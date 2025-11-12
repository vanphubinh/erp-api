use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organization::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Organization::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .comment("Organization unique identifier"),
                    )
                    .col(
                        ColumnDef::new(Organization::Name)
                            .text()
                            .not_null()
                            .comment("Organization name"),
                    )
                    .col(
                        ColumnDef::new(Organization::Email)
                            .text()
                            .comment("Primary email"),
                    )
                    .col(
                        ColumnDef::new(Organization::Phone)
                            .text()
                            .comment("Primary phone number"),
                    )
                    .col(
                        ColumnDef::new(Organization::Website)
                            .text()
                            .comment("Company website URL"),
                    )
                    .col(
                        ColumnDef::new(Organization::Industry)
                            .text()
                            .comment("Industry type"),
                    )
                    .col(
                        ColumnDef::new(Organization::Address)
                            .text()
                            .comment("Street address"),
                    )
                    .col(ColumnDef::new(Organization::City).text().comment("City"))
                    .col(
                        ColumnDef::new(Organization::State)
                            .text()
                            .comment("State/Province"),
                    )
                    .col(
                        ColumnDef::new(Organization::PostalCode)
                            .text()
                            .comment("Postal/ZIP code"),
                    )
                    .col(
                        ColumnDef::new(Organization::CountryCode)
                            .text()
                            .comment("ISO 3166-1 alpha-2 country code (e.g., US, GB, TH)"),
                    )
                    .col(
                        ColumnDef::new(Organization::Timezone)
                            .text()
                            .comment("IANA timezone (e.g., America/New_York, Asia/Bangkok)"),
                    )
                    .col(
                        ColumnDef::new(Organization::Currency)
                            .text()
                            .comment("ISO 4217 currency code (e.g., USD, EUR, THB)"),
                    )
                    .col(
                        ColumnDef::new(Organization::IsActive)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("Is organization active"),
                    )
                    .col(
                        ColumnDef::new(Organization::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .comment("Creation timestamp"),
                    )
                    .col(
                        ColumnDef::new(Organization::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .comment("Last update timestamp"),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for searching by name
        manager
            .create_index(
                Index::create()
                    .name("idx_organization_name")
                    .table(Organization::Table)
                    .col(Organization::Name)
                    .to_owned(),
            )
            .await?;

        // Index for filtering by country
        manager
            .create_index(
                Index::create()
                    .name("idx_organization_country")
                    .table(Organization::Table)
                    .col(Organization::CountryCode)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Organization::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Organization {
    Table,
    Id,
    Name,
    Email,
    Phone,
    Website,
    Industry,
    Address,
    City,
    State,
    PostalCode,
    CountryCode,
    Timezone,
    Currency,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
