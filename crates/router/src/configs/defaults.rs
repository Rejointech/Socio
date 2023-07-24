impl Default for super::settings::Database {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            host: "localhost".into(),
            port: 5432,
            dbname: String::new(),
            pool_size: 5,
            connection_timeout: 10,
        }
    }
}

impl Default for super::settings::Server {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "localhost".into(),
            base_url: "127.0.0.1".into(),
        }
    }
}
