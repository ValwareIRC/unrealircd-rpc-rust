# UnrealIRCd RPC Rust Client

This Rust library provides a client for controlling UnrealIRCd via its JSON-RPC interface.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
unrealircd-rpc = "0.1"
```

## UnrealIRCd Setup

UnrealIRCd 6.0.6 or later is needed and you need to enable [JSON-RPC](https://www.unrealircd.org/docs/JSON-RPC) in it. After doing that, be sure to rehash the IRCd.

## Usage

```rust
use unrealircd_rpc::{Connection, Options};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::new(
        "wss://127.0.0.1:8600/".to_string(),
        "username:password".to_string(),
        Some(Options {
            tls_verify: false,
            issuer: None,
        }),
    );

    conn.connect().await?;

    // Get server bans
    let bans = conn.server_ban().get_all().await?;
    println!("Bans: {:?}", bans);

    // Get users
    let users = conn.user().get_all(2).await?;
    println!("Users: {:?}", users);

    // Get channels
    let channels = conn.channel().get_all(1).await?;
    println!("Channels: {:?}", channels);

    conn.close().await?;
    Ok(())
}
```

## Environment Variables

The library supports configuration via environment variables:

- `UNREALIRCD_API_USERNAME`: API username (required)
- `UNREALIRCD_API_PASSWORD`: API password (required)
- `UNREALIRCD_WS_URL`: WebSocket URL for the UnrealIRCd RPC server (optional, defaults to `wss://127.0.0.1:8600/`)

## Custom Queries

All convenience methods internally use the `Connection::query()` method, which is the main wrapper for sending JSON-RPC requests to UnrealIRCd.

```rust
// Example: Get server uptime (custom query)
let uptime = conn.query("server.get", serde_json::json!({"server": "irc.example.com"}), false).await?;

// Example: Send a raw JSON-RPC request
let result = conn.query("stats.get", serde_json::json!({"object_detail_level": 2}), false).await?;

// Example: Asynchronous query (no wait for response)
conn.query("log.subscribe", serde_json::json!({"sources": ["opers", "errors"]}), true).await?;
```

## Modules

- **Connection**: Core WebSocket connection and JSON-RPC communication
- **RPC**: Meta operations (info, timers, issuer)
- **Server**: Server management
- **User**: User operations (nick changes, modes, joins/parts, etc.)
- **Channel**: Channel management (modes, topics, kicks)
- **ServerBan**: Server ban management
- **Spamfilter**: Spam filter configuration
- **NameBan**: Name ban (QLine) management
- **Log**: Log streaming and retrieval
- **Stats**: Server statistics
- **ServerBanException**: Ban exception handling

## Error Handling

The library uses the `Result<T>` type for error handling. All errors are wrapped in the `Error` enum which provides detailed information about what went wrong.

## License

MIT