//! Statistics operations module.

use crate::connection::Connection;
use crate::error::Result;
use serde_json;

/// Stats handler for statistical information.
#[derive(Clone)]
pub struct Stats {
    connection: Connection,
}

impl Stats {
    /// Create a new stats handler.
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Get basic statistical information: user counts, channel counts, etc.
    pub async fn get(&self, object_detail_level: i32) -> Result<serde_json::Value> {
        self.connection.query("stats.get", serde_json::json!({
            "object_detail_level": object_detail_level
        }), false).await
    }
}