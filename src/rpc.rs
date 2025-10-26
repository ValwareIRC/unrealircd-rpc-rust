//! RPC meta operations module.

use crate::connection::Connection;
use crate::error::Result;
use serde_json;

/// RPC handler for meta operations.
#[derive(Clone)]
pub struct Rpc {
    connection: Connection,
}

impl Rpc {
    /// Create a new RPC handler.
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Get information on all RPC modules loaded.
    pub async fn info(&self) -> Result<serde_json::Value> {
        self.connection.query("rpc.info", serde_json::Value::Null, false).await
    }

    /// Set the name of the issuer (requires UnrealIRCd 6.0.8+).
    pub async fn set_issuer(&self, name: &str) -> Result<serde_json::Value> {
        self.connection.query("rpc.set_issuer", serde_json::json!({"name": name}), false).await
    }

    /// Add a timer (requires UnrealIRCd 6.1.0+).
    pub async fn add_timer(
        &self,
        timer_id: &str,
        every_msec: i64,
        method: &str,
        params: serde_json::Value,
        id: Option<i64>,
    ) -> Result<serde_json::Value> {
        let request_id = id.unwrap_or_else(|| 100000 + rand::random::<i64>() % 900000);

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": request_id
        });

        self.connection.query("rpc.add_timer", serde_json::json!({
            "timer_id": timer_id,
            "every_msec": every_msec,
            "request": request
        }), false).await
    }

    /// Delete a timer (requires UnrealIRCd 6.1.0+).
    pub async fn del_timer(&self, timer_id: &str) -> Result<serde_json::Value> {
        self.connection.query("rpc.del_timer", serde_json::json!({"timer_id": timer_id}), false).await
    }
}