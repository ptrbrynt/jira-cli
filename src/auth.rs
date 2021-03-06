use base64::encode;
use clap::ArgMatches;
use clap::{App, Arg};
use dirs::home_dir;
use reqwest::blocking::Client;
use reqwest::header;
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::prelude::*;

const JIRA_DIR: &str = ".jira";
const AUTH_FILE_NAME: &str = "jira_auth";
const MISSING_HOME_ERROR: &str = "Couldn't find home directory";
#[allow(dead_code)]
const READ_ERROR: &str = "Invalid auth file. Please run jira auth to fix.";

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

pub fn auth_subcommand<'a>() -> App<'a> {
    App::new("auth")
        .alias("a")
        .about("Authenticate Jira CLI with your Jira account")
        .arg(
            Arg::with_name("domain")
                .about("Your Jira domain e.g. myteam.atlassian.net")
                .required(true)
                .short('d')
                .long("domain")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("email")
                .about("Your Jira email address")
                .required(true)
                .short('e')
                .long("email")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("token")
                .about("Your API token. You can generate one in your Jira account.")
                .required(true)
                .short('t')
                .long("token")
                .takes_value(true),
        )
}

/// Verifies and saves the user's authentication credentials
pub fn authenticate(auth_data: AuthData) -> Result<(), Box<dyn Error>> {
    test_auth_data(&auth_data)?;
    save_auth_data(&auth_data)?;
    Ok(())
}

/// Returns the currently saved auth data
#[allow(dead_code)]
pub fn get_auth_data() -> Result<AuthData, Box<dyn Error>> {
    let mut home = home_dir().ok_or(MISSING_HOME_ERROR)?;
    home.push(JIRA_DIR);
    let auth_file_contents = read_to_string(home.join(AUTH_FILE_NAME))?;
    let mut lines = auth_file_contents.lines();

    Ok(AuthData {
        domain: String::from(lines.next().ok_or(READ_ERROR)?),
        email: String::from(lines.next().ok_or(READ_ERROR)?),
        token: String::from(lines.next().ok_or(READ_ERROR)?),
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
    let mut home = home_dir().ok_or(MISSING_HOME_ERROR)?;
    home.push(JIRA_DIR);
    create_dir_all(&home)?;
    let mut file = File::create(home.join(AUTH_FILE_NAME))?;
    writeln!(file, "{}", auth_data.domain)?;
    writeln!(file, "{}", auth_data.email)?;
    writeln!(file, "{}", auth_data.token)?;
    Ok(())
}
