use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!("postgres://{}:{}@{}:{}/{}", 
                self.username, self.password.expose_secret(), self.host, self.port, self.database_name))
    }
}

pub fn must_load_configuration() -> Settings {
    let settings = config::Config::builder()
        .add_source(config::File::new("config/configuration.yml", config::FileFormat::Yaml))
        .build()
        .expect("Configuration file doesn't exist");
    
    settings.try_deserialize::<Settings>()
        .expect("Configuration can't be deserialized into Settings struct")
}
