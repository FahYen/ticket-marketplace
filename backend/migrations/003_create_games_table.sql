-- Create sport_type enum
CREATE TYPE sport_type AS ENUM (
    'football',
    'basketball',
    'hockey'
);

-- Create games table
CREATE TABLE games (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sport_type sport_type NOT NULL,
    name VARCHAR(255) NOT NULL,
    game_time TIMESTAMPTZ NOT NULL,
    cutoff_time TIMESTAMPTZ NOT NULL
);

-- Create indexes for common queries
CREATE INDEX idx_games_sport_type ON games(sport_type);
CREATE INDEX idx_games_game_time ON games(game_time);
CREATE INDEX idx_games_cutoff_time ON games(cutoff_time);
CREATE INDEX idx_games_sport_type_game_time ON games(sport_type, game_time);

