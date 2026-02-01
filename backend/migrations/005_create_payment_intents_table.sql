-- Create payment intent status enum type
CREATE TYPE payment_intent_status AS ENUM ('created', 'capturable', 'captured', 'cancelled');

-- Track Stripe payment intents for idempotency
CREATE TABLE payment_intents (
    id VARCHAR(255) PRIMARY KEY, -- Stripe payment_intent ID
    ticket_id UUID NOT NULL REFERENCES tickets(id) ON DELETE RESTRICT,
    buyer_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    amount INTEGER NOT NULL,
    currency VARCHAR(3) NOT NULL, -- ISO currency code (e.g., "usd")
    status payment_intent_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payment_intents_ticket_id ON payment_intents(ticket_id);
CREATE INDEX idx_payment_intents_status ON payment_intents(status);

-- Create trigger to automatically update updated_at
CREATE TRIGGER update_payment_intents_updated_at BEFORE UPDATE ON payment_intents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();



