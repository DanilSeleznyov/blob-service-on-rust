use serde::Deserialize;
use std::{env, error::Error, fs};

const DEFAULT_CONFIG_PATH: &str = "config.yaml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

pub fn load() -> Result<Config, Box<dyn Error>> {
    let path = env::var("CONFIG_PATH").unwrap_or_else(|_| DEFAULT_CONFIG_PATH.to_owned());

    let text =
        fs::read_to_string(&path).map_err(|e| format!("failed to read config `{path}`: {e}"))?;

    let mut cfg: Config = serde_yaml_ng::from_str(&text)
        .map_err(|e| format!("failed to parse config `{path}`: {e}"))?;

    apply_env_overrides(&mut cfg)?;

    Ok(cfg)
}

fn apply_env_overrides(cfg: &mut Config) -> Result<(), Box<dyn Error>> {
    if let Ok(v) = env::var("SERVER_HOST") {
        cfg.server.host = v;
    }
    if let Ok(v) = env::var("SERVER_PORT") {
        cfg.server.port = v
            .parse()
            .map_err(|e| format!("invalid SERVER_PORT `{v}`: {e}"))?;
    }
    if let Ok(v) = env::var("DB_HOST") {
        cfg.database.host = v;
    }
    if let Ok(v) = env::var("DB_PORT") {
        cfg.database.port = v
            .parse()
            .map_err(|e| format!("invalid DB_PORT `{v}`: {e}"))?;
    }
    if let Ok(v) = env::var("DB_USER") {
        cfg.database.user = v;
    }
    if let Ok(v) = env::var("DB_PASSWORD") {
        cfg.database.password = v;
    }
    if let Ok(v) = env::var("DB_NAME") {
        cfg.database.name = v;
    }

    Ok(())
}
