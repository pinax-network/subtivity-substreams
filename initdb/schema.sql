CREATE TABLE stats
(
    id              TEXT NOT NULL CONSTRAINT stats_pk PRIMARY KEY,
    chain           TEXT NOT NULL,
    block_num       INTEGER NOT NULL,
    seconds         INTEGER NOT NULL,
    interval        INTEGER NOT NULL,
    traces_count    INTEGER NOT NULL,
    action_count    INTEGER NOT NULL
);

CREATE TABLE cursors
(
    id          TEXT NOT NULL CONSTRAINT cursor_pk PRIMARY KEY,
    cursor      TEXT,
    block_num   BIGINT,
    block_id    TEXT
);
