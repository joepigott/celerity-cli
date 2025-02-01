use toml::Table;

pub fn config() -> Option<Table> {
    let mut config_dir = dirs_next::config_dir()?;
    config_dir.push("tasks");
    config_dir.push("config.toml");

    config_dir.try_exists().ok()?.then(|| -> Option<Table> {
        let config = std::fs::read_to_string(config_dir).ok()?;
        config.parse::<Table>().ok()
    })?
}
