# Fluvog
A distributed event streaming platform for log processing.

Fluvog is a from-scratch implementation of a distributed log-based event streaming system, built in Rust.
It is designed around append-only log storage, offset-based consumption, and a custom QUIC-based wire protocol,
with the goal of reaching production readiness.

## Features
- **Append-only log storage** — variable-length records with a length-prefixed binary format
- **Offset-based consumption** — consumers track their own read position, committed back to the broker
- **QUIC transport** — a custom binary protocol over QUIC (TLS 1.3 by default)

## Status
This project is under active development, working toward a production release.
The design and APIs may change as the project matures.

## Getting Started
```bash
# Build
cargo build

# Run tests
cargo test
```
*(Usage and deployment instructions will be added as the project matures.)*

## Design Notes
See [docs/](./docs) for notes on the log file format, indexing strategy, and protocol design decisions.

## Contributing
Contribution guidelines will be added as the project stabilizes.

## License
Licensed under either of
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
