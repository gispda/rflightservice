#!/bin/bash

docker exec -it postgres sh -c "PGPASSWORD=postgres /usr/bin/psql -h 127.0.0.1 -U postgres -d postgres"
#docker exec -it postgres sh -c "PGPASSWORD=postgres /usr/bin/pgadmin3 -h 127.0.0.1 -U postgres -d postgres"