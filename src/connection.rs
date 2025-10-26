//! Connection module for UnrealIRCd RPC.

use crate::error::{Error, Result};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

/// Options for connecting to the RPC server.
#[derive(Debug, Clone)]
pub struct Options {
    pub tls_verify: bool,
    pub issuer: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            tls_verify: true,
            issuer: None,
        }
    }
}

/// JSON-RPC request structure.
#[derive(Debug, Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: i64,
}

/// JSON-RPC response structure.
#[derive(Debug, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: Option<String>,
    result: Option<serde_json::Value>,
    error: Option<JsonRpcError>,
    id: Option<i64>,
}

/// JSON-RPC error structure.
#[derive(Debug, Deserialize)]
struct JsonRpcError {
    code: i64,
    message: String,
}

/// Main connection to the UnrealIRCd RPC server.
#[derive(Clone)]
pub struct Connection {
    uri: String,
    auth_header: String,
    options: Option<Options>,
    websocket: Arc<Mutex<Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>>>,
    next_id: Arc<AtomicI64>,
    errno: Arc<Mutex<i64>>,
    error: Arc<Mutex<Option<String>>>,
}

impl Connection {
    /// Create a new connection.
    pub fn new(uri: String, api_login: String, options: Option<Options>) -> Self {
        let auth_header = format!("Basic {}", base64::Engine::encode(&base64::engine::general_purpose::STANDARD, api_login.as_bytes()));

        Self {
            uri,
            auth_header,
            options,
            websocket: Arc::new(Mutex::new(None)),
            next_id: Arc::new(AtomicI64::new(1)),
            errno: Arc::new(Mutex::new(0)),
            error: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the URI (for testing purposes).
    #[cfg(test)]
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Get the auth header (for testing purposes).
    #[cfg(test)]
    pub fn auth_header(&self) -> &str {
        &self.auth_header
    }

    /// Establish the WebSocket connection.
    pub async fn connect(&mut self) -> Result<()> {
        let url = Url::parse(&self.uri)?;
        let request = http::Request::builder()
            .uri(url.as_str())
            .header("Authorization", &self.auth_header)
            .body(())?;

        let (ws_stream, _) = connect_async(request).await?;
        *self.websocket.lock().await = Some(ws_stream);

        // Set issuer if provided
        if let Some(issuer) = &self.options.as_ref().and_then(|o| o.issuer.as_ref()) {
            self.query("rpc.set_issuer", serde_json::json!({"name": issuer}), true).await?;
        }

        Ok(())
    }

    /// Send a JSON-RPC request and wait for response.
    pub async fn query(
        &self,
        method: &str,
        params: serde_json::Value,
        no_wait: bool,
    ) -> Result<serde_json::Value> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id,
        };

        let request_json = serde_json::to_string(&request)?;
        let message = Message::Text(request_json);

        let mut ws_guard = self.websocket.lock().await;
        let ws = ws_guard.as_mut().ok_or(Error::ConnectionClosed)?;

        ws.send(message).await?;

        if no_wait {
            return Ok(serde_json::Value::Bool(true));
        }

        // Wait for response with timeout
        let timeout_duration = std::time::Duration::from_secs(10);
        let response_msg = tokio::time::timeout(timeout_duration, ws.next())
            .await
            .map_err(|_| Error::Timeout)?
            .ok_or(Error::ConnectionClosed)??;

        let response_text = match response_msg {
            Message::Text(text) => text,
            Message::Close(_) => return Err(Error::ConnectionClosed),
            _ => return Err(Error::InvalidResponse),
        };

        let response: JsonRpcResponse = serde_json::from_str(&response_text)?;

        if let Some(error) = response.error {
            *self.errno.lock().await = error.code;
            *self.error.lock().await = Some(error.message.clone());
            return Err(Error::Rpc {
                code: error.code,
                message: error.message,
            });
        }

        if let Some(result) = response.result {
            *self.errno.lock().await = 0;
            *self.error.lock().await = None;
            Ok(result)
        } else {
            Err(Error::InvalidResponse)
        }
    }

    /// Get the last error code.
    pub async fn errno(&self) -> i64 {
        *self.errno.lock().await
    }

    /// Get the last error message.
    pub async fn error(&self) -> Option<String> {
        self.error.lock().await.clone()
    }

    /// Close the connection.
    pub async fn close(&mut self) -> Result<()> {
        if let Some(ws) = self.websocket.lock().await.as_mut() {
            ws.close(None).await?;
        }
        Ok(())
    }

    // Handler accessors
    pub fn rpc(&self) -> crate::rpc::Rpc {
        crate::rpc::Rpc::new(self.clone())
    }

    pub fn server(&self) -> crate::server::Server {
        crate::server::Server::new(self.clone())
    }

    pub fn user(&self) -> crate::user::User {
        crate::user::User::new(self.clone())
    }

    pub fn channel(&self) -> crate::channel::Channel {
        crate::channel::Channel::new(self.clone())
    }

    pub fn server_ban(&self) -> crate::server_ban::ServerBan {
        crate::server_ban::ServerBan::new(self.clone())
    }

    pub fn spamfilter(&self) -> crate::spamfilter::Spamfilter {
        crate::spamfilter::Spamfilter::new(self.clone())
    }

    pub fn name_ban(&self) -> crate::name_ban::NameBan {
        crate::name_ban::NameBan::new(self.clone())
    }

    pub fn log(&self) -> crate::log::Log {
        crate::log::Log::new(self.clone())
    }

    pub fn stats(&self) -> crate::stats::Stats {
        crate::stats::Stats::new(self.clone())
    }

    pub fn server_ban_exception(&self) -> crate::server_ban_exception::ServerBanException {
        crate::server_ban_exception::ServerBanException::new(self.clone())
    }
}