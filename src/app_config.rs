use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct AppConfig {
    #[envconfig(from = "SERVER_PORT", default = "8080")]
    pub server_port: u16,

    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
}
