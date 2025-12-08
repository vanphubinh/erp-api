use application::ports::PartyRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::party::Party;
use domain::party::value_objects::{DisplayName, LegalName, PartyType, RegistrationNumber, Tin};
use shared::{AppError, PaginationMeta};
use uuid::Uuid;

#[derive(Default)]
pub struct PartyRepositoryImpl;

impl PartyRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

// SQL field list for INSERT (no cast needed)
const INSERT_FIELDS: &str = "id, party_type, display_name, legal_name, tin, \
                             registration_number, is_active, created_at, updated_at";

// SQL field list for SELECT (cast party_type enum to text for Rust compatibility)
const SELECT_FIELDS: &str = "id, party_type::text as party_type, display_name, legal_name, tin, \
                             registration_number, is_active, created_at, updated_at";

// Private row struct for database deserialization
#[derive(sqlx::FromRow)]
struct PartyRow {
    id: Uuid,
    party_type: String,
    display_name: String,
    legal_name: Option<String>,
    tin: Option<String>,
    registration_number: Option<String>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl PartyRow {
    fn to_domain(self) -> Result<Party, AppError> {
        Ok(Party::from_storage(
            self.id,
            PartyType::from_str(&self.party_type)?,
            DisplayName::new(self.display_name)?,
            self.legal_name.map(LegalName::new).transpose()?,
            self.tin.map(Tin::new).transpose()?,
            self.registration_number
                .map(RegistrationNumber::new)
                .transpose()?,
            self.is_active,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl PartyRepository for PartyRepositoryImpl {
    async fn create<'a, E>(&self, executor: E, party: &Party) -> Result<(), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        sqlx::query(&format!(
            "INSERT INTO party ({INSERT_FIELDS}) \
            VALUES ($1, $2::party_type, $3, $4, $5, $6, $7, $8, $9)"
        ))
        .bind(party.id())
        .bind(party.party_type().as_str())
        .bind(party.display_name().value())
        .bind(party.legal_name().map(|n| n.value()))
        .bind(party.tin().map(|t| t.value()))
        .bind(party.registration_number().map(|r| r.value()))
        .bind(party.is_active())
        .bind(party.created_at())
        .bind(party.updated_at())
        .execute(&mut *executor.acquire().await?)
        .await?;

        Ok(())
    }

    async fn update<'a, E>(&self, executor: E, party: &Party) -> Result<(), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        sqlx::query(
            "UPDATE party SET \
             party_type = $2::party_type, display_name = $3, legal_name = $4, tin = $5, \
             registration_number = $6, is_active = $7, updated_at = $8 \
             WHERE id = $1",
        )
        .bind(party.id())
        .bind(party.party_type().as_str())
        .bind(party.display_name().value())
        .bind(party.legal_name().map(|n| n.value()))
        .bind(party.tin().map(|t| t.value()))
        .bind(party.registration_number().map(|r| r.value()))
        .bind(party.is_active())
        .bind(party.updated_at())
        .execute(&mut *executor.acquire().await?)
        .await?;

        Ok(())
    }

    async fn find_by_id<'a, E>(&self, executor: E, id: Uuid) -> Result<Option<Party>, AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        sqlx::query_as::<_, PartyRow>(&format!("SELECT {SELECT_FIELDS} FROM party WHERE id = $1"))
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
    ) -> Result<(Vec<Party>, PaginationMeta), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        let mut conn = executor.acquire().await?;

        // Get total count
        let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM party")
            .fetch_one(&mut *conn)
            .await?;

        // Get paginated results
        let offset = u64::from(page.saturating_sub(1)) * u64::from(page_size);
        let parties: Vec<Party> = sqlx::query_as::<_, PartyRow>(&format!(
            "SELECT {SELECT_FIELDS} FROM party \
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
        Ok((parties, PaginationMeta::new(page, page_size, total_u32)))
    }

    async fn delete<'a, E>(&self, executor: E, id: Uuid) -> Result<(), AppError>
    where
        E: sqlx::Acquire<'a, Database = sqlx::Postgres> + Send,
    {
        sqlx::query("DELETE FROM party WHERE id = $1")
            .bind(id)
            .execute(&mut *executor.acquire().await?)
            .await?;

        Ok(())
    }
}
