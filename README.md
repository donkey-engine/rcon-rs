# rcon-rs

**Simple implementation of a crate that allows you to work with the RCON protocol**

To work with TCP, the `TcpStream` structure built into the `std::net` module is used

## About RCON
- [Description of RCON at developer.valvesoftware.com](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol)

## Dependencies
- [rand](https://crates.io/crates/rand) for generate a random request ID
- [bytes](https://crates.io/crates/bytes) for converting types to bytes, for subsequent transmission via tcp

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
use rcon::{AuthRequest, RCONClient, RCONError, RCONRequest};

fn main() -> Result<(), RCONError> {
    let server_url = String::from("donkey-engine.host");
    let mut client = rcon::RCONClient::new(server_url)?;
    let auth_result = client.auth(AuthRequest::new(String::from("RCON_SECRET")))?;
    assert!(auth_result.is_success());
    let version = client.execute(RCONRequest {
        id: 228,
        request_type: 2,
        body: String::from("VERSION"),
    })?;
    assert_eq!(version.body, "1.0.0");
    Ok(())
}

```
