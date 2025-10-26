//! # UnrealIRCd RPC Rust Client
//!
//! This library provides a Rust client for controlling UnrealIRCd via its JSON-RPC interface.
//!
//! ## Example
//!
//! ```rust,no_run
//! use unrealircd_rpc::{Connection, Options};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut conn = Connection::new(
//!         "wss://127.0.0.1:8600/".to_string(),
//!         "username:password".to_string(),
//!         Some(Options {
//!             tls_verify: false,
//!             issuer: None,
//!         }),
//!     );
//!
//!     conn.connect().await?;
//!
//!     let bans = conn.server_ban().get_all().await?;
//!     let users = conn.user().get_all(2).await?;
//!
//!     Ok(())
//! }
//! ```

pub mod connection;
pub mod error;
pub mod rpc;
pub mod server;
pub mod user;
pub mod channel;
pub mod server_ban;
pub mod spamfilter;
pub mod name_ban;
pub mod log;
pub mod stats;
pub mod server_ban_exception;

pub use connection::{Connection, Options};
pub use error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_creation() {
        let conn = Connection::new(
            "wss://127.0.0.1:8600/".to_string(),
            "user:pass".to_string(),
            None,
        );

        assert_eq!(conn.uri(), "wss://127.0.0.1:8600/");
        assert_eq!(conn.auth_header(), "Basic dXNlcjpwYXNz"); // base64 encoded "user:pass"
    }

    #[test]
    fn test_options_default() {
        let options = Options::default();
        assert_eq!(options.tls_verify, true);
        assert_eq!(options.issuer, None);
    }
}