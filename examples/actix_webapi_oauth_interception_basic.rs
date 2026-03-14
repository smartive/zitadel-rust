use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use zitadel::actix::introspection::{IntrospectedUser, IntrospectionConfigBuilder};

#[get("/unauthed")]
async fn unauthed() -> impl Responder {
    println!("Hello Unauthorized User!");
    HttpResponse::Ok().body("Hello Unauthorized User!")
}

#[get("/authed")]
async fn authed(user: IntrospectedUser) -> impl Responder {
    println!("Hello Authorized User!");
    format!(
        "Hello Authorized {:?} with id {}",
        user.username, user.user_id
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server.");
    let auth = IntrospectionConfigBuilder::new("https://zitadel-libraries-l8boqa.zitadel.cloud")
        .with_basic_auth(
            "194339055499018497@zitadel_rust_test",
            "Ip56oGzxKL1rJ8JaleUVKL7qUlpZ1tqHQYRSd6JE1mTlTJ3pDkDzoObHdZsOg88B",
        )
        .build()
        .await
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(auth.clone())
            .service(unauthed)
            .service(authed)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
