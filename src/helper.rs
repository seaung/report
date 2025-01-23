use futures::stream::{self, StreamExt};
use std::sync::Arc;
use tokio_util::compat::TokioAsyncReadCompatExt;

#[async_trait::async_trait]
pub trait DatabaseHelper {
    fn connect(&self, host: &str, port: u32) -> bool;
    fn violence_report(&self, host: &str, port: u32, pass: &str) -> bool;
    async fn violence_report_concurrent(&self, host: &str, port: u32, passwords: Vec<String>, concurrency: usize) -> Vec<String>;
    fn unauthorized_report(&self, host: &str, port: u32) -> bool;
}

pub struct Mssql;

#[async_trait::async_trait]
impl DatabaseHelper for Mssql {
    fn connect(&self, host: &str, port: u32) -> bool {
        let connection_string = format!("server={}:{};user id=sa", host, port);
        match tiberius::Config::from_ado_string(&connection_string)
            .map_err(|e| e.to_string())
            .and_then(|config| {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async {
                        let tcp = tokio::net::TcpStream::connect(format!("{host}:{port}")).await.map_err(|e| e.to_string())?;
                        let tcp = tcp.compat();
                        tiberius::Client::connect(config, tcp).await
                            .map_err(|e| e.to_string())
                    })
            }) {
            Ok(_) => {
                println!("成功连接到MSSQL数据库 {}:{}", host, port);
                true
            },
            Err(e) => {
                println!("连接MSSQL数据库失败: {}", e);
                false
            }
        }
    }

    fn violence_report(&self, host: &str, port: u32, pass: &str) -> bool {
        let connection_string = format!("server={}:{};user id=sa;password={}", host, port, pass);
        match tiberius::Config::from_ado_string(&connection_string)
            .map_err(|e| e.to_string())
            .and_then(|config| {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async {
                        let tcp = tokio::net::TcpStream::connect(format!("{host}:{port}")).await.map_err(|e| e.to_string())?;
                        let tcp = tcp.compat();
                        tiberius::Client::connect(config, tcp).await
                            .map_err(|e| e.to_string())
                    })
            }) {
            Ok(_) => {
                println!("使用密码 {} 成功连接到MSSQL数据库 {}:{}", pass, host, port);
                true
            },
            Err(e) => {
                println!("使用密码 {} 连接MSSQL数据库失败: {}", pass, e);
                false
            }
        }
    }

    async fn violence_report_concurrent(&self, host: &str, port: u32, passwords: Vec<String>, concurrency: usize) -> Vec<String> {
        let mut successful_passwords = Vec::new();
        let host = host.to_string();
        let this = Arc::new(self);
        
        stream::iter(passwords)
            .map(|pass| {
                let this = Arc::clone(&this);
                let host = host.clone();
                async move {
                    if this.violence_report(&host, port, &pass) {
                        Some(pass)
                    } else {
                        None
                    }
                }
            })
            .buffer_unordered(concurrency)
            .for_each(|result| {
                if let Some(pass) = result {
                    successful_passwords.push(pass.clone());
                }
                async {}
            })
            .await;
        
        successful_passwords
    }

    fn unauthorized_report(&self, host: &str, port: u32) -> bool {
        self.connect(host, port)
    }
}

pub struct Mysql;

#[async_trait::async_trait]
impl DatabaseHelper for Mysql {
    async fn violence_report_concurrent(&self, host: &str, port: u32, passwords: Vec<String>, concurrency: usize) -> Vec<String> {
        let mut successful_passwords = Vec::new();
        let host = host.to_string();
        let this = Arc::new(self);
        
        stream::iter(passwords)
            .map(|pass| {
                let this = Arc::clone(&this);
                let host = host.clone();
                async move {
                    if this.violence_report(&host, port, &pass) {
                        Some(pass)
                    } else {
                        None
                    }
                }
            })
            .buffer_unordered(concurrency)
            .for_each(|result| {
                if let Some(pass) = result {
                    successful_passwords.push(pass.clone());
                }
                async {}
            })
            .await;
        
        successful_passwords
    }

    fn connect(&self, host: &str, port: u32) -> bool {
        let opts = mysql::OptsBuilder::new()
            .ip_or_hostname(Some(host))
            .tcp_port(port as u16)
            .user(Some("root"))
            .pass(Some(""));
        
        match mysql::Conn::new(opts) {
            Ok(_) => {
                println!("成功连接到MySQL数据库 {}:{}", host, port);
                true
            },
            Err(e) => {
                println!("连接MySQL数据库失败: {}", e);
                false
            }
        }
    }

    fn violence_report(&self, host: &str, port: u32, pass: &str) -> bool {
        let opts = mysql::OptsBuilder::new()
            .ip_or_hostname(Some(host))
            .tcp_port(port as u16)
            .user(Some("root"))
            .pass(Some(pass));
        
        match mysql::Conn::new(opts) {
            Ok(_) => {
                println!("使用密码 {} 成功连接到MySQL数据库 {}:{}", pass, host, port);
                true
            },
            Err(e) => {
                println!("使用密码 {} 连接MySQL数据库失败: {}", pass, e);
                false
            }
        }
    }

    fn unauthorized_report(&self, host: &str, port: u32) -> bool {
        self.connect(host, port)
    }
}

pub struct Redis;

#[async_trait::async_trait]
impl DatabaseHelper for Redis {
    async fn violence_report_concurrent(&self, host: &str, port: u32, passwords: Vec<String>, concurrency: usize) -> Vec<String> {
        let mut successful_passwords = Vec::new();
        let host = host.to_string();
        let this = Arc::new(self);
        
        stream::iter(passwords)
            .map(|pass| {
                let this = Arc::clone(&this);
                let host = host.clone();
                async move {
                    if this.violence_report(&host, port, &pass) {
                        Some(pass)
                    } else {
                        None
                    }
                }
            })
            .buffer_unordered(concurrency)
            .for_each(|result| {
                if let Some(pass) = result {
                    successful_passwords.push(pass);
                }
                async {}
            })
            .await;
        
        successful_passwords
    }

    fn connect(&self, host: &str, port: u32) -> bool {
        let connection_string = format!("redis://{}:{}", host, port);
        match redis::Client::open(connection_string) {
            Ok(client) => match client.get_connection() {
                Ok(_) => {
                    println!("成功连接到Redis数据库 {}:{}", host, port);
                    true
                },
                Err(e) => {
                    println!("连接Redis数据库失败: {}", e);
                    false
                }
            },
            Err(e) => {
                println!("创建Redis客户端失败: {}", e);
                false
            }
        }
    }

    fn violence_report(&self, host: &str, port: u32, pass: &str) -> bool {
        let connection_string = format!("redis://{}:{}@{}:{}", "", pass, host, port);
        match redis::Client::open(connection_string) {
            Ok(client) => match client.get_connection() {
                Ok(_) => {
                    println!("使用密码 {} 成功连接到Redis数据库 {}:{}", pass, host, port);
                    true
                },
                Err(e) => {
                    println!("使用密码 {} 连接Redis数据库失败: {}", pass, e);
                    false
                }
            },
            Err(e) => {
                println!("创建Redis客户端失败: {}", e);
                false
            }
        }
    }

    fn unauthorized_report(&self, host: &str, port: u32) -> bool {
        self.connect(host, port)
    }
}

pub struct PostgreSql;

#[async_trait::async_trait]
impl DatabaseHelper for PostgreSql {
    async fn violence_report_concurrent(&self, host: &str, port: u32, passwords: Vec<String>, concurrency: usize) -> Vec<String> {
        let mut successful_passwords = Vec::new();
        let host = host.to_string();
        let this = Arc::new(self);
        
        stream::iter(passwords)
            .map(|pass| {
                let this = Arc::clone(&this);
                let host = host.clone();
                async move {
                    if this.violence_report(&host, port, &pass) {
                        Some(pass)
                    } else {
                        None
                    }
                }
            })
            .buffer_unordered(concurrency)
            .for_each(|result| {
                if let Some(pass) = result {
                    successful_passwords.push(pass);
                }
                async {}
            })
            .await;
        
        successful_passwords
    }

    fn connect(&self, host: &str, port: u32) -> bool {
        let connection_string = format!("postgres://postgres@{}:{}/postgres", host, port);
        match postgres::Client::connect(&connection_string, postgres::NoTls) {
            Ok(_) => {
                println!("成功连接到PostgreSQL数据库 {}:{}", host, port);
                true
            },
            Err(e) => {
                println!("连接PostgreSQL数据库失败: {}", e);
                false
            }
        }
    }

    fn violence_report(&self, host: &str, port: u32, pass: &str) -> bool {
        let connection_string = format!("postgres://postgres:{}@{}:{}/postgres", pass, host, port);
        match postgres::Client::connect(&connection_string, postgres::NoTls) {
            Ok(_) => {
                println!("使用密码 {} 成功连接到PostgreSQL数据库 {}:{}", pass, host, port);
                true
            },
            Err(e) => {
                println!("使用密码 {} 连接PostgreSQL数据库失败: {}", pass, e);
                false
            }
        }
    }

    fn unauthorized_report(&self, host: &str, port: u32) -> bool {
        self.connect(host, port)
    }
}

pub struct Elasticsearch;

#[async_trait::async_trait]
impl DatabaseHelper for Elasticsearch {
    async fn violence_report_concurrent(&self, host: &str, port: u32, passwords: Vec<String>, concurrency: usize) -> Vec<String> {
        let mut successful_passwords = Vec::new();
        let host = host.to_string();
        let this = Arc::new(self);
        
        stream::iter(passwords)
            .map(|pass| {
                let this = Arc::clone(&this);
                let host = host.clone();
                async move {
                    if this.violence_report(&host, port, &pass) {
                        Some(pass)
                    } else {
                        None
                    }
                }
            })
            .buffer_unordered(concurrency)
            .for_each(|result| {
                if let Some(pass) = result {
                    successful_passwords.push(pass.clone());
                }
                async {}
            })
            .await;
        
        successful_passwords
    }

    fn connect(&self, host: &str, port: u32) -> bool {
        let url = format!("http://{}:{}", host, port);
        let client = reqwest::blocking::Client::new();
        match client.get(&url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    println!("成功连接到Elasticsearch {}:{}", host, port);
                    true
                } else {
                    println!("连接Elasticsearch失败: 服务器返回错误状态码");
                    false
                }
            },
            Err(e) => {
                println!("连接Elasticsearch失败: {}", e);
                false
            }
        }
    }

    fn violence_report(&self, host: &str, port: u32, pass: &str) -> bool {
        let url = format!("http://{}:{}", host, port);
        let client = reqwest::blocking::Client::new();
        match client.get(&url).basic_auth("elastic", Some(pass)).send() {
            Ok(response) => {
                if response.status().is_success() {
                    println!("使用密码 {} 成功连接到Elasticsearch {}:{}", pass, host, port);
                    true
                } else {
                    println!("使用密码 {} 连接Elasticsearch失败: 认证失败", pass);
                    false
                }
            },
            Err(e) => {
                println!("使用密码 {} 连接Elasticsearch失败: {}", pass, e);
                false
            }
        }
    }

    fn unauthorized_report(&self, host: &str, port: u32) -> bool {
        self.connect(host, port)
    }
}

pub struct MongoDB;

#[async_trait::async_trait]
impl DatabaseHelper for MongoDB {
    async fn violence_report_concurrent(&self, host: &str, port: u32, passwords: Vec<String>, concurrency: usize) -> Vec<String> {
        let mut successful_passwords = Vec::new();
        let host = host.to_string();
        let this = Arc::new(self);
        
        stream::iter(passwords)
            .map(|pass| {
                let this = Arc::clone(&this);
                let host = host.clone();
                async move {
                    if this.violence_report(&host, port, &pass) {
                        Some(pass)
                    } else {
                        None
                    }
                }
            })
            .buffer_unordered(concurrency)
            .for_each(|result| {
                if let Some(pass) = result {
                    successful_passwords.push(pass.clone());
                }
                async {}
            })
            .await;
        
        successful_passwords
    }

    fn connect(&self, host: &str, port: u32) -> bool {
        let connection_string = format!("mongodb://{}:{}", host, port);
        match tokio::runtime::Runtime::new().unwrap().block_on(mongodb::Client::with_uri_str(&connection_string)) {
            Ok(_) => {
                println!("成功连接到MongoDB数据库 {}:{}", host, port);
                true
            },
            Err(e) => {
                println!("连接MongoDB数据库失败: {}", e);
                false
            }
        }
    }

    fn violence_report(&self, host: &str, port: u32, pass: &str) -> bool {
        let connection_string = format!("mongodb://{}:{}@{}:{}", "admin", pass, host, port);
        match tokio::runtime::Runtime::new().unwrap().block_on(mongodb::Client::with_uri_str(&connection_string)) {
            Ok(_) => {
                println!("使用密码 {} 成功连接到MongoDB数据库 {}:{}", pass, host, port);
                true
            },
            Err(e) => {
                println!("使用密码 {} 连接MongoDB数据库失败: {}", pass, e);
                false
            }
        }
    }

    fn unauthorized_report(&self, host: &str, port: u32) -> bool {
        self.connect(host, port)
    }
}

pub struct Memcached;

#[async_trait::async_trait]
impl DatabaseHelper for Memcached {
    async fn violence_report_concurrent(&self, host: &str, port: u32, passwords: Vec<String>, concurrency: usize) -> Vec<String> {
        let mut successful_passwords = Vec::new();
        let host = host.to_string();
        let this = Arc::new(self);
        
        let mut results: Vec<String> = Vec::new();
        for pass in passwords {
            if this.violence_report(&host, port, &pass) {
                successful_passwords.push(pass);
            }
        }
        
        successful_passwords
    }

    fn connect(&self, host: &str, port: u32) -> bool {
        let connection_string = format!("{host}:{port}");
        match memcache::connect(connection_string) {
            Ok(_) => {
                println!("成功连接到Memcached {}:{}", host, port);
                true
            },
            Err(e) => {
                println!("连接Memcached失败: {}", e);
                false
            }
        }
    }

    fn violence_report(&self, host: &str, port: u32, pass: &str) -> bool {
        let connection_string = format!("{host}:{port}");
        match memcache::connect(connection_string) {
            Ok(_) => {
                println!("成功连接到Memcached {}:{}", host, port);
                true
            },
            Err(e) => {
                println!("连接Memcached失败: {}", e);
                false
            }
        }
    }

    fn unauthorized_report(&self, host: &str, port: u32) -> bool {
        self.connect(host, port)
    }
}

pub struct Oracle;

#[async_trait::async_trait]
impl DatabaseHelper for Oracle {
    async fn violence_report_concurrent(&self, host: &str, port: u32, passwords: Vec<String>, concurrency: usize) -> Vec<String> {
        let mut successful_passwords = Vec::new();
        let host = host.to_string();
        let this = Arc::new(self);
        
        let mut results: Vec<String> = Vec::new();
        for pass in passwords {
            if this.violence_report(&host, port, &pass) {
                successful_passwords.push(pass);
            }
        }
        
        successful_passwords
    }

    fn connect(&self, host: &str, port: u32) -> bool {
        let username = "system";
        let password = "";
        let connect_string = format!("{host}:{port}/ORCL");
        match oracle::Connection::connect(username, password, &connect_string) {
            Ok(_) => {
                println!("成功连接到Oracle数据库 {}:{}", host, port);
                true
            },
            Err(e) => {
                println!("连接Oracle数据库失败: {}", e);
                false
            }
        }
    }

    fn violence_report(&self, host: &str, port: u32, pass: &str) -> bool {
        let username = "system";
        let connect_string = format!("{host}:{port}/ORCL");
        match oracle::Connection::connect(username, pass, &connect_string) {
            Ok(_) => {
                println!("使用密码 {} 成功连接到Oracle数据库 {}:{}", pass, host, port);
                true
            },
            Err(e) => {
                println!("使用密码 {} 连接Oracle数据库失败: {}", pass, e);
                false
            }
        }
    }

    fn unauthorized_report(&self, host: &str, port: u32) -> bool {
        self.connect(host, port)
    }
}
