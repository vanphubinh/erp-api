use application::ports::OrganizationRepository;
use domain::organization::{Email, Organization, OrganizationName, Phone, Url};
use fake::{
    Fake,
    faker::{
        company::en::CompanyName,
        internet::en::{SafeEmail, Username},
        phone_number::en::PhoneNumber,
    },
};
use infrastructure::repositories::OrganizationRepositoryImpl;
use sqlx::PgPool;

// ============================================================================
// Helpers
// ============================================================================

/// Generate unique name to avoid test conflicts in shared DB
pub fn unique_name(prefix: &str) -> String {
    format!("{}_{}", prefix, uuid::Uuid::now_v7())
}

// ============================================================================
// Organization Factories
// ============================================================================

/// Create organization with specific name
pub fn org(name: &str) -> Organization {
    Organization::new(OrganizationName::new(name).unwrap())
}

/// Create organization with unique fake company name
pub fn fake_org() -> Organization {
    org(&unique_name(&CompanyName().fake::<String>()))
}

/// Create organization with all fields populated
pub fn fake_org_full() -> Organization {
    let base = fake_org();

    Organization::from_storage(
        base.id(),
        Some(format!("ORG-{}", uuid::Uuid::now_v7().to_string()[..8].to_uppercase())),
        OrganizationName::new(base.name().value()).unwrap(),
        Some(format!("{} Display", base.name().value())),
        Some("0123456789".to_string()),
        Some("BRN-12345".to_string()),
        Some(Phone::new(PhoneNumber().fake::<String>()).unwrap()),
        Some(Email::new(SafeEmail().fake::<String>()).unwrap()),
        Some(
            Url::new(format!(
                "https://{}.com",
                Username().fake::<String>().to_lowercase()
            ))
            .unwrap(),
        ),
        None, // parent_id
        base.created_at(),
        base.updated_at(),
    )
}

// ============================================================================
// Seeding Helpers
// ============================================================================

/// Seed n fake organizations, returns them for assertions
pub async fn seed_n(
    pool: &PgPool,
    repo: &OrganizationRepositoryImpl,
    n: usize,
) -> Vec<Organization> {
    let mut orgs = Vec::with_capacity(n);
    for _ in 0..n {
        let o = fake_org();
        repo.create(pool, &o).await.expect("Failed to seed");
        orgs.push(o);
    }
    orgs
}

/// Seed a single organization and return it
pub async fn seed_one(pool: &PgPool, repo: &OrganizationRepositoryImpl) -> Organization {
    let o = fake_org();
    repo.create(pool, &o).await.expect("Failed to seed");
    o
}

/// Seed predefined organizations with unique names
pub async fn seed_known(
    pool: &PgPool,
    repo: &OrganizationRepositoryImpl,
) -> (Organization, Organization, Organization) {
    let acme = org(&unique_name("Acme"));
    let wayne = org(&unique_name("Wayne"));
    let stark = org(&unique_name("Stark"));

    repo.create(pool, &acme).await.unwrap();
    repo.create(pool, &wayne).await.unwrap();
    repo.create(pool, &stark).await.unwrap();

    (acme, wayne, stark)
}
