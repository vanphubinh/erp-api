use application::ports::OrganizationRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::organization::value_objects::{Email, Phone, Url};
use domain::organization::{Organization, OrganizationName};
use serde_json::Value as JsonValue;
use shared::{AppError, PaginationMeta};
use uuid::Uuid;

#[derive(Default)]
pub struct OrganizationRepositoryImpl;

impl OrganizationRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

// SQL field list constant
const SELECT_FIELDS: &str = "id, code, name, display_name, tax_number, registration_no, \
                             phone, email, website, parent_id, metadata, created_at, updated_at";

// Private row struct for database deserialization
#[derive(sqlx::FromRow)]
struct OrganizationRow {
    id: Uuid,
    code: Option<String>,
    name: String,
    display_name: Option<String>,
    tax_number: Option<String>,
    registration_no: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    website: Option<String>,
    parent_id: Option<Uuid>,
    metadata: JsonValue,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl OrganizationRow {
    fn to_domain(self) -> Result<Organization, AppError> {
        Ok(Organization::from_storage(
            self.id,
            self.code,
            OrganizationName::new(self.name)?,
            self.display_name,
            self.tax_number,
            self.registration_no,
            self.phone.map(Phone::new).transpose()?,
            self.email.map(Email::new).transpose()?,
            self.website.map(Url::new).transpose()?,
            self.parent_id,
            self.metadata,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl OrganizationRepository for OrganizationRepositoryImpl {
    async fn create<'a, E>(&self, executor: E, organization: &Organization) -> Result<(), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        sqlx::query(&format!(
            "INSERT INTO organization ({SELECT_FIELDS}) \
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)"
        ))
        .bind(organization.id())
        .bind(organization.code())
        .bind(organization.name().value())
        .bind(organization.display_name())
        .bind(organization.tax_number())
        .bind(organization.registration_no())
        .bind(organization.phone().map(|p| p.to_string()))
        .bind(organization.email().map(|e| e.to_string()))
        .bind(organization.website().map(|w| w.to_string()))
        .bind(organization.parent_id())
        .bind(organization.metadata())
        .bind(organization.created_at())
        .bind(organization.updated_at())
        .execute(&mut *executor.acquire().await?)
        .await?;

        Ok(())
    }

    async fn update<'a, E>(&self, executor: E, organization: &Organization) -> Result<(), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        sqlx::query(
            "UPDATE organization SET \
             code = $2, name = $3, display_name = $4, tax_number = $5, registration_no = $6, \
             phone = $7, email = $8, website = $9, parent_id = $10, metadata = $11, updated_at = $12 \
             WHERE id = $1",
        )
        .bind(organization.id())
        .bind(organization.code())
        .bind(organization.name().value())
        .bind(organization.display_name())
        .bind(organization.tax_number())
        .bind(organization.registration_no())
        .bind(organization.phone().map(|p| p.to_string()))
        .bind(organization.email().map(|e| e.to_string()))
        .bind(organization.website().map(|w| w.to_string()))
        .bind(organization.parent_id())
        .bind(organization.metadata())
        .bind(organization.updated_at())
        .execute(&mut *executor.acquire().await?)
        .await?;

        Ok(())
    }

    async fn find_by_id<'a, E>(
        &self,
        executor: E,
        id: Uuid,
    ) -> Result<Option<Organization>, AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        sqlx::query_as::<_, OrganizationRow>(&format!(
            "SELECT {SELECT_FIELDS} FROM organization WHERE id = $1"
        ))
        .bind(id)
        .fetch_optional(&mut *executor.acquire().await?)
        .await?
        .map(|row| row.to_domain())
        .transpose()
    }

    async fn find_paginated<'a, E>(
        &self,
        executor: E,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<Organization>, PaginationMeta), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        let mut conn = executor.acquire().await?;

        // Get total count
        let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM organization")
            .fetch_one(&mut *conn)
            .await?;

        // Get paginated results
        let offset = u64::from(page.saturating_sub(1)) * u64::from(page_size);
        let organizations: Vec<Organization> = sqlx::query_as::<_, OrganizationRow>(&format!(
            "SELECT {SELECT_FIELDS} FROM organization \
             ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        ))
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .map(|row| row.to_domain())
        .collect::<Result<Vec<_>, _>>()?;

        let total_u32 = total.try_into().unwrap_or(u32::MAX);
        Ok((
            organizations,
            PaginationMeta::new(page, page_size, total_u32),
        ))
    }

    async fn delete<'a, E>(&self, executor: E, id: Uuid) -> Result<(), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        sqlx::query("DELETE FROM organization WHERE id = $1")
            .bind(id)
            .execute(&mut *executor.acquire().await?)
            .await?;

        Ok(())
    }
}
