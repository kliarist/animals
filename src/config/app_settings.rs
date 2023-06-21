use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct AppSettings {
    #[envconfig(from = "SERVER_PORT", default = "8080")]
    server_port: u16,

    #[envconfig(from = "DATABASE_URL")]
    database_url: String,

    #[envconfig(from = "RUST_LOG", default = "info")]
    log_level: String,
}

impl AppSettings {
    pub fn server_port(&self) -> u16 {
        self.server_port
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn log_level(&self) -> &str {
        &self.log_level
    }
}
