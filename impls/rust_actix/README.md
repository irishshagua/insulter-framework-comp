# Insulter - Rust with Actix Web
Implementation of the Insulter API using [Rust](https://www.rust-lang.org/) and
the [Actix Web](https://actix.rs) framework. Very much a WIP at the moment...

## Dev
To build locally you need the rust toolsuite installed. Best to use [rustup](https://rustup.rs/).
Once setup you can use `cargo` as per below to build and run the app locally.
```bash
cargo run & # Wait for the line: Running `target/debug/rust-web-svc`
curl -i localhost:8088/insult/brian
```

Then to stop the app:
```bash
 kill $(lsof -ti tcp:8088)
```
