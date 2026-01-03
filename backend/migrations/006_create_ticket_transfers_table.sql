-- Create transfer type enum
CREATE TYPE transfer_type AS ENUM ('to_custodial', 'to_buyer');

-- Create transfer status enum
CREATE TYPE transfer_status AS ENUM ('pending', 'in_progress', 'completed', 'failed');

-- Create ticket_transfers table
CREATE TABLE ticket_transfers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE RESTRICT,
    transfer_type transfer_type NOT NULL,
    paciolan_ticket_id VARCHAR(255),
    status transfer_status NOT NULL DEFAULT 'pending',
    error_message TEXT,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for common queries
CREATE INDEX idx_ticket_transfers_transaction_id ON ticket_transfers(transaction_id);
CREATE INDEX idx_ticket_transfers_status ON ticket_transfers(status);
CREATE INDEX idx_ticket_transfers_paciolan_ticket_id ON ticket_transfers(paciolan_ticket_id) WHERE paciolan_ticket_id IS NOT NULL;


