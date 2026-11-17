CREATE TABLE IF NOT EXISTS blobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    content_type VARCHAR(100) NOT NULL,
    extension VARCHAR(100) NOT NULL,
    data BYTEA NOT NULL,
    size BIGINT NOT NULL, 
    sha256_hash VARCHAR(300) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    tags VARCHAR(50)[],
    metadata JSONB,
    owner_id VARCHAR(255),
    access_level VARCHAR(100)
);

CREATE INDEX IF NOT EXISTS blobs_created_idx ON blobs (created_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS blobs_tags_idx ON blobs USING GIN (tags);
