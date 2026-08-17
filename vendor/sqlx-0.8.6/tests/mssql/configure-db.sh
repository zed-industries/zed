#!/usr/bin/env bash

# Wait 60 seconds for SQL Server to start up
sleep 60

# Run the setup script to create the DB and the schema in the DB
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P $SA_PASSWORD -d master -i setup.sql
