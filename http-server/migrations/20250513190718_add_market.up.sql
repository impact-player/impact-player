CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE markets (
    id           UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    name         VARCHAR(255) NOT NULL,
    description  TEXT,
    base_asset   VARCHAR(50) NOT NULL,
    quote_asset  VARCHAR(50) NOT NULL,
    start_time   TIMESTAMPTZ NOT NULL,
    end_time     TIMESTAMPTZ NOT NULL,
    status       VARCHAR(50) DEFAULT 'incoming',
    created_at   TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
