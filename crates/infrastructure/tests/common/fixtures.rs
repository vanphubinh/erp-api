use application::ports::PartyRepository;
use domain::party::{DisplayName, LegalName, Party, PartyType, RegistrationNumber, Tin};
use fake::{
    Fake,
    faker::company::en::CompanyName,
};
use infrastructure::repositories::PartyRepositoryImpl;
use sqlx::PgPool;

// ============================================================================
// Helpers
// ============================================================================

/// Generate unique name to avoid test conflicts in shared DB
pub fn unique_name(prefix: &str) -> String {
    format!("{}_{}", prefix, uuid::Uuid::now_v7())
}

// ============================================================================
// Party Factories
// ============================================================================

/// Create party with specific name
pub fn party(name: &str) -> Party {
    Party::new(PartyType::Company, DisplayName::new(name).unwrap())
}

/// Create party with unique fake company name
pub fn fake_party() -> Party {
    party(&unique_name(&CompanyName().fake::<String>()))
}

/// Create party with all fields populated
pub fn fake_party_full() -> Party {
    let base = fake_party();

    Party::from_storage(
        base.id(),
        PartyType::Company,
        DisplayName::new(base.display_name().value()).unwrap(),
        Some(LegalName::new(format!("{} Ltd.", base.display_name().value())).unwrap()),
        Some(Tin::new("0123456789").unwrap()),
        Some(RegistrationNumber::new("BRN-12345").unwrap()),
        true,
        base.created_at(),
        base.updated_at(),
    )
}

// ============================================================================
// Seeding Helpers
// ============================================================================

/// Seed n fake parties, returns them for assertions
pub async fn seed_n(
    pool: &PgPool,
    repo: &PartyRepositoryImpl,
    n: usize,
) -> Vec<Party> {
    let mut parties = Vec::with_capacity(n);
    for _ in 0..n {
        let p = fake_party();
        repo.create(pool, &p).await.expect("Failed to seed");
        parties.push(p);
    }
    parties
}

/// Seed a single party and return it
pub async fn seed_one(pool: &PgPool, repo: &PartyRepositoryImpl) -> Party {
    let p = fake_party();
    repo.create(pool, &p).await.expect("Failed to seed");
    p
}

/// Seed predefined parties with unique names
pub async fn seed_known(
    pool: &PgPool,
    repo: &PartyRepositoryImpl,
) -> (Party, Party, Party) {
    let acme = party(&unique_name("Acme"));
    let wayne = party(&unique_name("Wayne"));
    let stark = party(&unique_name("Stark"));

    repo.create(pool, &acme).await.unwrap();
    repo.create(pool, &wayne).await.unwrap();
    repo.create(pool, &stark).await.unwrap();

    (acme, wayne, stark)
}
