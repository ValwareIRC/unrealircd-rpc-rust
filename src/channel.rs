//! Channel operations module.

use crate::connection::Connection;
use crate::error::Result;
use serde_json;

/// Channel handler for channel-related operations.
#[derive(Clone)]
pub struct Channel {
    connection: Connection,
}

impl Channel {
    /// Create a new channel handler.
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Get a list of channels users.
    pub async fn get_all(&self, object_detail_level: i32) -> Result<serde_json::Value> {
        let result = self.connection.query("channel.list", serde_json::json!({
            "object_detail_level": object_detail_level
        }), false).await?;

        if let Some(list) = result.get("list") {
            Ok(list.clone())
        } else {
            Err(crate::error::Error::InvalidResponse)
        }
    }

    /// Get a channel object.
    pub async fn get(&self, channel: &str, object_detail_level: i32) -> Result<Option<serde_json::Value>> {
        let result = self.connection.query("channel.get", serde_json::json!({
            "channel": channel,
            "object_detail_level": object_detail_level
        }), false).await?;

        if let Some(ch) = result.get("channel") {
            Ok(Some(ch.clone()))
        } else {
            Ok(None)
        }
    }

    /// Set and unset modes on a channel.
    pub async fn set_mode(&self, channel: &str, modes: &str, parameters: &str) -> Result<serde_json::Value> {
        self.connection.query("channel.set_mode", serde_json::json!({
            "channel": channel,
            "modes": modes,
            "parameters": parameters
        }), false).await
    }

    /// Set the channel topic.
    pub async fn set_topic(
        &self,
        channel: &str,
        topic: &str,
        set_by: Option<&str>,
        set_at: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut params = serde_json::json!({
            "channel": channel,
            "topic": topic
        });

        if let Some(sb) = set_by {
            params["set_by"] = sb.into();
        }
        if let Some(sa) = set_at {
            params["set_at"] = sa.into();
        }

        self.connection.query("channel.set_topic", params, false).await
    }

    /// Kick a user from the channel.
    pub async fn kick(&self, channel: &str, nick: &str, reason: &str) -> Result<serde_json::Value> {
        self.connection.query("channel.kick", serde_json::json!({
            "nick": nick,
            "channel": channel,
            "reason": reason
        }), false).await
    }
}