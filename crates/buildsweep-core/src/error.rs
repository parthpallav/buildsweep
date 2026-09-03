use thiserror::Error;

#[derive(Debug, Error, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", content = "message")]
pub enum BuildSweepError {
    #[error("invalid path: {0}")]
    InvalidPath(String),
    #[error("path escapes scan root: {0}")]
    PathEscape(String),
    #[error("protected path: {0}")]
    ProtectedPath(String),
    #[error("unknown path cannot be cleaned: {0}")]
    UnknownPath(String),
    #[error("symlink or reparse point rejected: {0}")]
    SymlinkRejected(String),
    #[error("scan cancelled")]
    ScanCancelled,
    #[error("scan not found: {0}")]
    ScanNotFound(String),
    #[error("cleanup plan not found: {0}")]
    PlanNotFound(String),
    #[error("license error: {0}")]
    License(String),
    #[error("pro feature required: {0}")]
    ProRequired(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<std::io::Error> for BuildSweepError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

pub type Result<T> = std::result::Result<T, BuildSweepError>;
