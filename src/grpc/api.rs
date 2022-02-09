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

pub mod zitadel {
    pub mod v1 {
        tonic::include_proto!("zitadel.v1");
    }
    pub mod action {
        pub mod v1 {
            tonic::include_proto!("zitadel.action.v1");
        }
    }
    pub mod admin {
        pub mod v1 {
            tonic::include_proto!("zitadel.admin.v1");
        }
    }
    pub mod app {
        pub mod v1 {
            tonic::include_proto!("zitadel.app.v1");
        }
    }
    pub mod auth {
        pub mod v1 {
            tonic::include_proto!("zitadel.auth.v1");
        }
    }
    pub mod authn {
        pub mod v1 {
            tonic::include_proto!("zitadel.authn.v1");
        }
    }
    pub mod change {
        pub mod v1 {
            tonic::include_proto!("zitadel.change.v1");
        }
    }
    pub mod features {
        pub mod v1 {
            tonic::include_proto!("zitadel.features.v1");
        }
    }
    pub mod idp {
        pub mod v1 {
            tonic::include_proto!("zitadel.idp.v1");
        }
    }
    pub mod management {
        pub mod v1 {
            tonic::include_proto!("zitadel.management.v1");
        }
    }
    pub mod member {
        pub mod v1 {
            tonic::include_proto!("zitadel.member.v1");
        }
    }
    pub mod metadata {
        pub mod v1 {
            tonic::include_proto!("zitadel.metadata.v1");
        }
    }
    pub mod org {
        pub mod v1 {
            tonic::include_proto!("zitadel.org.v1");
        }
    }
    pub mod policy {
        pub mod v1 {
            tonic::include_proto!("zitadel.policy.v1");
        }
    }
    pub mod project {
        pub mod v1 {
            tonic::include_proto!("zitadel.project.v1");
        }
    }
    pub mod text {
        pub mod v1 {
            tonic::include_proto!("zitadel.text.v1");
        }
    }
    pub mod user {
        pub mod v1 {
            tonic::include_proto!("zitadel.user.v1");
        }
    }
}
