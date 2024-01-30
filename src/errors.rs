#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),

    #[error(transparent)]
    EnvError(#[from] dotenv::Error),

    #[error(transparent)]
    TeloxideError(#[from] teloxide::RequestError),

    #[error(transparent)]
    ParseLevelError(#[from] log::ParseLevelError),

    #[error(transparent)]
    InMemStorageError(#[from] teloxide::dispatching::dialogue::InMemStorageError),

    #[error(transparent)]
    SetLoggerError(#[from] log::SetLoggerError),

    #[error(transparent)]
    RegexError(#[from] regex::Error),
}

pub type Result<T> = std::result::Result<T, Errors>;
