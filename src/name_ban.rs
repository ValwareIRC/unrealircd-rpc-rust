//! Name ban operations module.

use crate::connection::Connection;
use crate::error::Result;
use serde_json;

/// NameBan handler for name ban (QLine) operations.
#[derive(Clone)]
pub struct NameBan {
    connection: Connection,
}

impl NameBan {
    /// Create a new name ban handler.
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Add a name ban (QLine).
    pub async fn add(
        &self,
        name: &str,
        reason: &str,
        duration: Option<&str>,
        set_by: Option<&str>,
    ) -> Result<Option<serde_json::Value>> {
        let mut params = serde_json::json!({
            "name": name,
            "reason": reason,
            "duration_string": "0"
        });

        if let Some(d) = duration {
            params["duration_string"] = d.into();
        }
        if let Some(sb) = set_by {
            params["set_by"] = sb.into();
        }

        let result = self.connection.query("name_ban.add", params, false).await?;
        if let Some(tkl) = result.get("tkl") {
            Ok(Some(tkl.clone()))
        } else {
            Ok(None)
        }
    }

    /// Delete a ban.
    pub async fn delete(&self, name: &str) -> Result<Option<serde_json::Value>> {
        let result = self.connection.query("name_ban.del", serde_json::json!({"name": name}), false).await?;
        if let Some(tkl) = result.get("tkl") {
            Ok(Some(tkl.clone()))
        } else {
            Ok(None)
        }
    }

    /// Get a list of all bans.
    pub async fn get_all(&self) -> Result<serde_json::Value> {
        let result = self.connection.query("name_ban.list", serde_json::Value::Null, false).await?;
        if let Some(list) = result.get("list") {
            Ok(list.clone())
        } else {
            Err(crate::error::Error::InvalidResponse)
        }
    }

    /// Get a specific ban.
    pub async fn get(&self, name: &str) -> Result<Option<serde_json::Value>> {
        let result = self.connection.query("name_ban.get", serde_json::json!({"name": name}), false).await?;
        if let Some(tkl) = result.get("tkl") {
            Ok(Some(tkl.clone()))
        } else {
            Ok(None)
        }
    }
}