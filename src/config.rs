use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
    pub client: Client,
}

#[derive(Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Client {
    pub date_format: String,
}

/// Parses the configuration file into a `toml::Table` (a hashmap).
pub fn config() -> Result<Config, String> {
    let mut config_dir = dirs_next::config_dir().ok_or("Unable to locate config directory")?;
    config_dir.push("tasks");
    config_dir.push("config.toml");

    config_dir
        .try_exists()
        .map_err(|e| e.to_string())?
        .then(|| -> Result<Config, String> {
            let config = std::fs::read_to_string(config_dir).map_err(|e| e.to_string())?;
            toml::from_str::<Config>(&config).map_err(|e| e.to_string())
        })
        .ok_or("Configuration is invalid")?
}

/// Retrieves only the `date_format` configuration, to be used by the cli
/// parser.
pub fn date_format() -> Result<String, String> {
    let config = config()?;
    Ok(config.client.date_format)
}
