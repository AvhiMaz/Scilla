use thiserror::Error;

pub type ScillaResult<T> = anyhow::Result<T>;

#[derive(Debug, Error)]
pub enum ScillaError {
    #[error("Scilla ScillaConfig path doesnt exists")]
    ConfigPathDoesntExists,
    #[error("Io error")]
    IoError(#[from] std::io::Error),
    #[error("Toml Parse error")]
    TomlParseError(#[from] toml::de::Error),
    #[error("Anyhow err")]
    Anyhow(#[from] anyhow::Error),
}
