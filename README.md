# rcon-rs

**Simple implementation of a crate that allows you to work with the RCON protocol**

To work with TCP, the `TcpStream` structure built into the `std::net` module is used

## About RCON
- [Description of RCON at developer.valvesoftware.com](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol)

## Dependencies
- [bytes](https://crates.io/crates/bytes) for converting types to bytes, for subsequent transmission via tcp
- [rand](https://crates.io/crates/rand) for generate a random request ID
- [serde](https://crates.io/crates/serde) for serializing errors
- [thiserror](https://crates.io/crates/thiserror) for serializing errors too

## Games that support this protocol
- Minecraft
- Counter Strike
- ARK
- Rust
- SAMP
- MTA
- etc

## Example

```rust
use rcon::{AuthRequest, RCONClient, RCONConfig, RCONError, RCONRequest};

fn main() -> Result<(), RCONError> {
    // Create new RCON client
    let mut client = RCONClient::new(RCONConfig {
        url: "donkey-engine.host".to_string(),
        // Optional
        read_timeout: Some(13),
        write_timeout: Some(37),
    })?;

    // Auth request to RCON server (SERVERDATA_AUTH)
    let auth_result = client.auth(AuthRequest::new("rcon.password".to_string()))?;
    assert!(auth_result.is_success());

    // Execute command request to RCON server (SERVERDATA_EXECCOMMAND)
    let version = client.execute(RCONRequest::new("seed".to_string()))?;
    assert_eq!(version.body, "Seed: [3257840388504953787]");

    Ok(())
}

```
