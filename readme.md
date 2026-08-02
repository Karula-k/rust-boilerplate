## Boilerplate


### prerequisite
setup cli command
```sh
cargo install refinery_cli
cargo install cornucopia
```
install make using chocho or your favorite hub
```sh
choco install make
```

### Planning
- [x] set up AXUM
- [x] set up database
    - [x] tokio postgresql
    - [x] refinery as migration
    - [x] Cornucopia as sql builder
    - [ ] Routing 
    - [ ] Integrate Cornucopia as Handler route
- [ ] Middleware

#### Tech Stack
 - [AXUM](https://github.com/tokio-rs/axum)
 - [Cornucopia](https://github.com/cornucopia-rs/cornucopia)
 - [Refinery ](https://github.com/rust-db/refinery)
 - TBD since idk what to add this project exist because i cant sleep


### Structure
```sh
    .
    ├── database
    │   ├── migrations     # Refinery migrations
    │   └── sql            # Cornucopia queries
    │
    ├── src
    │   ├── routes         # HTTP handlers
    │   ├── config.rs      # Environment configuration
    │   ├── db             # Generated Cornucopia code
    │   └── main.rs
    │
    ├── Cargo.toml
    └── .env
```


### Resource
[Refinery docs](https://deepwiki.com/rust-db/refinery)

[axum-quick-tutorial](https://www.shuttle.dev/blog/2023/12/06/using-axum-rust)

[better-axum-tutorial](https://www.rustfinity.com/blog/axum-rust-tutorial)

[tokio:postgress wiki](https://deepwiki.com/rust-postgres/rust-postgres/6.1-connection-strings-and-config-builder)

[deadpool postgres](https://docs.rs/deadpool-postgres/latest/deadpool_postgres/)

[tokio-postgress](https://docs.rs/tokio-postgres/latest/tokio_postgres/)

[graphql-reference](https://romankudryashov.com/blog/2020/12/graphql-rust/#_overview)

[jwt](https://github.com/Keats/jsonwebtoken)

[argon2](https://crates.io/crates/argon2)
[random](https://docs.rs/rand/latest/rand/rngs/index.html)


https://rustify.rs/glossary/argon2
