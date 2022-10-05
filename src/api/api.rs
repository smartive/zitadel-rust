//! [gRPC](https://grpc.io/) module that contains the ZITADEL API compiled proto definitions.

#![allow(clippy::all)]

/// Generated gRPC code for the ZITADEL API.
#[cfg(feature = "api")]
pub mod zitadel {
    pub mod v1 {
        tonic::include_proto!("zitadel.v1");

        pub mod v1 {
            tonic::include_proto!("zitadel.v1.v1");
        }
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
    pub mod authn {
        pub mod v1 {
            tonic::include_proto!("zitadel.authn.v1");
        }
    }
    pub mod auth {
        pub mod v1 {
            tonic::include_proto!("zitadel.auth.v1");
        }
    }
    pub mod change {
        pub mod v1 {
            tonic::include_proto!("zitadel.change.v1");
        }
    }
    pub mod idp {
        pub mod v1 {
            tonic::include_proto!("zitadel.idp.v1");
        }
    }
    pub mod instance {
        pub mod v1 {
            tonic::include_proto!("zitadel.instance.v1");
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
    pub mod settings {
        pub mod v1 {
            tonic::include_proto!("zitadel.settings.v1");
        }
    }
    pub mod system {
        pub mod v1 {
            tonic::include_proto!("zitadel.system.v1");
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
