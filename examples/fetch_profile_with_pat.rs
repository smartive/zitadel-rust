use zitadel::api::{clients::ClientBuilder, zitadel::auth::v1::GetMyUserRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const PERSONAL_ACCESS_TOKEN: &str =
        "dEnGhIFs3VnqcQU5D2zRSeiarB1nwH6goIKY0J8MWZbsnWcTuu1C59lW9DgCq1y096GYdXA";
    const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";

    let mut client = ClientBuilder::new(ZITADEL_URL)
        .with_access_token(PERSONAL_ACCESS_TOKEN)
        .build_auth_client()
        .await?;
    let user = client.get_my_user(GetMyUserRequest {}).await?.into_inner();
    println!("{:#?}", user);

    Ok(())
}
