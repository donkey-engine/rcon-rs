# rcon-rs

Simple [RCON](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol) protocol realization on RUST programming language

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
