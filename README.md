# zero2prod

A simple Rust web service based on the "Zero to Production" tutorial.

## Overview

- Built with `actix-web` and `sqlx`.
- Provides health check and newsletter subscription endpoints.
- Database migrations stored in `migrations/`.

## Getting Started

1. Install Rust and Cargo: https://www.rust-lang.org/tools/install
2. Start PostgreSQL and create a database.
3. Update `DATABASE_URL` in environment variables (or `.env`) to point at your Postgres DB.

## Run migrations

```bash
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
```

## Run tests

```bash
cargo test
```

## Run server

```bash
cargo run
```

Then open `http://127.0.0.1:8000`.

## Endpoints

- `GET /health_check` - simple status check (200 OK)
- `POST /subscriptions` - accepts form data: `name`, `email`

## Project structure

- `src/main.rs` — server entrypoint
- `src/routes/` — route definitions
- `migrations/` — database schema migrations
- `tests/` — integration tests

## Notes

This repository is intended for learning and demonstration purposes.