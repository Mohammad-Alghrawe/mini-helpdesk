-- add owner_id to tickets
ALTER TABLE tickets ADD COLUMN owner_id INTEGER NOT NULL DEFAULT 0;

-- optional index for performance
CREATE INDEX IF NOT EXISTS idx_tickets_owner_id ON tickets(owner_id);
