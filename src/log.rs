//! Log operations module.

use crate::connection::Connection;
use crate::error::Result;
use serde_json;

/// Log handler for log operations.
#[derive(Clone)]
pub struct Log {
    connection: Connection,
}

impl Log {
    /// Create a new log handler.
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Subscribe to log events. Any previous subscriptions are overwritten.
    pub async fn subscribe(&self, sources: Vec<String>) -> Result<serde_json::Value> {
        self.connection.query("log.subscribe", serde_json::json!({
            "sources": sources
        }), false).await
    }

    /// Unsubscribe from all log events.
    pub async fn unsubscribe(&self) -> Result<serde_json::Value> {
        self.connection.query("log.unsubscribe", serde_json::Value::Null, false).await
    }

    /// Get past log events.
    pub async fn get_all(&self, sources: Option<Vec<String>>) -> Result<Option<serde_json::Value>> {
        let params = if let Some(srcs) = sources {
            serde_json::json!({"sources": srcs})
        } else {
            serde_json::Value::Null
        };

        let result = self.connection.query("log.list", params, false).await?;
        if let Some(list) = result.get("list") {
            Ok(Some(list.clone()))
        } else {
            Ok(None)
        }
    }
}