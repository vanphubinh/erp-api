-- Create organization table
CREATE TABLE organization (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    website TEXT,
    industry TEXT,
    address TEXT,
    city TEXT,
    state TEXT,
    postal_code TEXT,
    country_code TEXT,
    timezone TEXT,
    currency TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- Create indexes
CREATE INDEX idx_organization_name ON organization(name);
CREATE INDEX idx_organization_country ON organization(country_code);

-- Add comments
COMMENT ON TABLE organization IS 'Organization/Company master data';
COMMENT ON COLUMN organization.id IS 'Organization unique identifier';
COMMENT ON COLUMN organization.name IS 'Organization name';
COMMENT ON COLUMN organization.email IS 'Primary email';
COMMENT ON COLUMN organization.phone IS 'Primary phone number';
COMMENT ON COLUMN organization.website IS 'Company website URL';
COMMENT ON COLUMN organization.industry IS 'Industry type';
COMMENT ON COLUMN organization.address IS 'Street address';
COMMENT ON COLUMN organization.city IS 'City';
COMMENT ON COLUMN organization.state IS 'State/Province';
COMMENT ON COLUMN organization.postal_code IS 'Postal/ZIP code';
COMMENT ON COLUMN organization.country_code IS 'ISO 3166-1 alpha-2 country code (e.g., US, GB, TH)';
COMMENT ON COLUMN organization.timezone IS 'IANA timezone (e.g., America/New_York, Asia/Bangkok)';
COMMENT ON COLUMN organization.currency IS 'ISO 4217 currency code (e.g., USD, EUR, THB)';
COMMENT ON COLUMN organization.is_active IS 'Is organization active';
COMMENT ON COLUMN organization.created_at IS 'Creation timestamp';
COMMENT ON COLUMN organization.updated_at IS 'Last update timestamp';

