extern crate base64;
extern crate clap;
extern crate dirs;
extern crate reqwest;

mod auth;

use auth::{auth_subcommand, authenticate, AuthData};
use clap::{App, ArgMatches};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_matches();

    if let ("auth", Some(subcommand)) = matches.subcommand() {
        authenticate(AuthData::from(subcommand))
    } else {
        Ok(())
    }
}

fn get_matches() -> ArgMatches {
    App::new("jira")
        .version("1.0.0")
        .about("Command line client for Jira Cloud")
        .author("Peter B. <peter@ptrbrynt.com>")
        .subcommand(auth_subcommand())
        .get_matches()
}
