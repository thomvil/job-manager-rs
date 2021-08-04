CREATE TABLE IF NOT EXISTS jobs (
    id           SERIAL                 PRIMARY KEY,
    load         TEXT                   NOT NULL,
    difficulty   SMALLINT               NOT NULL DEFAULT '1'::smallint,
    started_at   TIME WITHOUT TIME ZONE,
    completed_at TIME WITHOUT TIME ZONE,
    persistant   BOOLEAN                NOT NULL DEFAULT false,
    started_by   TEXT,
    result       TEXT
);