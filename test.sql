SELECT
    page,
    count(*) AS "count!",
    cast(extract(hour from created_at) AS integer) AS "hour_of_day!",
    count((
        SELECT 1
        FROM page_hits
        WHERE page_hits.status = 200
        GROUP BY extract(hour from created_at)) AS "per_hour!"
FROM page_hits
WHERE page_hits.status = 200
GROUP BY "hour_of_day!", page;
