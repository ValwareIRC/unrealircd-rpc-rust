//! Error types for the UnrealIRCd RPC client.

/// Errors that can occur when using the UnrealIRCd RPC client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] http::Error),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Connection timeout")]
    Timeout,

    #[error("RPC error: code={code}, message={message}")]
    Rpc { code: i64, message: String },

    #[error("Invalid JSON-RPC response")]
    InvalidResponse,

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Authentication failed")]
    AuthFailed,

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;