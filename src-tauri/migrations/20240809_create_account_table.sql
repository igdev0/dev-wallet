CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY,
    wallet_id UUID REFERENCES wallets(id),
    address NOT NULL UNIQUE,
    "index" INTEGER NOT NULL,
    path TEXT NOT NULL UNIQUE,
    network TEXT NOT NULL,
    blockchain TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);