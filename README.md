# SQLx serialization panic reproducer

How to reproduce the panic with this repository:

1. Make sure docker compose is installed
2. Run `docker compose up -d` to start the database (or start it some other way if you don't want to use docker compose)
3. `cargo install sqlx-cli` to install sqlx-cli
4. `sqlx migrate run` to add the necessary database table
5. `cargo run` This will reproduce the panic