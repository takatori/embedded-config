use crate::env::Env;

use config::{Config, ConfigError, Environment, File, FileFormat::Toml};
use rust_embed::RustEmbed;
use serde::Deserialize;

const DEFAULT_CONFIG_FILE: &str = "Dev.toml";

#[derive(RustEmbed)]
#[folder = "config/"]
struct ConfigFile;

impl ConfigFile {
    /// 埋め込まれたファイルを文字列としてロードする
    fn load(filepath: &str) -> anyhow::Result<String> {
        let config_file_info = ConfigFile::get(filepath)
            .ok_or_else(|| ConfigError::NotFound(format!("{} not found", filepath)))?;
        let str = std::str::from_utf8(config_file_info.data.as_ref())?;
        Ok(str.to_string())
    }
}

pub fn config<'de, T: Deserialize<'de>>(env: Env) -> anyhow::Result<T> {
    let mut builder = Config::builder()
        .set_default("env", env.to_string())?
        .add_source(File::from_str(
            &ConfigFile::load(DEFAULT_CONFIG_FILE)?,
            Toml,
        ))
        .add_source(File::from_str(
            &ConfigFile::load(&format!("{}.toml", env))?,
            Toml,
        ));

    let configs = builder.build()?;
    configs.try_deserialize().map_err(|e| e.into())
}
