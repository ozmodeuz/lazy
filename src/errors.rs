use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Base directories error: {0}")]
    BaseDirectoriesError(xdg::BaseDirectoriesError),
    #[error("IO error: {0}")]
    IOError(std::io::Error),
    #[error("Yaml error: {0}")]
    YamlError(serde_yaml::Error),
    #[error("Other error: {0}")]
    OtherError(String),
}
