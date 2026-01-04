-- Create ticket status enum type
CREATE TYPE ticket_status AS ENUM ('unverified', 'verified', 'reserved', 'paid', 'sold', 'cancelled');

-- Create tickets table
CREATE TABLE tickets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    seller_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    game_id UUID NOT NULL REFERENCES games(id) ON DELETE RESTRICT,
    event_name VARCHAR(255) NOT NULL,
    event_date TIMESTAMPTZ NOT NULL,
    level VARCHAR(50) NOT NULL,
    seat_section VARCHAR(100) NOT NULL,
    seat_row VARCHAR(50) NOT NULL,
    seat_number VARCHAR(50) NOT NULL,
    price INTEGER NOT NULL CHECK (price >= 0),
    status ticket_status NOT NULL DEFAULT 'unverified',
    reserved_at TIMESTAMPTZ,
    reserved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for common queries
CREATE INDEX idx_tickets_seller_id ON tickets(seller_id);
CREATE INDEX idx_tickets_game_id ON tickets(game_id);
CREATE INDEX idx_tickets_status ON tickets(status);
CREATE INDEX idx_tickets_event_date ON tickets(event_date);
CREATE INDEX idx_tickets_status_event_date ON tickets(status, event_date);
CREATE INDEX idx_tickets_verified_status ON tickets(game_id) WHERE status = 'verified';

-- Create trigger to automatically update updated_at
CREATE TRIGGER update_tickets_updated_at BEFORE UPDATE ON tickets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();


