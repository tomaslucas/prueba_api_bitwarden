use bitwarden::{
    auth::request::AccessTokenLoginRequest,
    client::client_settings::{ClientSettings, DeviceType},
    error::Result,
    secrets_manager::secrets::SecretGetRequest,
    Client,
};

//use std::env;
use dotenv::dotenv;
use std::error::Error;
use uuid::Uuid;

async fn authenticate(token: String) -> Result<Client, Box<dyn Error>> {
    let settings = ClientSettings {
        identity_url: "https://identity.bitwarden.com".to_string(),
        api_url: "https://api.bitwarden.com".to_string(),
        user_agent: "Bitwarden Rust-SDK".to_string(),
        device_type: DeviceType::SDK,
    };
    let mut client = Client::new(Some(settings));

    // Before we operate, we need to authenticate with a token
    let token = AccessTokenLoginRequest {
        access_token: std::env::var(&token)
            .unwrap_or_else(|_| panic!("Token: {token} not found, review .env file.")),
    };
    client.access_token_login(&token).await?;
    Ok(client)
}

async fn get_secret(
    client: &mut Client,
    secret_id: Uuid,
) -> Result<bitwarden::secrets_manager::secrets::SecretResponse, Box<dyn Error>> {
    let sec_id = SecretGetRequest { id: secret_id };
    Ok(client.secrets().get(&sec_id).await?)
}

async fn get_uuid(secret_id: String) -> Result<Uuid, Box<dyn Error>> {
    Ok(Uuid::try_parse(&std::env::var(&secret_id).unwrap_or_else(
        |_| panic!("Secret: {secret_id} not found, review .env file."),
    ))?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    // Use string regarding to Secret test
    let token: String = String::from("BWS_ACCESS_TOKEN");
    let mut client = authenticate(token).await?;

    let secret_id: String = String::from("BWS_SECRET");
    let uuid_id = get_uuid(secret_id).await?;

    let sec_id = get_secret(&mut client, uuid_id).await?;

    // Use sec_id.value

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_secret() -> Result<(), Box<dyn Error>> {
        // Load environment variables from .env
        dotenv().ok();
        let token: String = String::from("BWS_ACCESS_TOKEN");
        // Create a conection
        let mut client = authenticate(token).await?;

        let secret_id: String = String::from("BWS_SECRET_TEST");
        let uuid_id = get_uuid(secret_id).await?;
        // Get the secret
        let sec_id = get_secret(&mut client, uuid_id).await?;

        assert_eq!(sec_id.value, "Prueba de secreto número 1.");
        Ok(())
    }
}
