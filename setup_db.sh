#!/bin/bash

# Configuration
DB_NAME="test.db"
SQL_INIT="init.sql"

# Check if sqlite3 is installed
if ! command -v sqlite3 &> /dev/null
then
    echo "Error: sqlite3 could not be found. Please install it to continue."
    exit 1
fi

# Remove existing database to start fresh
if [ -f "$DB_NAME" ]; then
    echo "Removing existing database: $DB_NAME"
    rm "$DB_NAME"
fi

# Initialize the database
echo "Initializing database $DB_NAME using $SQL_INIT..."
sqlite3 "$DB_NAME" < "$SQL_INIT"

if [ $? -eq 0 ]; then
    echo "Database successfully initialized."
else
    echo "Error: Failed to initialize database."
    exit 1
fi
