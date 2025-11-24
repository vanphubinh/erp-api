use application::ports::OrganizationRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::organization::value_objects::{Email, Phone, Url};
use domain::organization::{Organization, OrganizationName};
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
const SELECT_FIELDS: &str = "id, name, email, phone, website, industry, \
                             address, city, state, postal_code, country_code, \
                             timezone, currency, is_active, created_at, updated_at";

// Private row struct for database deserialization
#[derive(sqlx::FromRow)]
struct OrganizationRow {
    id: Uuid,
    name: String,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>,
    industry: Option<String>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    country_code: Option<String>,
    timezone: Option<String>,
    currency: Option<String>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl OrganizationRow {
    fn to_domain(self) -> Result<Organization, AppError> {
        Ok(Organization::from_storage(
            self.id,
            OrganizationName::new(self.name)?,
            self.email.map(Email::new).transpose()?,
            self.phone.map(Phone::new).transpose()?,
            self.website.map(Url::new).transpose()?,
            self.industry,
            self.address,
            self.city,
            self.state,
            self.postal_code,
            self.country_code,
            self.timezone,
            self.currency,
            self.is_active,
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
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)"
        ))
        .bind(organization.id())
        .bind(organization.name().value())
        .bind(organization.email().map(|e| e.to_string()))
        .bind(organization.phone().map(|p| p.to_string()))
        .bind(organization.website().map(|w| w.to_string()))
        .bind(organization.industry())
        .bind(organization.address())
        .bind(organization.city())
        .bind(organization.state())
        .bind(organization.postal_code())
        .bind(organization.country_code())
        .bind(organization.timezone())
        .bind(organization.currency())
        .bind(organization.is_active())
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
             name = $2, email = $3, phone = $4, website = $5, industry = $6, \
             address = $7, city = $8, state = $9, postal_code = $10, country_code = $11, \
             timezone = $12, currency = $13, is_active = $14, updated_at = $15 \
             WHERE id = $1",
        )
        .bind(organization.id())
        .bind(organization.name().value())
        .bind(organization.email().map(|e| e.to_string()))
        .bind(organization.phone().map(|p| p.to_string()))
        .bind(organization.website().map(|w| w.to_string()))
        .bind(organization.industry())
        .bind(organization.address())
        .bind(organization.city())
        .bind(organization.state())
        .bind(organization.postal_code())
        .bind(organization.country_code())
        .bind(organization.timezone())
        .bind(organization.currency())
        .bind(organization.is_active())
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
        page: u64,
        page_size: u64,
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
        let offset = page.saturating_sub(1) * page_size;
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

        Ok((
            organizations,
            PaginationMeta::new(page, page_size, total as u64),
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
