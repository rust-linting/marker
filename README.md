# Marker

A project trying to implement a stable linting API for Rust.

## Contributing

The internal structure of marker and its component is documented in [`docs/internal/`](./docs/internal/).

## Running

The project is currently only available in this GitHub repo. For a quick test, checkout the repo and run `cargo run --bin cargo_marker -- -l ./marker_lints`. This will start `cargo_marker`, load [`./marker_lints`](./marker_lints) as a *lint crate* and run it in this repository.

## License

Copyright (c) 2022-2022 Rust-linting

Rust-linting is distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE), [LICENSE-MIT](./LICENSE-MIT).
