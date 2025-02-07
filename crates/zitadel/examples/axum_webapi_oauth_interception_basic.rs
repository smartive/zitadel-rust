use std::net::SocketAddr;

use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;
use zitadel::axum::introspection::{IntrospectedUser, IntrospectionStateBuilder};

async fn unauthed() -> String {
    "Hello Unauthorized User".into()
}

//#[axum::debug_handler]
async fn authed(user: IntrospectedUser) -> impl IntoResponse {
    format!(
        "Hello authorized user: {:?} with id {}",
        user.username, user.user_id
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is = IntrospectionStateBuilder::new("https://zitadel-libraries-l8boqa.zitadel.cloud")
        .with_basic_auth(
            "194339055499018497@zitadel_rust_test",
            "Ip56oGzxKL1rJ8JaleUVKL7qUlpZ1tqHQYRSd6JE1mTlTJ3pDkDzoObHdZsOg88B",
        )
        .build()
        .await
        .unwrap();

    let app = Router::new()
        .route("/unauthed", get(unauthed))
        .route("/authed", get(authed))
        .with_state(is);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("listening on: {addr}");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
