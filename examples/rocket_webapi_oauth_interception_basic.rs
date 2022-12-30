use zitadel::rocket::introspection::{IntrospectedUser, IntrospectionConfigBuilder};

#[rocket::get("/unauthed")]
fn unauthed() -> &'static str {
    "Hello Unauthorized User"
}

#[rocket::get("/authed")]
fn authed(user: IntrospectedUser) -> String {
    format!(
        "Hello Authorized {:?} with id {}",
        user.username, user.user_id
    )
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket::build()
        .mount("/", rocket::routes![unauthed, authed])
        .manage(
            IntrospectionConfigBuilder::new("https://zitadel-libraries-l8boqa.zitadel.cloud")
                .with_basic_auth(
                    "194339055499018497@zitadel_rust_test",
                    "Ip56oGzxKL1rJ8JaleUVKL7qUlpZ1tqHQYRSd6JE1mTlTJ3pDkDzoObHdZsOg88B",
                )
                .build()
                .await
                .unwrap(),
        )
        .launch()
        .await?;

    Ok(())
}
