use clap::{Command, Arg};
use crate::utils::DictReader;
use crate::utils::Progress;
use crate::helper;
use crate::helper::DatabaseHelper;

pub struct Options;

impl Options {
    pub fn run() {
        let matches = Self::parse_args();

        match matches.subcommand() {
            Some(("mysql", sub_matches)) => {
                let host = sub_matches.get_one::<String>("host").unwrap();
                let port = sub_matches.get_one::<u32>("port").unwrap();

                if let Some(dict_file) = sub_matches.get_one::<String>("dict") {
                    let credentials = match DictReader::read_credentials(dict_file) {
                        Ok(creds) => creds,
                        Err(e) => {
                            println!("读取字典文件失败: {}", e);
                            return;
                        }
                    };

                    let sp = Progress::start("正在进行MySQL暴力破解...");
                    let db = helper::Mysql;
                    let passwords: Vec<String> = credentials.into_iter().map(|c| c.password).collect();
                    let success = tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(db.violence_report_concurrent(host, *port, passwords, 10));
                    Progress::stop(sp, !success.is_empty());
                } else {
                    let db = helper::Mysql;
                    db.unauthorized_report(host, *port);
                }
            },
            Some(("redis", sub_matches)) => {
                let host = sub_matches.get_one::<String>("host").unwrap();
                let port = sub_matches.get_one::<u32>("port").unwrap();

                if let Some(dict_file) = sub_matches.get_one::<String>("dict") {
                    let credentials = match DictReader::read_credentials(dict_file) {
                        Ok(creds) => creds,
                        Err(e) => {
                            println!("读取字典文件失败: {}", e);
                            return;
                        }
                    };

                    let sp = Progress::start("正在进行Redis暴力破解...");
                    let db = helper::Redis;
                    let passwords: Vec<String> = credentials.into_iter().map(|c| c.password).collect();
                    let success = tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(db.violence_report_concurrent(host, *port, passwords, 10));
                    Progress::stop(sp, !success.is_empty());
                } else {
                    let db = helper::Redis;
                    db.unauthorized_report(host, *port);
                }
            },
            Some(("mongodb", sub_matches)) => {
                let host = sub_matches.get_one::<String>("host").unwrap();
                let port = sub_matches.get_one::<u32>("port").unwrap();

                if let Some(dict_file) = sub_matches.get_one::<String>("dict") {
                    let credentials = match DictReader::read_credentials(dict_file) {
                        Ok(creds) => creds,
                        Err(e) => {
                            println!("读取字典文件失败: {}", e);
                            return;
                        }
                    };

                    let sp = Progress::start("正在进行MongoDB暴力破解...");
                    let db = helper::MongoDB;
                    let passwords: Vec<String> = credentials.into_iter().map(|c| c.password).collect();
                    let success = tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(db.violence_report_concurrent(host, *port, passwords, 10));
                    Progress::stop(sp, !success.is_empty());
                } else {
                    let db = helper::MongoDB;
                    db.unauthorized_report(host, *port);
                }
            },
            Some(("postgresql", sub_matches)) => {
                let host = sub_matches.get_one::<String>("host").unwrap();
                let port = sub_matches.get_one::<u32>("port").unwrap();

                if let Some(dict_file) = sub_matches.get_one::<String>("dict") {
                    let credentials = match DictReader::read_credentials(dict_file) {
                        Ok(creds) => creds,
                        Err(e) => {
                            println!("读取字典文件失败: {}", e);
                            return;
                        }
                    };

                    let sp = Progress::start("正在进行PostgreSQL暴力破解...");
                    let db = helper::PostgreSql;
                    let passwords: Vec<String> = credentials.into_iter().map(|c| c.password).collect();
                    let success = tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(db.violence_report_concurrent(host, *port, passwords, 10));
                    Progress::stop(sp, !success.is_empty());
                } else {
                    let db = helper::PostgreSql;
                    db.unauthorized_report(host, *port);
                }
            },
            Some(("elasticsearch", sub_matches)) => {
                let host = sub_matches.get_one::<String>("host").unwrap();
                let port = sub_matches.get_one::<u32>("port").unwrap();

                if let Some(dict_file) = sub_matches.get_one::<String>("dict") {
                    let credentials = match DictReader::read_credentials(dict_file) {
                        Ok(creds) => creds,
                        Err(e) => {
                            println!("读取字典文件失败: {}", e);
                            return;
                        }
                    };

                    let sp = Progress::start("正在进行Elasticsearch暴力破解...");
                    let db = helper::Elasticsearch;
                    let passwords: Vec<String> = credentials.into_iter().map(|c| c.password).collect();
                    let success = tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(db.violence_report_concurrent(host, *port, passwords, 10));
                    Progress::stop(sp, !success.is_empty());
                } else {
                    let db = helper::Elasticsearch;
                    db.unauthorized_report(host, *port);
                }
            },
            Some(("memcached", sub_matches)) => {
                let host = sub_matches.get_one::<String>("host").unwrap();
                let port = sub_matches.get_one::<u32>("port").unwrap();

                if let Some(dict_file) = sub_matches.get_one::<String>("dict") {
                    let credentials = match DictReader::read_credentials(dict_file) {
                        Ok(creds) => creds,
                        Err(e) => {
                            println!("读取字典文件失败: {}", e);
                            return;
                        }
                    };

                    let sp = Progress::start("正在进行Memcached暴力破解...");
                    let db = helper::Memcached;
                    let passwords: Vec<String> = credentials.into_iter().map(|c| c.password).collect();
                    let success = tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(db.violence_report_concurrent(host, *port, passwords, 10));
                    Progress::stop(sp, !success.is_empty());
                } else {
                    let db = helper::Memcached;
                    db.unauthorized_report(host, *port);
                }
            },
            Some(("oracle", sub_matches)) => {
                let host = sub_matches.get_one::<String>("host").unwrap();
                let port = sub_matches.get_one::<u32>("port").unwrap();

                if let Some(dict_file) = sub_matches.get_one::<String>("dict") {
                    let credentials = match DictReader::read_credentials(dict_file) {
                        Ok(creds) => creds,
                        Err(e) => {
                            println!("读取字典文件失败: {}", e);
                            return;
                        }
                    };

                    let sp = Progress::start("正在进行Oracle暴力破解...");
                    let db = helper::Oracle;
                    let passwords: Vec<String> = credentials.into_iter().map(|c| c.password).collect();
                    let success = tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(db.violence_report_concurrent(host, *port, passwords, 10));
                    Progress::stop(sp, !success.is_empty());
                } else {
                    let db = helper::Oracle;
                    db.unauthorized_report(host, *port);
                }
            },
            Some(("mssql", sub_matches)) => {
                let host = sub_matches.get_one::<String>("host").unwrap();
                let port = sub_matches.get_one::<u32>("port").unwrap();

                if let Some(dict_file) = sub_matches.get_one::<String>("dict") {
                    let credentials = match DictReader::read_credentials(dict_file) {
                        Ok(creds) => creds,
                        Err(e) => {
                            println!("读取字典文件失败: {}", e);
                            return;
                        }
                    };

                    let sp = Progress::start("正在进行MSSQL暴力破解...");
                    let db = helper::Mssql;
                    let passwords: Vec<String> = credentials.into_iter().map(|c| c.password).collect();
                    let success = tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(db.violence_report_concurrent(host, *port, passwords, 10));
                    Progress::stop(sp, !success.is_empty());
                } else {
                    let db = helper::Mssql;
                    db.unauthorized_report(host, *port);
                }
            },
            _ => unreachable!()
        }
    }

    fn parse_args() -> clap::ArgMatches {
        Command::new("Report Command-Line Application")
            .version("1.0.0#dev")
            .author("Author: Seaung <https://www.github.com/seaung>")
            .subcommand_required(true)
            .subcommand(
                Command::new("mysql")
                    .about("MySQL数据库安全检测")
                    .arg(Arg::new("host").required(true).help("MySQL主机地址"))
                    .arg(Arg::new("port").required(true).help("MySQL端口号"))
                    .arg(Arg::new("dict").required(false).help("字典文件路径"))
            )
            .subcommand(
                Command::new("redis")
                    .about("Redis数据库安全检测")
                    .arg(Arg::new("host").required(true).help("Redis主机地址"))
                    .arg(Arg::new("port").required(true).help("Redis端口号"))
                    .arg(Arg::new("dict").required(false).help("字典文件路径"))
            )
            .subcommand(
                Command::new("mongodb")
                    .about("MongoDB数据库安全检测")
                    .arg(Arg::new("host").required(true).help("MongoDB主机地址"))
                    .arg(Arg::new("port").required(true).help("MongoDB端口号"))
                    .arg(Arg::new("dict").required(false).help("字典文件路径"))
            )
            .subcommand(
                Command::new("postgresql")
                    .about("PostgreSQL数据库安全检测")
                    .arg(Arg::new("host").required(true).help("PostgreSQL主机地址"))
                    .arg(Arg::new("port").required(true).help("PostgreSQL端口号"))
                    .arg(Arg::new("dict").required(false).help("字典文件路径"))
            )
            .subcommand(
                Command::new("elasticsearch")
                    .about("Elasticsearch数据库安全检测")
                    .arg(Arg::new("host").required(true).help("Elasticsearch主机地址"))
                    .arg(Arg::new("port").required(true).help("Elasticsearch端口号"))
                    .arg(Arg::new("dict").required(false).help("字典文件路径"))
            )
            .subcommand(
                Command::new("memcached")
                    .about("Memcached数据库安全检测")
                    .arg(Arg::new("host").required(true).help("Memcached主机地址"))
                    .arg(Arg::new("port").required(true).help("Memcached端口号"))
                    .arg(Arg::new("dict").required(false).help("字典文件路径"))
            )
            .subcommand(
                Command::new("oracle")
                    .about("Oracle数据库安全检测")
                    .arg(Arg::new("host").required(true).help("Oracle主机地址"))
                    .arg(Arg::new("port").required(true).help("Oracle端口号"))
                    .arg(Arg::new("dict").required(false).help("字典文件路径"))
            )
            .subcommand(
                Command::new("mssql")
                    .about("MSSQL数据库安全检测")
                    .arg(Arg::new("host").required(true).help("MSSQL主机地址"))
                    .arg(Arg::new("port").required(true).help("MSSQL端口号"))
                    .arg(Arg::new("dict").required(false).help("字典文件路径"))
            )
            .get_matches()
    }
}