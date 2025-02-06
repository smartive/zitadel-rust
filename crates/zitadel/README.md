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

Starting from 5.5.2, the crate zitadel-gen is responsible to generate the code for the API clients.
The generated code is then used by the zitadel crate to provide a more idiomatic Rust API.
The zitadel-gen version should always be the same as the ZITADEL version (SemVer).

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the package by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sup>

#### Thanks and Acknowledgements

This project is supported by [Christoph Bühler from smartive.ch](https://smartive.ch) and the crates owns him 99% of the hard work.
Thanks also to all notable contributors and the ZITADEL team for their support and feedback.