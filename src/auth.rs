use base64::encode;
use clap::ArgMatches;
use reqwest::blocking::Client;
use reqwest::{header, Result};

/// Represents data required for authentication.
#[derive(Debug)]
pub struct AuthData {
    pub domain: String,
    pub email: String,
    pub token: String,
}

impl From<&ArgMatches> for AuthData {
    fn from(matches: &ArgMatches) -> Self {
        AuthData {
            domain: String::from(matches.value_of("domain").unwrap_or_default()),
            email: String::from(matches.value_of("email").unwrap_or_default()),
            token: String::from(matches.value_of("token").unwrap_or_default()),
        }
    }
}

/// Verifies and saves the user's authentication credentials
pub fn authenticate(auth_data: AuthData) -> Result<()> {
    test_auth_data(&auth_data)?;
    save_auth_data(&auth_data)?;
    Ok(())
}

/// Attempts an API call to verify the correctness of the provided Auth data
fn test_auth_data(auth_data: &AuthData) -> Result<()> {
    let mut headers = header::HeaderMap::new();

    let auth_token = encode(format!("{}:{}", auth_data.email, auth_data.token));
    let auth_header = format!("Basic {}", auth_token);

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&auth_header).unwrap(),
    );

    let client = Client::builder().default_headers(headers).build()?;

    client
        .get(&format!("https://{}/rest/api/3/myself", auth_data.domain))
        .send()?
        .error_for_status()
        .map(|_| ())
}

fn save_auth_data(auth_data: &AuthData) -> Result<()> {
    Ok(())
}
