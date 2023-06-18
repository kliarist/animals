# Animal rest api

A rust crud rest api poc using:

* axum
* diesel
* postgres

### Build
Build using `cargo build --release`.

### Run
Run using `./target/release/animals-api`.

### Development
Bring the postgres docker container up by `docker-compose up -d database`. Check Diesel documentation for installing the `diesel cli` and `libmq`.

For local development it makes sense to use `cargo-watch`.

Get it using `cargo install cargo-watch` and start the

application using `cargo watch -x run` to enjoy file watching and reloading.

### Diesel migrations
To add a new migration please use `diesel migration generate <migration_name>`.

To run migrations manually use `diesel migration run`.

To rollback migrations use `diesel migration redo`.

The migrations are run automatically on application startup.

### Dockerize
To build the docker image use `docker build -t animals-api .`.

### Goals for this project
- [x] CRUD requests
- [x] Connect to a database
- [x] Externalise configuration/properties
- [x] Dockerise
- [x] Logging
- [ ] Global and local api error handling
- [ ] Add JWT security
- [ ] Release process

### Useful links

✓ [Axum](https://github.com/tokio-rs/axum)

✓ [Diesel](https://diesel.rs/guides/getting-started)

✓ [Envconfig](https://docs.rs/envconfig/latest/envconfig/)


