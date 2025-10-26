//! Server ban exception operations module.

use crate::connection::Connection;
use crate::error::Result;
use serde_json;

/// ServerBanException handler for server ban exception operations.
#[derive(Clone)]
pub struct ServerBanException {
    connection: Connection,
}

impl ServerBanException {
    /// Create a new server ban exception handler.
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Add a ban exception.
    pub async fn add(
        &self,
        name: &str,
        exception_types: &str,
        reason: &str,
        set_by: Option<&str>,
        duration: Option<&str>,
    ) -> Result<Option<serde_json::Value>> {
        let mut params = serde_json::json!({
            "name": name,
            "exception_types": exception_types,
            "reason": reason
        });

        if let Some(sb) = set_by {
            params["set_by"] = sb.into();
        }
        if let Some(d) = duration {
            params["duration_string"] = d.into();
        }

        let result = self.connection.query("server_ban_exception.add", params, false).await?;
        if let Some(tkl) = result.get("tkl") {
            Ok(Some(tkl.clone()))
        } else {
            Ok(None)
        }
    }

    /// Delete a ban exception.
    pub async fn delete(&self, name: &str) -> Result<Option<serde_json::Value>> {
        let result = self.connection.query("server_ban_exception.del", serde_json::json!({"name": name}), false).await?;
        if let Some(tkl) = result.get("tkl") {
            Ok(Some(tkl.clone()))
        } else {
            Ok(None)
        }
    }

    /// Get a list of all exceptions.
    pub async fn get_all(&self) -> Result<serde_json::Value> {
        let result = self.connection.query("server_ban_exception.list", serde_json::Value::Null, false).await?;
        if let Some(list) = result.get("list") {
            Ok(list.clone())
        } else {
            Err(crate::error::Error::InvalidResponse)
        }
    }

    /// Get a specific ban exception.
    pub async fn get(&self, name: &str) -> Result<Option<serde_json::Value>> {
        let result = self.connection.query("server_ban_exception.get", serde_json::json!({"name": name}), false).await?;
        if let Some(tkl) = result.get("tkl") {
            Ok(Some(tkl.clone()))
        } else {
            Ok(None)
        }
    }
}