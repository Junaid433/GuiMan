use thiserror::Error;

#[derive(Error, Debug)]
pub enum GuiManError {
    #[error("Command execution failed: {command}")]
    CommandFailed { command: String },

    #[error("IO error: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("UTF-8 conversion error: {source}")]
    Utf8Error {
        #[from]
        source: std::string::FromUtf8Error,
    },

    #[error("JSON parsing error: {source}")]
    JsonError {
        #[from]
        source: serde_json::Error,
    },

    #[error("Regex compilation error: {source}")]
    RegexError {
        #[from]
        source: regex::Error,
    },

    #[error("Package validation error: {message}")]
    ValidationError { message: String },

    #[error("Package not found: {package}")]
    PackageNotFound { package: String },

    #[error("Repository error: {message}")]
    RepositoryError { message: String },

    #[error("AUR error: {message}")]
    AurError { message: String },

    #[error("Dependency resolution error: {message}")]
    DependencyError { message: String },

    #[error("System operation failed: {operation}")]
    SystemError { operation: String },

    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    #[error("Polkit policy error: {message}")]
    PolkitError { message: String },

    #[error("Backup operation failed: {message}")]
    BackupError { message: String },

    #[error("File operation failed: {path}")]
    FileError { path: String },

    #[error("Network error: {source}")]
    NetworkError {
        #[from]
        source: reqwest::Error,
    },

    #[error("Parse error: {message}")]
    ParseError { message: String },

    #[error("Operation cancelled")]
    Cancelled,

    #[error("Unknown error: {message}")]
    Unknown { message: String },
}

impl From<GuiManError> for String {
    fn from(error: GuiManError) -> String {
        error.to_string()
    }
}

pub type Result<T> = std::result::Result<T, GuiManError>;
