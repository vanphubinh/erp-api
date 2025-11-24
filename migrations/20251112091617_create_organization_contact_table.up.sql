-- Create organization_contact table (junction/association table)
CREATE TABLE organization_contact (
    id UUID PRIMARY KEY,
    organization_id UUID NOT NULL,
    contact_id UUID NOT NULL,
    job_title TEXT,
    department TEXT,
    role TEXT,
    reports_to_id UUID,
    is_primary BOOLEAN NOT NULL DEFAULT false,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    
    -- Foreign key constraints
    CONSTRAINT fk_org_contact_organization
        FOREIGN KEY (organization_id)
        REFERENCES organization(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    
    CONSTRAINT fk_org_contact_contact
        FOREIGN KEY (contact_id)
        REFERENCES contact(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    
    CONSTRAINT fk_org_contact_reports_to
        FOREIGN KEY (reports_to_id)
        REFERENCES organization_contact(id)
        ON UPDATE CASCADE
        ON DELETE SET NULL,
    
    -- Unique constraint for organization-contact combination
    CONSTRAINT idx_org_contact_unique
        UNIQUE (organization_id, contact_id)
);

-- Create indexes
CREATE INDEX idx_org_contact_org ON organization_contact(organization_id);
CREATE INDEX idx_org_contact_contact ON organization_contact(contact_id);
CREATE INDEX idx_org_contact_reports_to ON organization_contact(reports_to_id);

-- Add comments
COMMENT ON TABLE organization_contact IS 'Association between organizations and contacts';
COMMENT ON COLUMN organization_contact.id IS 'Unique identifier';
COMMENT ON COLUMN organization_contact.organization_id IS 'Reference to organization';
COMMENT ON COLUMN organization_contact.contact_id IS 'Reference to contact';
COMMENT ON COLUMN organization_contact.job_title IS 'Job title at this organization';
COMMENT ON COLUMN organization_contact.department IS 'Department';
COMMENT ON COLUMN organization_contact.role IS 'Role description';
COMMENT ON COLUMN organization_contact.reports_to_id IS 'Reference to another org-contact (reporting structure)';
COMMENT ON COLUMN organization_contact.is_primary IS 'Is this the primary contact for the organization';
COMMENT ON COLUMN organization_contact.is_active IS 'Is this association active';
COMMENT ON COLUMN organization_contact.created_at IS 'Creation timestamp';
COMMENT ON COLUMN organization_contact.updated_at IS 'Last update timestamp';

