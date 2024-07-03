#[doc = "Application settings"]
#[derive(serde::Deserialize, Clone)]

pub struct Settings {
  pub application: AppSettings,
}

#[doc = "ettings for application (host, port, cors)"]
#[derive(serde::Deserialize, Clone)]
pub struct AppSettings {
  pub host: String,
  pub port: u16,
  pub cors_location: String,
  pub api_prefix: String,
}

#[doc = "Function to deserializes the application.yaml into setting"]
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
  let settings = config::Config::builder()
    .add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml))
    .build()?;
  settings.try_deserialize::<Settings>()
}

impl AppSettings {
  #[doc = "Retrieve the URL address of the application."]
  pub fn get_addr(&self) -> String {
      format!("{}:{}", self.host, self.port)
  }
}