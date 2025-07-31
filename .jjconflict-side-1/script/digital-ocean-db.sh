#!/bin/bash
set -e

# Check if database name is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <database-name>"
    doctl databases list
    exit 1
fi

DATABASE_NAME="$1"
DATABASE_ID=$(doctl databases list --format ID,Name --no-header | grep "$DATABASE_NAME" | awk '{print $1}')

if [ -z "$DATABASE_ID" ]; then
    echo "Error: Database '$DATABASE_NAME' not found"
    exit 1
fi
CURRENT_IP=$(curl -s https://api.ipify.org)
if [ -z "$CURRENT_IP" ]; then
    echo "Error: Failed to get current IP address"
    exit 1
fi

EXISTING_RULE=$(doctl databases firewalls list "$DATABASE_ID" | grep "ip_addr" | grep "$CURRENT_IP")

if [ -z "$EXISTING_RULE" ]; then
    echo "IP not found in whitelist. Adding $CURRENT_IP to database firewall..."
    doctl databases firewalls append "$DATABASE_ID" --rule ip_addr:"$CURRENT_IP"
fi

CONNECTION_URL=$(doctl databases connection "$DATABASE_ID" --format URI --no-header)

if [ -z "$CONNECTION_URL" ]; then
    echo "Error: Failed to get database connection details"
    exit 1
fi

psql "$CONNECTION_URL"
