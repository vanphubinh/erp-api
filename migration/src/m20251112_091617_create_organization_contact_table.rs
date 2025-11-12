use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create organization_contact junction table
        manager
            .create_table(
                Table::create()
                    .table(OrganizationContact::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrganizationContact::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .comment("Unique identifier"),
                    )
                    .col(
                        ColumnDef::new(OrganizationContact::OrganizationId)
                            .uuid()
                            .not_null()
                            .comment("Organization ID"),
                    )
                    .col(
                        ColumnDef::new(OrganizationContact::ContactId)
                            .uuid()
                            .not_null()
                            .comment("Contact ID"),
                    )
                    .col(
                        ColumnDef::new(OrganizationContact::JobTitle)
                            .text()
                            .comment("Job title/position (e.g., CEO, Sales Manager, Engineer)"),
                    )
                    .col(
                        ColumnDef::new(OrganizationContact::Department)
                            .text()
                            .comment("Department (e.g., Sales, Engineering, Finance)"),
                    )
                    .col(
                        ColumnDef::new(OrganizationContact::Role)
                            .text()
                            .comment("CRM role (e.g., decision_maker, influencer, champion)"),
                    )
                    .col(
                        ColumnDef::new(OrganizationContact::ReportsToId)
                            .uuid()
                            .comment("FK to organization_contact.id - who this person reports to"),
                    )
                    .col(
                        ColumnDef::new(OrganizationContact::IsPrimary)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("Is this the primary contact for the organization?"),
                    )
                    .col(
                        ColumnDef::new(OrganizationContact::IsActive)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("Is this relationship active?"),
                    )
                    .col(
                        ColumnDef::new(OrganizationContact::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .comment("Creation timestamp"),
                    )
                    .col(
                        ColumnDef::new(OrganizationContact::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .comment("Last update timestamp"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_org_contact_organization")
                            .from(
                                OrganizationContact::Table,
                                OrganizationContact::OrganizationId,
                            )
                            .to(Organization::Table, Organization::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_org_contact_contact")
                            .from(OrganizationContact::Table, OrganizationContact::ContactId)
                            .to(Contact::Table, Contact::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_org_contact_reports_to")
                            .from(OrganizationContact::Table, OrganizationContact::ReportsToId)
                            .to(OrganizationContact::Table, OrganizationContact::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint: one contact can only have one active relationship per organization
        manager
            .create_index(
                Index::create()
                    .name("idx_org_contact_unique")
                    .table(OrganizationContact::Table)
                    .col(OrganizationContact::OrganizationId)
                    .col(OrganizationContact::ContactId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index for finding all contacts in an organization
        manager
            .create_index(
                Index::create()
                    .name("idx_org_contact_organization")
                    .table(OrganizationContact::Table)
                    .col(OrganizationContact::OrganizationId)
                    .to_owned(),
            )
            .await?;

        // Index for finding all organizations a contact belongs to
        manager
            .create_index(
                Index::create()
                    .name("idx_org_contact_contact")
                    .table(OrganizationContact::Table)
                    .col(OrganizationContact::ContactId)
                    .to_owned(),
            )
            .await?;

        // Index for hierarchy queries (finding direct reports)
        manager
            .create_index(
                Index::create()
                    .name("idx_org_contact_reports_to")
                    .table(OrganizationContact::Table)
                    .col(OrganizationContact::ReportsToId)
                    .to_owned(),
            )
            .await?;

        // Index for finding primary contacts
        manager
            .create_index(
                Index::create()
                    .name("idx_org_contact_primary")
                    .table(OrganizationContact::Table)
                    .col(OrganizationContact::IsPrimary)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrganizationContact::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OrganizationContact {
    Table,
    Id,
    OrganizationId,
    ContactId,
    JobTitle,
    Department,
    Role,
    ReportsToId,
    IsPrimary,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Organization {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Contact {
    Table,
    Id,
}
