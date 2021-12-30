#!/bin/bash

PGPASSWORD="----" psql \
-h ---- \
-p ---- -d ---- \
-U --- \
-c "select t.category_id, c.name as category, t.status, count(t.id), c.max_tickets from ticket t join ticket_category c on t.category_id = c.id
    where t.event_id = (select id from event where short_name = '2022')
    and t.status <> 'INVALIDATED'
    and (c.name like 'Blind%' or c.name like 'Early%')
    group by t.status, t.category_id, c.name, c.max_tickets
    order by t.category_id"

