-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Create people table
CREATE TABLE IF NOT EXISTS people (
    id UUID DEFAULT uuid_generate_v4(),
    nickname VARCHAR(32) CONSTRAINT people_pkey PRIMARY KEY,
    name VARCHAR(100),
    birth_date CHAR(10),
    stack VARCHAR(1024),
    search_trgm TEXT GENERATED ALWAYS AS (
        LOWER(name || nickname || stack)
    ) STORED
);

-- Create trigram index for fuzzy search
CREATE INDEX IF NOT EXISTS idx_people_search_trgm
    ON people USING GIST (search_trgm gist_trgm_ops (siglen = 64));
