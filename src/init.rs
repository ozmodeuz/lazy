use super::config::{get_config_directory, get_config_file, parse_config_file, Config};
use super::errors::Error;

pub fn init() -> Result<Config, Error> {
    get_config_directory()
        .and_then(|p| get_config_file(p))
        .and_then(|p| parse_config_file(p))
    //TODO: do rest of init steps...
}
