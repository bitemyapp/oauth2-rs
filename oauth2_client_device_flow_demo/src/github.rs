/*
RUST_BACKTRACE=1 RUST_LOG=debug,isahc=off cargo run -p oauth2_client_device_flow_demo --bin github -- 'YOUR_CLIENT_ID'
*/

use std::{env, error};

use http_api_isahc_client::IsahcClient;
use oauth2_client::device_authorization_grant::Flow;
use oauth2_github::{GithubProviderWithDevice, GithubScope};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    let client_id = env::args().nth(1).unwrap();

    run(client_id).await
}

async fn run(client_id: String) -> Result<(), Box<dyn error::Error>> {
    let scopes = vec![GithubScope::PublicRepo, GithubScope::UserEmail];

    let flow = Flow::new(IsahcClient::new()?, IsahcClient::new()?);
    let provider = GithubProviderWithDevice::new(client_id)?;

    let access_token_body = flow
        .execute(
            &provider,
            scopes,
            |user_code, verification_uri, _verification_uri_complete| {
                println!("open [{}] then input [{}]", verification_uri, user_code);
            },
        )
        .await?;

    println!("access_token_body: {:?}", access_token_body);

    Ok(())
}
