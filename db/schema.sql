CREATE TABLE IF NOT EXISTS hourly_stats
(
    id              INTEGER NOT NULL CONSTRAINT stats_pk PRIMARY KEY,
    block_num       BIGINT,
    chain           TEXT NOT NULL,
    traces_count    INTEGER NOT NULL,
    action_count    INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS cursors
(
    id          TEXT NOT NULL CONSTRAINT cursor_pk PRIMARY KEY,
    cursor      TEXT,
    block_num   BIGINT,
    block_id    TEXT
);

-- create table if not exists last_block
-- (
--     chain     text    not null
--         constraint last_block_pk primary key,
--     block_num bigint  not null,
--     trx_count integer not null,
--     act_count integer not null
-- );

-- create table if not exists max_trx_block
-- (
--     block_num bigint  not null
--         constraint max_trx_block_pk primary key,
--     chain     text    not null,
--     trx_count integer not null
-- );

-- create table if not exists max_action_block
-- (
--     block_num bigint  not null
--         constraint max_action_block_pk primary key,
--     chain     text    not null,
--     act_count integer not null
-- );

-- create table if not exists cursors
-- (
--     id        text not null
--         constraint cursor_pk primary key,
--     cursor    text,
--     block_num bigint,
--     block_id  text
-- );