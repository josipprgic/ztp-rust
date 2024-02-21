use secrecy::{ExposeSecret, Secret};

pub enum Environment {
    Production,
    Development
}

impl Environment {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Production => "production",
            Self::Development => "development"
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    
    fn try_from(s: String) -> Result<Environment, Self::Error> {
        match s.as_str() {
           "production" => Ok(Environment::Production),
           "development" => Ok(Environment::Development),
           _ => Err(format!("Failed to parse application env(from: {}) - falling back to development", s))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16
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
    let app_env = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_string())
        .try_into()
        .expect("Failed to parse application environment");

    let base_path = match app_env {
        Environment::Production => "/opt/ztp-rust/config",
        Environment::Development => "./config"
    };

    let settings = config::Config::builder()
        .add_source(config::File::new(format!("{}/configuration.yml", base_path).as_str(), config::FileFormat::Yaml))
        .add_source(config::File::new(format!("{}/configuration-{}.yml", base_path, app_env.as_str()).as_str(), config::FileFormat::Yaml))
        .build()
        .expect("Configuration file doesn't exist");
    
    settings.try_deserialize::<Settings>()
        .expect("Configuration can't be deserialized into Settings struct")
}
