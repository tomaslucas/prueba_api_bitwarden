use bitwarden::{
    auth::request::AccessTokenLoginRequest,
    client::client_settings::{ClientSettings, DeviceType},
    error::Result,
    secrets_manager::secrets::SecretGetRequest,
    Client,
};

//use std::env;
use dotenv::dotenv;
use uuid::Uuid;

async fn authenticate() -> Result<Client> {
    let settings = ClientSettings {
        identity_url: "https://identity.bitwarden.com".to_string(),
        api_url: "https://api.bitwarden.com".to_string(),
        user_agent: "Bitwarden Rust-SDK".to_string(),
        device_type: DeviceType::SDK,
    };
    let mut client = Client::new(Some(settings));

    // Before we operate, we need to authenticate with a token
    let token = AccessTokenLoginRequest {
        access_token: std::env::var("BWS_ACCESS_TOKEN").expect("BWS_ACCESS_TOKEN not found."),
    };
    client.access_token_login(&token).await.unwrap();
    Ok(client)
}

async fn get_secret(
    client: &mut Client,
    secret_id: Uuid,
) -> bitwarden::secrets_manager::secrets::SecretResponse {
    let sec_id = SecretGetRequest { id: secret_id };
    client.secrets().get(&sec_id).await.unwrap()
}

#[tokio::main]
async fn main() -> Result<(), bitwarden::error::Error> {
    dotenv().ok();
    // Use string regarding to Secret test
    let mut client = authenticate().await.unwrap();

    let uuid_id =
        Uuid::try_parse(&std::env::var("BWS_SECRET_TEST").expect("BWS_SECRET_TEST not found."))
            .unwrap();

    let sec_id = get_secret(&mut client, uuid_id).await;
    // Test
    assert_eq!(sec_id.value, "Prueba de secreto número 1.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_secret() -> Result<()> {
        // Load environment variables from .env
        dotenv().ok();
        // Create a conection
        let mut client = authenticate().await.unwrap();

        let uuid_id =
            Uuid::try_parse(&std::env::var("BWS_SECRET_TEST").expect("BWS_SECRET_TEST not found."))
                .unwrap();
        // Get the secret
        let sec_id = get_secret(&mut client, uuid_id).await;

        assert_eq!(sec_id.value, "Prueba de secreto número 1.");
        Ok(())
    }
}
