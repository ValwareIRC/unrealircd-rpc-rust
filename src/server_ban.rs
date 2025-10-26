//! Server ban operations module.

use crate::connection::Connection;
use crate::error::Result;
use serde_json;

/// ServerBan handler for server ban operations.
#[derive(Clone)]
pub struct ServerBan {
    connection: Connection,
}

impl ServerBan {
    /// Create a new server ban handler.
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Add a ban.
    pub async fn add(&self, name: &str, ban_type: &str, duration: &str, reason: &str) -> Result<Option<serde_json::Value>> {
        let result = self.connection.query("server_ban.add", serde_json::json!({
            "name": name,
            "type": ban_type,
            "reason": reason,
            "duration_string": duration
        }), false).await?;

        if let Some(tkl) = result.get("tkl") {
            Ok(Some(tkl.clone()))
        } else {
            Ok(None)
        }
    }

    /// Delete a ban.
    pub async fn delete(&self, name: &str, ban_type: &str) -> Result<Option<serde_json::Value>> {
        let result = self.connection.query("server_ban.del", serde_json::json!({
            "name": name,
            "type": ban_type
        }), false).await?;

        if let Some(tkl) = result.get("tkl") {
            Ok(Some(tkl.clone()))
        } else {
            Ok(None)
        }
    }

    /// Get a list of all bans.
    pub async fn get_all(&self) -> Result<serde_json::Value> {
        let result = self.connection.query("server_ban.list", serde_json::Value::Null, false).await?;
        if let Some(list) = result.get("list") {
            Ok(list.clone())
        } else {
            Err(crate::error::Error::InvalidResponse)
        }
    }

    /// Get a specific ban.
    pub async fn get(&self, name: &str, ban_type: &str) -> Result<Option<serde_json::Value>> {
        let result = self.connection.query("server_ban.get", serde_json::json!({
            "name": name,
            "type": ban_type
        }), false).await?;

        if let Some(tkl) = result.get("tkl") {
            Ok(Some(tkl.clone()))
        } else {
            Ok(None)
        }
    }
}