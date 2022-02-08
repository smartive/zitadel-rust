#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/caos/zitadel/main/console/src/favicon.ico",
    html_logo_url = "https://raw.githubusercontent.com/caos/zitadel/main/console/src/assets/images/zitadel-logo-solo-light.svg",
    issue_tracker_base_url = "https://github.com/buehler/zitadel-rust/issues/"
)]
//! This is the [ZITADEL](https://zitadel.ch/) API package for rust.
//! It does provide the gRPC clients to access the ZITADEL API and contains other
//! elements around ZITADEL.
//!
//! # ZITADEL
//!
//! [ZITADEL](https://zitadel.ch/) is an open source IAM (identity and access management) system.
//! With this crate, it is possible to access the API of ZITADEL itself
//! on the SaaS cloud instance or any self-hosted variant of ZITADEL.
//!
//! # API Access
//!
//! To use this crate for accessing the ZITADEL API, you may use the provided
//! clients and their shortcuts to create a client that directly authenticates
//! itself against the API. The `*.proto` files are directly fetched from
//! [the ZITADEL repository](https://github.com/caos/zitadel).
//!
//! ## Getting Started
//!
//! This quickstart guide will show how to use a service account to create a service client
//! and then fetch the profile of the service account. The
//! [service account][credentials::ServiceAccount] is able to perform the authentication
//! based on [JWT Profile](https://datatracker.ietf.org/doc/html/rfc7523).
//!
//! ### Create a Service Account
//!
//! To be able to access the ZITADEL API, you are required to create a service account
//! within a ZITADEL organization. Perform the following steps to create a service account:
//!
//! 1. Head over to the [zitadel console](https://console.zitadel.ch) and log in with
//!    an organization owner or user manager.
//! 2. Under ["Service Users"](https://console.zitadel.ch/users/list/machines) create
//!    a new service user.
//! 3. Create a JSON key for the service user and download it.
//!
//! ### Access the API
//!
//! After you created a service account on ZITADEL, you can use the downloaded JSON key
//! to access the API with a gRPC client. You can either load the service account
//! [from the file][credentials::ServiceAccount::load_from_file()] or
//! [from the JSON string][credentials::ServiceAccount::load_from_json()].
//!
//! Then, we can fetch the profile of the service account (json inline is hidden):
//!
//! ```
//! # const SERVICE_ACCOUNT: &'static str = r#"
//! # {
//! #     "type": "serviceaccount",
//! #     "keyId": "147183960294746926",
//! #     "key": "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEAy+yyAONERzgj0SNKEc2KRHQKhh+ekh/IKy1gZBy5l7L5JiAv\nzTY6+tY1PyGbkB2EkH3d0Gki5rIa4UrvjXyuOlUu+8uBG+rykVayhngnPZ+qO07W\nT8FC4YKiJABRm6hkLumrq/nckOno5RISdGt77+s02eGL33m0lr2SdIqj+ZWIJICK\n+keQne1llC3wqvsNJQiDnvRLar+w0wtJ95tOD2xz3met+0oQIpEt1UrNYgQQDirh\n45ih+pMvPR0zdx99nPWv920UXCM/KLGMveTZ4fDcF5tn1/2ghI6xNgrEZSKFNtdh\n8jijXAQvz7immVhGnAkxGX4AtXYyNgxK76aB6QIDAQABAoIBAHC5R3BMMJr5wnrB\n+hi7OJo8VvDrG5mErf6IF8dfRYxAp47WrfXO621q6YYbSsWwO24v1WR2KY/Cli9B\nYAgjCqA+JDmVtam8BxgmB4tjcbWTw+MC4l614wWLU5t4/aOAwthX3Mi01qLYWh/+\nDGuEWr81kkJ6dfozaYsGAaYgWSIF9Hqi5b3Lr1J34+SoDTLjOsqcdl57iIJeOw1X\nA0Gpp8JBfuDPmgS03GRMMXpb1YE4WMHpwyIRAc06dJyJtBfqZwYCZKR3Ln2WS5qO\nvibmDOXysnfHVnrnrnKkgsdInSApMu8YOuiLtjnsAq7GEW0Ts3ebX0tttkW+z3tM\ncSF177ECgYEA2AFo6IzfFtDnPVE1OVV1OuhVoTUIr5yYTRw1E9kRrH2QmG2BSlyh\n7MoUEOtrSJASrUdafhtsyY7sXF46EazRFm6Yq6LOc/ubLouNFnPcXmxrKtHw8jHP\nVQNP4Zt1D0gnafqEO1+9/WR5lidVDV7N/980E7W5n1PkpdXR474G6N0CgYEA8a6n\ncXckju3E2krfUepTwcmMKOyYgYoj39zIoBIF/D+WlTRSc7q0bRWB/11J9pkoOsHD\ny/5bSolfK4VGKH/FloRcP8Z/lVrZ/HCbR/hw+EEgaWUVyqanNU5485oxgDi+OjTO\nCbpkx2iRueH9g4MbDVAJdurSsTkOL8SYTOguJn0CgYBvhJLP9OK8Wc/4pTNwTUF/\nzzFeUA4S9CrhLJ3uiFQKlK0RNP/aD2b94/pmHdS+mrs5wKvkjW4lxWcb7P3X4Dv2\nc9TYT+58jLq6VgvaOqjcCudtLQRTVgnvnw0fse3GnP7URST9rlldOAFZ1yafB4Id\nBvRQ9LJHor1aLMD27kWM9QKBgQDZpjxNRsq5nQ9Gt17eWnULdAKxaED/h/Q+yooy\n/Yg+XtWxkOkgJ+gMxO3Jl63gUpWUNKOrtmloesYmX2OLXWYH4zNgi9aiHqtpV8+/\nxNGYAK67u7kgQ20Z6I2sdBRYMMG/kYZr5FyV6Go0SH0STqOyHX4ohdkwmP1Zr2ao\n+/9z0QKBgF8G+sY3yqiqV10I8bfa4mX3El876dMdeZJJgpQigD8n9LXN3N44GEga\n2b2GCUvvMXo4qTqKhgIXe7Cc3wJ7LjGXGuhDZYwfQ0giHR5aNBRKYYnaibKQTOQ0\nQqPe2TmJZ3N+7YXUw6ZlnUqYgDCohh6idQbE6/WputEdyT/gmBlY\n-----END RSA PRIVATE KEY-----\n",
//! #     "userId": "147183953818739366"
//! # }
//! # "#;
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>>{
//!     use zitadel::credentials::ServiceAccount;
//!     use zitadel::grpc::clients::with_access_token::auth_client_with_account;
//!     use zitadel::grpc::zitadel::auth::v1::GetMyUserRequest;
//!     
//!     let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT)?;
//!     let mut client = auth_client_with_account(&sa).await?;
//!     let req = tonic::Request::new(GetMyUserRequest{});
//!     let res = client.get_my_user(req).await?;
//!     println!("{:#?}", &res);
//!     Ok(())
//! }
//! ```
//!
//! The following clients exist:
//! - [Management][grpc::zitadel::management::v1::management_service_client::ManagementServiceClient] (management of the organization)
//! - [Admin][grpc::zitadel::admin::v1::admin_service_client::AdminServiceClient] (management of ZITADEL itself, if you have access)
//! - [Auth][grpc::zitadel::auth::v1::auth_service_client::AuthServiceClient] (management of the own profile / user)
//!
//! To find other methods that are possible on the clients, go to their respective implementation.
//!
//! ### TLS
//!
//! *A special word for TLS connection*: Please ensure that you use [`tonic`][tonic] with
//! the `tls` features enabled such that the ca common roots are loaded. Otherwise you may encounter
//! HTTP2 errors.
//!
//! ```toml
//! tonic = { version = "x.x.x", features = ["tls", "tls-roots", "tls-roots-common"]}
//! ```

pub use crate::zitadel::*;

pub mod credentials;
pub mod grpc;
mod zitadel;
