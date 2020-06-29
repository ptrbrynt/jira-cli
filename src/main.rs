extern crate clap;

mod auth;

use auth::{authenticate, AuthData};
use clap::{App, Arg, ArgMatches};

fn main() {
    let matches = get_matches();

    if let ("auth", Some(sub)) = matches.subcommand() {
        authenticate(AuthData::from(sub))
    }
}

fn get_matches() -> ArgMatches {
    App::new("jira")
        .version("1.0.0")
        .about("Command line client for Jira Cloud")
        .author("Peter B. <peter@ptrbrynt.com>")
        .subcommand(
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
                ),
        )
        .get_matches()
}
