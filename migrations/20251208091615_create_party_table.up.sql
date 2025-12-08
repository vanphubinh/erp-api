-- Create party_type enum
CREATE TYPE party_type AS ENUM ('company', 'person');

-- Create party table (unified entity for companies and persons)
CREATE TABLE party (
    id                  UUID PRIMARY KEY,
    party_type          party_type NOT NULL,
    display_name        TEXT NOT NULL,
    legal_name          TEXT,
    tin                 TEXT,
    registration_number TEXT,
    is_active           BOOLEAN NOT NULL DEFAULT TRUE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_party_display_name ON party(display_name);
CREATE INDEX idx_party_legal_name ON party(legal_name);
CREATE INDEX idx_party_tin ON party(tin);
CREATE INDEX idx_party_party_type ON party(party_type);
CREATE INDEX idx_party_is_active ON party(is_active);

-- Add comments
COMMENT ON TABLE party IS 'Unified party table for companies and persons (Party pattern)';
COMMENT ON COLUMN party.id IS 'Party unique identifier (UUID v7)';
COMMENT ON COLUMN party.party_type IS 'Type of party: company or person';
COMMENT ON COLUMN party.display_name IS 'Display/trading name (required)';
COMMENT ON COLUMN party.legal_name IS 'Legal/registered name';
COMMENT ON COLUMN party.tin IS 'Tax identification number (MST for Vietnam)';
COMMENT ON COLUMN party.registration_number IS 'Business registration number';
COMMENT ON COLUMN party.is_active IS 'Whether the party is active';
COMMENT ON COLUMN party.created_at IS 'Creation timestamp';
COMMENT ON COLUMN party.updated_at IS 'Last update timestamp';
