pub struct Config {
    pub Port: u32,
    pub Host: String,
}

pub struct DBConfig {
    pub DBHost: String,
    pub DBPort: u32,
    pub DBName: String,
    pub DBUser: String,
    pub DBPass: String,
}

pub impl Config {
    pub fn LoadConfig(&self, filename: String) -> Self {
    }
}

pub impl DBConfig {
    pub fn LoadDBConfig(&self, filename: String) -> Self {
    }
}

pub mod config;
