# Laskugeneraattori

This is the laskugeneraattoori backend written in Rust.

The application is based on [axum](https://github.com/tokio-rs/axum):

## Development

You need a local Postgres setup or (maybe preferably) use [Docker Compose](https://docs.docker.com/compose/gettingstarted/) or any other container runtime

```sh
docker compose up -d
```

You need diesel CLI for running migrations:

```sh
cargo install diesel_cli
```

Nith the DB up, run migrations:

```sh
diesel migration run
```

Now the tests should pass:

```sh
cargo test
```

## Features/TODO:

- [x] create invoice + validation
- [ ] create user + authentication
- [x] list invoices
- [ ] edit invoice
- [ ] ratelimits
- [ ] generate pdf
- [ ] write documentation
