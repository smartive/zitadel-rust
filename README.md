# ZITADEL for Rust &emsp; [![Crates.io](https://img.shields.io/crates/v/zitadel)](https://crates.io/crates/zitadel) [![docs.rs](https://img.shields.io/docsrs/zitadel)](https://docs.rs/zitadel/latest/zitadel/) ![Crates.io](https://img.shields.io/crates/dv/zitadel) ![Crates.io](https://img.shields.io/crates/l/zitadel)

This repository contains the gRPC bindings for [ZITADEL](https://zitadel.ch)
and other helpers to interact with the ZITADEL API.

With the help of the service account it is possible to access the ZITADEL API and use the management client to execute
gRPC calls against the API.

## Example

Fetch the first 1000 users in the organization of a service account:

```rust
use zitadel::credentials::ServiceAccount;
use zitadel::grpc::zitadel::management::v1::management_service_client::ManagementServiceClient;

#[tokio::main]
async fn main() {
    let service_account = ServiceAccount::load_from_file("./my_json_key.json")?;
    let client = ManagementServiceClient.connect("https://api.zitadel.ch");
    let access_token = service_account.authenticate().await?;
    let req = tonic::Request::new(ListUsersRequest {
        query: Some(ListQuery {
            offset: progressed as u64,
            limit: 1000,
            asc: true,
        }),
        sorting_column: 0,
        queries: vec![],
    });
    req.metadata_mut()
        .insert("authorization", format!("Bearer {}", access_token)
            .parse()
            .unwrap());
    let response = client.list_users(req).await?;

    Ok(());
}
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the package by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sup>
