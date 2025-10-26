//! User operations module.

use crate::connection::Connection;
use crate::error::Result;
use serde_json;

/// User handler for user-related operations.
#[derive(Clone)]
pub struct User {
    connection: Connection,
}

impl User {
    /// Create a new user handler.
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Get a list of all users.
    pub async fn get_all(&self, object_detail_level: i32) -> Result<serde_json::Value> {
        let result = self.connection.query("user.list", serde_json::json!({
            "object_detail_level": object_detail_level
        }), false).await?;

        if let Some(list) = result.get("list") {
            Ok(list.clone())
        } else {
            Err(crate::error::Error::InvalidResponse)
        }
    }

    /// Get a user object.
    pub async fn get(&self, nick: &str, object_detail_level: i32) -> Result<Option<serde_json::Value>> {
        let result = self.connection.query("user.get", serde_json::json!({
            "nick": nick,
            "object_detail_level": object_detail_level
        }), false).await?;

        if let Some(client) = result.get("client") {
            Ok(Some(client.clone()))
        } else {
            Ok(None)
        }
    }

    /// Set the nickname of a user (changes the nick).
    pub async fn set_nick(&self, nick: &str, newnick: &str) -> Result<serde_json::Value> {
        self.connection.query("user.set_nick", serde_json::json!({
            "nick": nick,
            "newnick": newnick
        }), false).await
    }

    /// Set the username/ident of a user.
    pub async fn set_username(&self, nick: &str, username: &str) -> Result<serde_json::Value> {
        self.connection.query("user.set_username", serde_json::json!({
            "nick": nick,
            "username": username
        }), false).await
    }

    /// Set the realname/gecos of a user.
    pub async fn set_realname(&self, nick: &str, realname: &str) -> Result<serde_json::Value> {
        self.connection.query("user.set_realname", serde_json::json!({
            "nick": nick,
            "realname": realname
        }), false).await
    }

    /// Set a virtual host (vhost) on the user.
    pub async fn set_vhost(&self, nick: &str, vhost: &str) -> Result<serde_json::Value> {
        self.connection.query("user.set_vhost", serde_json::json!({
            "nick": nick,
            "vhost": vhost
        }), false).await
    }

    /// Change the user modes of a user.
    pub async fn set_mode(&self, nick: &str, mode: &str, hidden: bool) -> Result<serde_json::Value> {
        self.connection.query("user.set_mode", serde_json::json!({
            "nick": nick,
            "modes": mode,
            "hidden": hidden
        }), false).await
    }

    /// Change the snomask of a user (oper).
    pub async fn set_snomask(&self, nick: &str, snomask: &str, hidden: bool) -> Result<serde_json::Value> {
        self.connection.query("user.set_snomask", serde_json::json!({
            "nick": nick,
            "snomask": snomask,
            "hidden": hidden
        }), false).await
    }

    /// Make user an IRC Operator (oper).
    pub async fn set_oper(
        &self,
        nick: &str,
        oper_account: &str,
        oper_class: &str,
        class: Option<&str>,
        modes: Option<&str>,
        snomask: Option<&str>,
        vhost: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut params = serde_json::json!({
            "nick": nick,
            "oper_account": oper_account,
            "oper_class": oper_class
        });

        if let Some(c) = class {
            params["class"] = c.into();
        }
        if let Some(m) = modes {
            params["modes"] = m.into();
        }
        if let Some(s) = snomask {
            params["snomask"] = s.into();
        }
        if let Some(v) = vhost {
            params["vhost"] = v.into();
        }

        self.connection.query("user.set_oper", params, false).await
    }

    /// Join a user to a channel.
    pub async fn join(&self, nick: &str, channel: &str, key: Option<&str>, force: bool) -> Result<serde_json::Value> {
        let mut params = serde_json::json!({
            "nick": nick,
            "channel": channel,
            "force": force
        });

        if let Some(k) = key {
            params["key"] = k.into();
        }

        self.connection.query("user.join", params, false).await
    }

    /// Part a user from a channel.
    pub async fn part(&self, nick: &str, channel: &str, force: bool) -> Result<serde_json::Value> {
        self.connection.query("user.part", serde_json::json!({
            "nick": nick,
            "channel": channel,
            "force": force
        }), false).await
    }

    /// Quit a user from IRC. Pretends it is a normal QUIT.
    pub async fn quit(&self, nick: &str, reason: &str) -> Result<serde_json::Value> {
        self.connection.query("user.quit", serde_json::json!({
            "nick": nick,
            "reason": reason
        }), false).await
    }

    /// Kill a user from IRC. Shows that the user is forcefully removed.
    pub async fn kill(&self, nick: &str, reason: &str) -> Result<serde_json::Value> {
        self.connection.query("user.kill", serde_json::json!({
            "nick": nick,
            "reason": reason
        }), false).await
    }
}