//! Server operations module.

use crate::connection::Connection;
use crate::error::Result;
use serde_json;

/// Server handler for server-related operations.
#[derive(Clone)]
pub struct Server {
    connection: Connection,
}

impl Server {
    /// Create a new server handler.
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Get a list of all servers.
    pub async fn get_all(&self) -> Result<serde_json::Value> {
        let result = self.connection.query("server.list", serde_json::Value::Null, false).await?;
        if let Some(list) = result.get("list") {
            Ok(list.clone())
        } else {
            Err(crate::error::Error::InvalidResponse)
        }
    }

    /// Get a server object.
    pub async fn get(&self, server: Option<&str>) -> Result<Option<serde_json::Value>> {
        let params = if let Some(srv) = server {
            serde_json::json!({"server": srv})
        } else {
            serde_json::Value::Null
        };

        let result = self.connection.query("server.get", params, false).await?;
        if let Some(srv) = result.get("server") {
            Ok(Some(srv.clone()))
        } else {
            Ok(None)
        }
    }
}