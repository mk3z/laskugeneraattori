# Laskugeneraattori

This is the laskugeneraattoori backend written in Rust.

The application is based on [axum](https://github.com/tokio-rs/axum).
PDF generation is based on [typst](https://github.com/typst/typst).

## Configuration

The following variables can be configured in the environment (or the .env file)

```sh
# VARIABLE="default value"

PORT=3000
BIND_ADDR=127.0.0.1
ALLOWED_ORIGINS= # comma separated list of urls
MAILGUN_URL=
MAILGUN_USER=
MAILGUN_PASSWORD=
MAILGUN_TO=
MAILGUN_FROM=
```

## Running laskugeneraattori

### With cargo
```sh
cargo run
```

### With Docker
```sh
docker build . -t laskugeneraattori
docker run laskugeneraattori
``
