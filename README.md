# ZITADEL for Rust &emsp; [![Crates.io](https://img.shields.io/crates/v/zitadel)](https://crates.io/crates/zitadel) [![docs.rs](https://img.shields.io/docsrs/zitadel)](https://docs.rs/zitadel/latest/zitadel/) ![Crates.io](https://img.shields.io/crates/dv/zitadel) ![Crates.io](https://img.shields.io/crates/l/zitadel)

This repository contains the gRPC service clients and helpers/credentials/other utilities
for [ZITADEL](https://github.com/zitadel/zitadel).

The following features are present:
- API clients for communication with the ZITADEL API (calling gRPC methods)
- Credentials support for the API clients (access token and service account interceptors)
- OIDC Introspection support for [rocket](https://rocket.rs)
- OIDC Introspection support for [axum](https://github.com/tokio-rs/axum)

### Example

There exist a few examples in the `zitadel-examples` crate.
Go there to see the library in action, or head over to the
[documentation](https://docs.rs/zitadel/latest/zitadel/) to see the full API documentation.

### Compatibility Matrix

| ZITADEL Version | Rust Version | zitadel-rust Version | zitadel-gen Version |
|-----------------|--------------|----------------------|---------------------|
| 2.69.1          | 1.80.0+      | 5.5.2                | 2.69.1              |

### Development

After you clone the repository, you need ["just"](https://just.systems) to run
certain tasks. 
Generating the gRPC clients is done via `just generate-grpc` or `just`
(as it is configured to be the default action for just).

For the generation to complete successfully, you need to have ["buf"](https://buf.build)
and the required proto plugins installed.

Required tooling:

- [just](https://just.systems)
- [buf](https://buf.build)
- `protoc-gen-prost-crate`: `cargo install protoc-gen-prost-crate`

Installing the tools is also partially available via `just install-tools`.
#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the package by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sup>
