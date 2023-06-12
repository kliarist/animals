# Animal rest api

A rust crud rest api poc using:

* axum

### Build
Build using `cargo build --release`.

### Run
Run using `./target/release/animals-api`.

### Development
For local development it makes sense to use `cargo-watch`. 
Get it using `cargo install cargo-watch` and start the 
application using `cargo watch -x run` to enjoy file watching and reloading.

### Diesel migrations
To add a new migration please use `diesel migration generate <migration_name>`.

To run migrations use `diesel migration run`.

To rollback migrations use `diesel migration redo`.

### Goals for this project
- [x] CRUD requests
- [ ] Externalise configuration/properties
- [ ] Connect to a database
- [ ] Logging
- [ ] Add JWT security
- [ ] Global and local api error handling
- [ ] Dockerise

### Useful links
✓ [Diesel](https://diesel.rs/guides/getting-started)

