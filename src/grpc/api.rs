//! [gRPC](https://grpc.io/) module that contains the ZITADEL API clients and
//! constructors for clients that directly interact with a `service account` for
//! authentication. Beware that the root level module ("zitadel") is the entry point
//! into the gRPC world of the API. So all relevant calls/objects/enums/others can befound
//! in this root module.
//!
//! #Example
//!
//! You may instantiate a management client like this:
//! ```
//! use zitadel::grpc;
//! let client = grpc::zitadel::management::v1::management_service_client::ManagementServiceClient::connect("https:://api.zitadel.ch");
//! ```

#![allow(clippy::all)]

grpc_helper::generate_grpc_modules!(
    "zitadel.v1",
    "zitadel.action.v1",
    "zitadel.admin.v1",
    "zitadel.app.v1",
    "zitadel.auth.v1",
    "zitadel.authn.v1",
    "zitadel.change.v1",
    "zitadel.features.v1",
    "zitadel.idp.v1",
    "zitadel.management.v1",
    "zitadel.member.v1",
    "zitadel.metadata.v1",
    "zitadel.org.v1",
    "zitadel.policy.v1",
    "zitadel.project.v1",
    "zitadel.text.v1",
    "zitadel.user.v1",
);
