use clap::{Arg, Command};
use ansi_term::Colour::{Blue, Green, Red, Yellow};

pub struct Logger;

impl Logger {
    pub fn error(message: &str) {
        println!("{} - {}", Red.bold().paint("[-]"), message);
    }

    pub fn info(message: &str) {
        println!("{} - {}", Blue.bold().paint("[+]"), message);
    }

    pub fn success(message: &str) {
        println!("{} - {}", Green.bold().paint("[*]"), message);
    }

    pub fn warnning(message: &str) {
        println!("{} - {}", Yellow.bold().paint("[!]"), message);
    }
}

pub struct Options;

impl Options {
    pub fn parse() {
        let matches = Command::new("Report Command-Line Application")
                .version("1.0.0#dev")
                .author("Author: Seaung <https://www.github.com/seaung>")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("mysql")
                        .about("report mysql")
                        .arg(
                            Arg::new("file")
                            .short('f')
                            .long("file")
                            .help("report mysql file."))
                )
                .subcommand(
                    Command::new("mssql")
                        .about("report mssql")
                        .arg(
                            Arg::new("file")
                            .short('f')
                            .long("file")
                            .help("report mssql")
                        )
                )
                .get_matches();

        match matches.subcommand() {}
    }
}
