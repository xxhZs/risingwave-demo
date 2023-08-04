create source if not exists actionhistory (
    userid varchar,
    eventype int, -- '1' is mfa , other is other
    timestamp timestamp,
    changenum int,
) with (
    connector = 'kafka',
    topic = 'recwave',
    properties.bootstrap.server = 'localhost:9092',
)
FORMAT PLAIN ENCODE JSON;

create materialized view user_action_mfa as select userid, timestamp,changenum from actionhistory where eventype = 1;

create materialized view user_mfa_change_count as 
      select userid , count(*) as count, window_start
      from(
        select * from tumble(user_action_mfa , timestamp , INTERVAL '30 seconds')
      ) group by userid,window_start;

create materialized view user_mfa_change_num as 
      select userid , udf_sum(eventype,changenum) as udf_sum, window_start
      from(
        select * from tumble(user_action_mfa , timestamp , INTERVAL '30 seconds')
      ) group by userid,window_start;