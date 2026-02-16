#[derive(thiserror::Error)]
pub enum Error {
    #[error("invalid regex {0}")]
    Regex(#[from] regex::Error),
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error("failed to move file: {0}")]
    TempfilePersist(#[from] tempfile::PersistError),
    #[error(transparent)]
    Writer(#[from] crate::writer::Error),
    #[error(transparent)]
    Patcher(#[from] crate::patcher::Error),
    #[error(transparent)]
    Output(#[from] crate::output::Error),
    #[error("invalid UTF-8 in replacement: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

// pretty-print the error
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
