use base64::encode;
use clap::ArgMatches;
use dirs::home_dir;
use reqwest;
use reqwest::blocking::Client;
use reqwest::header;
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::prelude::*;

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
pub fn authenticate(auth_data: AuthData) -> Result<(), Box<dyn Error>> {
    test_auth_data(&auth_data)?;
    save_auth_data(&auth_data)?;
    Ok(())
}

/// Returns the currently saved auth data
pub fn get_auth_data() -> Result<AuthData, Box<dyn Error>> {
    let mut home = home_dir().ok_or("Couldn't find home directory")?;
    home.push(".jira");
    let auth_file_contents = read_to_string(home.join("jira_auth"))?;
    let mut lines = auth_file_contents.lines();

    let read_error = "Invalid auth file. Please run jira auth to fix.";

    Ok(AuthData {
        domain: String::from(lines.next().ok_or(read_error)?),
        email: String::from(lines.next().ok_or(read_error)?),
        token: String::from(lines.next().ok_or(read_error)?),
    })
}

/// Attempts an API call to verify the correctness of the provided Auth data
fn test_auth_data(auth_data: &AuthData) -> reqwest::Result<()> {
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

/// Saves the given auth data to the user's home directory
fn save_auth_data(auth_data: &AuthData) -> Result<(), Box<dyn Error>> {
    let mut home = home_dir().ok_or("Couldn't find home directory")?;
    home.push(".jira");
    create_dir_all(&home)?;
    let mut file = File::create(home.join("jira_auth"))?;
    writeln!(file, "{}", auth_data.domain)?;
    writeln!(file, "{}", auth_data.email)?;
    writeln!(file, "{}", auth_data.token)?;
    Ok(())
}
