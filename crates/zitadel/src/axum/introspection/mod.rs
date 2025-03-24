//! The introspection module allows you to use the OAuth 2.0 Token Introspection flow to authenticate users against ZITADEL.
//!
//! Axum uses "extractors" and "middlewares" to intercept calls. To authenticate a user against ZITADEL, you can use the [IntrospectedUser].
//! Which enables an extractor workflow: [extractor](https://docs.rs/axum/latest/axum/extract/index.html)
//!
//! #### Configure Axum
//!
//! To use the introspection flow, you need to configure the [IntrospectionState] and add it to your [Router](https://docs.rs/axum/latest/axum/routing/struct.Router.html).
//! When a custom state is used, [FromRef](axum::extract::FromRef) must be implemented. See [IntrospectionState] for more Details.
//!
//! ```no_run
//! #
//! # use axum::body::Body;
//! # use axum::http::Request;
//! # use axum::response::IntoResponse;
//! # use axum::routing::get;
//! # use axum::Router;
//! # use tokio::net::TcpListener;
//! # use tokio::runtime::Builder;
//! # use std::net::SocketAddr;
//! #
//! # use zitadel::axum::introspection::{IntrospectionState, IntrospectionStateBuilder};
//! # async fn example() {
//! #     let introspection_state_builder = IntrospectionStateBuilder::new("https://zitadel-libraries-l8boqa.zitadel.cloud")
//! #        .with_basic_auth(
//! #            "194339055499018497@zitadel_rust_test",
//! #            "Ip56oGzxKL1rJ8JaleUVKL7qUlpZ1tqHQYRSd6JE1mTlTJ3pDkDzoObHdZsOg88B",
//! #        )
//! #        .build()
//! #        .await
//! #        .unwrap();
//! #
//! let app = Router::new().with_state(introspection_state_builder);
//!
//! let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
//! println!("listening on: {addr}");
//! let listener = TcpListener::bind(addr).await.unwrap();
//! axum::serve(listener, app.into_make_service())
//!     .await
//!     .unwrap();
//! # }
//! ```
//!
//! #### Use the [IntrospectedUser] extractor
//!
//! ```no_run
//! # use zitadel::axum::introspection::IntrospectedUser;
//! # use axum::response::IntoResponse;
//! #
//! async fn authed(user: IntrospectedUser) -> impl IntoResponse {
//!    format!(
//!        "Hello authorized user: {:?} with id {}",
//!        user.username, user.user_id
//!    )
//! }
//! ```
//!
//! See examples for the complete code

mod state;
mod state_builder;
mod user;

pub use state::IntrospectionState;
pub use state_builder::{IntrospectionStateBuilder, IntrospectionStateBuilderError};
pub use user::{IntrospectedUser, IntrospectionGuardError};
