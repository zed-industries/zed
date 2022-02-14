#!/bin/bash

databases=$(psql --tuples-only --command "
  SELECT
    datname
  FROM
    pg_database
  WHERE
    datistemplate = false
    AND datname like 'zed-test-%'
")

for database in $databases; do
  echo $database
  dropdb $database
done