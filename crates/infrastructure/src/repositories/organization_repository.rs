use application::ports::OrganizationRepository;
use async_trait::async_trait;
use domain::organization::value_objects::{Email, Phone, Url};
use domain::organization::{Organization, OrganizationName};
use sea_orm::*;
use shared::{AppError, PaginationMeta};
use std::convert::TryFrom;
use uuid::Uuid;

use crate::persistence::entity::organization;

pub struct OrganizationRepositoryImpl;

impl OrganizationRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

// Mapping: SeaORM Model → Domain
impl TryFrom<organization::Model> for Organization {
    type Error = AppError;

    fn try_from(entity: organization::Model) -> Result<Self, Self::Error> {
        Ok(Organization::from_storage(
            entity.id,
            OrganizationName::new(entity.name)?,
            entity.email.map(Email::new).transpose()?,
            entity.phone.map(Phone::new).transpose()?,
            entity.website.map(Url::new).transpose()?,
            entity.industry,
            entity.address,
            entity.city,
            entity.state,
            entity.postal_code,
            entity.country_code,
            entity.timezone,
            entity.currency,
            entity.is_active,
            entity.created_at.naive_utc().and_utc(), // DateTimeWithTimeZone → DateTime<Utc>
            entity.updated_at.naive_utc().and_utc(),
        ))
    }
}

// Mapping: Domain → SeaORM ActiveModel
impl From<Organization> for organization::ActiveModel {
    fn from(org: Organization) -> Self {
        organization::ActiveModel {
            id: Set(org.id()),
            name: Set(org.name().value().to_string()),
            email: Set(org.email().map(|e| e.to_string())),
            phone: Set(org.phone().map(|p| p.to_string())),
            website: Set(org.website().map(|w| w.to_string())),
            industry: Set(org.industry().map(|s| s.to_string())),
            address: Set(org.address().map(|s| s.to_string())),
            city: Set(org.city().map(|s| s.to_string())),
            state: Set(org.state().map(|s| s.to_string())),
            postal_code: Set(org.postal_code().map(|s| s.to_string())),
            country_code: Set(org.country_code().map(|s| s.to_string())),
            timezone: Set(org.timezone().map(|s| s.to_string())),
            currency: Set(org.currency().map(|s| s.to_string())),
            is_active: Set(org.is_active()),
            created_at: Set(org.created_at().into()), // DateTime<Utc> → DateTimeWithTimeZone
            updated_at: Set(org.updated_at().into()),
        }
    }
}

#[async_trait]
impl OrganizationRepository for OrganizationRepositoryImpl {
    async fn save<C>(&self, conn: &C, organization: &Organization) -> Result<(), AppError>
    where
        C: ConnectionTrait,
    {
        let active_model: organization::ActiveModel = organization.clone().into();
        active_model.insert(conn).await?;
        Ok(())
    }

    async fn update<C>(&self, conn: &C, organization: &Organization) -> Result<(), AppError>
    where
        C: ConnectionTrait,
    {
        let active_model: organization::ActiveModel = organization.clone().into();
        active_model.update(conn).await?;
        Ok(())
    }

    async fn find_by_id<C>(&self, conn: &C, id: Uuid) -> Result<Option<Organization>, AppError>
    where
        C: ConnectionTrait,
    {
        organization::Entity::find_by_id(id)
            .one(conn)
            .await?
            .map(Organization::try_from)
            .transpose()
    }

    async fn find_paginated<C>(
        &self,
        conn: &C,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<Organization>, PaginationMeta), AppError>
    where
        C: ConnectionTrait,
    {
        // Get total count
        let total = organization::Entity::find().count(conn).await?;

        // Calculate offset
        let offset = (page.saturating_sub(1)) * page_size;

        // Get paginated results
        let organizations = organization::Entity::find()
            .offset(offset)
            .limit(page_size)
            .order_by_desc(organization::Column::CreatedAt)
            .all(conn)
            .await?
            .into_iter()
            .map(Organization::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        // Create pagination metadata
        let pagination_meta = PaginationMeta::new(page, page_size, total);

        Ok((organizations, pagination_meta))
    }

    async fn delete<C>(&self, conn: &C, id: Uuid) -> Result<(), AppError>
    where
        C: ConnectionTrait,
    {
        organization::Entity::delete_by_id(id).exec(conn).await?;
        Ok(())
    }
}
