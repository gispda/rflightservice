#!/bin/bash

docker run --rm \
  --name postgres -e POSTGRES_PASSWORD=postgres \
  -p 127.0.0.1:5432:5432 -v pgdata:/var/lib/postgresql/data postgres
