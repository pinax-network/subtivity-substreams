CREATE TABLE hourly_stats
(
    id              INTEGER NOT NULL CONSTRAINT stats_pk PRIMARY KEY,
    block_num       BIGINT,
    chain           TEXT NOT NULL,
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

-- CREATE table last_block
-- (
--     chain     text    not null
--         constraint last_block_pk primary key,
--     block_num bigint  not null,
--     trx_count integer not null,
--     act_count integer not null
-- );
