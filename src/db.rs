pub struct mssql;

impl mssql {
    pub fn connect_mssql(&self, host: &str, port: u32) -> bool {}
}

pub struct mysql;

impl mysql {
    pub fn connect_mysql(&self, host: &str, port: u32) -> bool {}
}

pub struct redis;

impl redis {
    pub fn connect_redist(&self, host: &str, port: u32) -> bool {}
}

pub struct postgresql;

impl postgresql {
    pub fn connect_postgresql(&self, host: &str, port: u32) -> bool {}
}

pub struct elasticsearch;

impl elasticsearch {
    pub fn connect_elasticsearch(&self, host: &str, port: u32) -> bool {}
}

pub struct mongodb;

impl mongodb {
    pub fn connect_mongo(&self, host: &str, port: u32) -> bool {}
}

pub struct memcached;

impl memcached {
    pub fn connect_memcached(&self, host: &str, port: u32) -> bool {}
}

pub struct oracle;

impl oracle {
    pub fn connect_oracle(&self, host: &str, port: u32) -> bool {}
}
