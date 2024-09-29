#!/bin/sh

# app
export DB_HOST=localhost
export DB_PORT=5432
export DB_USER="dungeon"
export DB_PASSWORD_PATH=$PWD/db/db_password.txt
export DB_INIT_PATH=$PWD/db/init.sql
# rust
export RUST_BACKTRACE=1
export RUST_LOG=trace

cd server && cargo run
