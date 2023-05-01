use clap::{Arg, Command};
use ansi_term::Colour::{Blue, Green, Red, Yellow};

enum db_enum {
    MYSQL;
    MSSQL;
    POSTGRESQL;
    REDIS;
    MEMCACHED;
    ELASTICSEACH;
    ORACLE;
    MONGODB;
};

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
                    Command::new("violence")
                        .about("暴力破解数据库!")
                        .arg(
                            Arg::new("type")
                            .short('T')
                            .long("Type")
                            .help("爆破数据库的类型"))
                        .arg(
                            Arg::new("file")
                            .short('F')
                            .long("File")
                            .help("爆破数据库字典文件")
                        )
                )
                .subcommand(
                    Command::new("unauthorized")
                        .about("数据库未认证爆破")
                        .arg(
                            Arg::new("type")
                            .short('T')
                            .long("type")
                            .help("数据库未认证漏洞爆破")
                        )
                )
                .get_matches();

        match matches.subcommand() {}
    }
}
