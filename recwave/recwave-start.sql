create source if not exists actionhistory (
    userid int,
    itemid int,
    action int,
    timestamp timestamp,
) with (
    connector = 'kafka',
    topic = 'recwave',
    properties.bootstrap.server = 'localhost:9092',
)
FORMAT PLAIN ENCODE JSON;

create table if not exists user (
  id integer,
  address_lat numeric,
  address_long numeric,  -- datatype `point` not implemented
  age_approx integer,
  gender integer,
  occupation numeric,
  -- and more ...
);

create table if not exists item (
  id integer,
  category integer,
  brand integer,
  freshness numeric,
  popularity numeric,
  price numeric,
  rating numeric
  -- and more ...
);

create materialized view recenthistory as
 select distinct userid, itemid, timestamp from actionhistory where action=1 order by timestamp desc limit 5000;

create materialized view user_interacted_item_window as
    select userid, itemid, count(itemid) as count, window_start
    from (
        select * from tumble(actionhistory, timestamp, interval '1 minutes')
    ) recent
    group by userid, itemid, window_start;


create materialized view user_most_interacted_item as
    with counts as (select userid, itemid, count(itemid) as count, window_start
    from (
        select * from tumble(actionhistory, timestamp, interval '1 minutes')
    ) recent
    group by userid, itemid, window_start
    )
select userid, max((window_start, count, itemid)) as maxcount_item from counts group by userid;
