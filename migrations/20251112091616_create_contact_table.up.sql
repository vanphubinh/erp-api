-- Create contact table
CREATE TABLE contact (
    id UUID PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    mobile TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- Create indexes
CREATE INDEX idx_contact_email ON contact(email);
CREATE INDEX idx_contact_name ON contact(first_name, last_name);

-- Add comments
COMMENT ON TABLE contact IS 'Contact information';
COMMENT ON COLUMN contact.id IS 'Contact unique identifier';
COMMENT ON COLUMN contact.first_name IS 'First name';
COMMENT ON COLUMN contact.last_name IS 'Last name';
COMMENT ON COLUMN contact.email IS 'Email address';
COMMENT ON COLUMN contact.phone IS 'Phone number';
COMMENT ON COLUMN contact.mobile IS 'Mobile number';
COMMENT ON COLUMN contact.is_active IS 'Is contact active';
COMMENT ON COLUMN contact.created_at IS 'Creation timestamp';
COMMENT ON COLUMN contact.updated_at IS 'Last update timestamp';

