-- Create organization table
CREATE TABLE organization (
    id              UUID PRIMARY KEY,
    code            TEXT UNIQUE,
    name            TEXT NOT NULL,
    display_name    TEXT,
    tax_number      TEXT,
    registration_no TEXT,
    phone           TEXT,
    email           TEXT,
    website         TEXT,
    parent_id       UUID REFERENCES organization(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_organization_code ON organization(code);
CREATE INDEX idx_organization_name ON organization(name);
CREATE INDEX idx_organization_parent ON organization(parent_id);

-- Add comments
COMMENT ON TABLE organization IS 'Organization/Company master data with hierarchy support';
COMMENT ON COLUMN organization.id IS 'Organization unique identifier (UUID v7)';
COMMENT ON COLUMN organization.code IS 'Unique organization code (e.g., ORG-001)';
COMMENT ON COLUMN organization.name IS 'Legal organization name';
COMMENT ON COLUMN organization.display_name IS 'Display/trading name';
COMMENT ON COLUMN organization.tax_number IS 'Tax identification number (MST for Vietnam)';
COMMENT ON COLUMN organization.registration_no IS 'Business registration number';
COMMENT ON COLUMN organization.phone IS 'Primary phone number';
COMMENT ON COLUMN organization.email IS 'Primary email';
COMMENT ON COLUMN organization.website IS 'Company website URL';
COMMENT ON COLUMN organization.parent_id IS 'Parent organization for hierarchy (sub-companies)';
COMMENT ON COLUMN organization.created_at IS 'Creation timestamp';
COMMENT ON COLUMN organization.updated_at IS 'Last update timestamp';
