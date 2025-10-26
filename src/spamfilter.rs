//! Spamfilter operations module.

use crate::connection::Connection;
use crate::error::Result;
use serde_json;

/// Spamfilter handler for spamfilter operations.
#[derive(Clone)]
pub struct Spamfilter {
    connection: Connection,
}

impl Spamfilter {
    /// Create a new spamfilter handler.
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Add a spamfilter.
    pub async fn add(
        &self,
        name: &str,
        match_type: &str,
        spamfilter_targets: &str,
        ban_action: &str,
        ban_duration: &str,
        reason: &str,
    ) -> Result<Option<serde_json::Value>> {
        let result = self.connection.query("spamfilter.add", serde_json::json!({
            "name": name,
            "match_type": match_type,
            "spamfilter_targets": spamfilter_targets,
            "ban_action": ban_action,
            "ban_duration": ban_duration,
            "reason": reason
        }), false).await?;

        if let Some(tkl) = result.get("tkl") {
            Ok(Some(tkl.clone()))
        } else {
            Ok(None)
        }
    }

    /// Delete a spamfilter.
    pub async fn delete(
        &self,
        name: &str,
        match_type: &str,
        spamfilter_targets: &str,
        ban_action: &str,
    ) -> Result<Option<serde_json::Value>> {
        let result = self.connection.query("spamfilter.del", serde_json::json!({
            "name": name,
            "match_type": match_type,
            "spamfilter_targets": spamfilter_targets,
            "ban_action": ban_action
        }), false).await?;

        if let Some(tkl) = result.get("tkl") {
            Ok(Some(tkl.clone()))
        } else {
            Ok(None)
        }
    }

    /// Get a list of all spamfilters.
    pub async fn get_all(&self) -> Result<serde_json::Value> {
        let result = self.connection.query("spamfilter.list", serde_json::Value::Null, false).await?;
        if let Some(list) = result.get("list") {
            Ok(list.clone())
        } else {
            Err(crate::error::Error::InvalidResponse)
        }
    }

    /// Get a specific spamfilter.
    pub async fn get(
        &self,
        name: &str,
        match_type: &str,
        spamfilter_targets: &str,
        ban_action: &str,
    ) -> Result<Option<serde_json::Value>> {
        let result = self.connection.query("spamfilter.get", serde_json::json!({
            "name": name,
            "match_type": match_type,
            "spamfilter_targets": spamfilter_targets,
            "ban_action": ban_action
        }), false).await?;

        if let Some(tkl) = result.get("tkl") {
            Ok(Some(tkl.clone()))
        } else {
            Ok(None)
        }
    }
}