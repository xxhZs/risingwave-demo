CREATE SOURCE twitter (
    data STRUCT < created_at TIMESTAMP,
    id VARCHAR,
    text VARCHAR,
    lang VARCHAR >,
    author STRUCT < created_at TIMESTAMP,
    id VARCHAR,
    name VARCHAR,
    username VARCHAR,
    followers INT >,
) WITH (
    'connector' = 'kafka',
    'kafka.topic' = 'twitter',
    'kafka.brokers' = 'message_queue:29092',
    'kafka.scan.startup.mode' = 'earliest'
) ROW FORMAT JSON;

--
-- The CREATE TABLE version
--
CREATE TABLE twitter (
    data STRUCT < created_at TIMESTAMP,
    id VARCHAR,
    text VARCHAR,
    lang VARCHAR >,
    author STRUCT < created_at TIMESTAMP,
    id VARCHAR,
    name VARCHAR,
    username VARCHAR,
    followers INT >,
);

CREATE MATERIALIZED VIEW influencer_tweets AS
SELECT
    (data).text
FROM
    twitter
WHERE
    (author).followers > 5000
    AND (data).lang = 'English';

CREATE MATERIALIZED VIEW hot_hashtags AS WITH tags AS (
    SELECT
        regexp_match((data).text, '#\w+', 'g') AS hashtag,
        (data).created_at as created_at
    FROM
        twitter
)
SELECT
    hashtag,
    COUNT(*) as hashtag_occurrences
FROM
    tags
WHERE
    created_at :: Date = CURRENT_DATE
GROUP BY
    hashtag;

--
-- Postgres
--
CREATE TYPE twitter_user AS (
    created_at TIMESTAMP,
    id VARCHAR,
    name VARCHAR,
    username VARCHAR,
    followers INT
);

CREATE TYPE tweet_data AS (
    created_at TIMESTAMP,
    id VARCHAR,
    text VARCHAR,
    lang VARCHAR
);

CREATE TABLE twitter (data tweet_data, author twitter_user);