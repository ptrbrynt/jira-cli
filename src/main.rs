extern crate clap;

use clap::{App, ArgMatches};

fn main() {
    get_args();
}

fn get_args() -> ArgMatches {
    App::new("jira")
        .version("1.0.0")
        .about("Command line client for Jira Cloud")
        .author("Peter B. <peter@ptrbrynt.com>")
        .get_matches()
}
