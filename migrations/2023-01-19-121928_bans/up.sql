CREATE TABLE IF NOT EXISTS `bans` (
    id SERIAL PRIMARY KEY,
    identifiers JSON NOT NULL,
    reason VARCHAR(256) NOT NULL,
    server VARCHAR(32) NOT NULL,
    expires BIGINT UNSIGNED NOT NULL
);