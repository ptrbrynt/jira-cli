use clap::ArgMatches;

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

pub fn authenticate(auth_data: AuthData) {
    println!("{:?}", auth_data);
}
