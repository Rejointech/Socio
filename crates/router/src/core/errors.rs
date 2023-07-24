use config::ConfigError;
use thiserror;

pub type ApplicationResult<T> = Result<T, ApplicationError>;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    // Display's impl can be overridden by the attribute error marco.
    // Don't use Debug here, Debug gives error stack in response.
    #[error("Application configuration error: {0}")]
    ConfigurationError(ConfigError),

    #[error("I/O: {0}")]
    IoError(std::io::Error),
}

impl From<ConfigError> for ApplicationError {
    fn from(err: ConfigError) -> Self {
        Self::ConfigurationError(err)
    }
}

impl From<std::io::Error> for ApplicationError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
