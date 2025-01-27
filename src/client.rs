use crate::errors::RCONError;
use crate::types::{ExecuteResponse, RCONRequest, RCONResponse};
use crate::{AuthRequest, AuthResponse};
use bytes::BufMut;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

const DEFAULT_READ_TIMEOUT: u64 = 30;
const DEFAULT_WRITE_TIMEOUT: u64 = 30;

/// Configuration for RCON client
#[derive(Default)]
pub struct RCONConfig {
    /// URL to server listening RCON
    /// example: `0.0.0.0:25575` or `donkey-engine.host:1337`
    pub url: String,
    /// Timeout in secs for commands sending to server
    /// Default: 30 secs
    pub write_timeout: Option<u64>,
    /// Timeout in secs for response waiting from server
    /// Default: 30 secs
    pub read_timeout: Option<u64>,
}

/// Simple RCON client
#[derive(Debug)]
pub struct RCONClient {
    pub url: String,
    pub(self) socket: TcpStream,
}

/// RCON client
impl RCONClient {
    /// Create new connection
    pub fn new(config: RCONConfig) -> Result<Self, RCONError> {
        let socket = TcpStream::connect(&config.url).map_err(|err| {
            RCONError::TcpConnectionError(format!("TCP connection error: {}", err))
        })?;

        socket
            .set_write_timeout(Some(Duration::new(
                config.write_timeout.unwrap_or(DEFAULT_WRITE_TIMEOUT),
                0,
            )))
            .map_err(|err| {
                RCONError::TcpConnectionError(format!("Cannot set socket write_timeout: {}", err))
            })?;
        socket
            .set_read_timeout(Some(Duration::new(
                config.read_timeout.unwrap_or(DEFAULT_READ_TIMEOUT),
                0,
            )))
            .map_err(|err| {
                RCONError::TcpConnectionError(format!("Cannot set socket read_timeout: {}", err))
            })?;

        Ok(Self {
            url: config.url,
            socket,
        })
    }

    /// Auth on game server
    pub fn auth(&mut self, auth: AuthRequest) -> Result<AuthResponse, RCONError> {
        let response = execute(
            &mut self.socket,
            auth.id as i32,
            auth.request_type as i32,
            auth.password,
        )?;
        Ok(AuthResponse {
            id: response.response_id as isize,
            response_type: response.response_type as u8,
        })
    }

    /// Execute request
    pub fn execute(&mut self, data: RCONRequest) -> Result<RCONResponse, RCONError> {
        let response = execute(
            &mut self.socket,
            data.id as i32,
            data.request_type as i32,
            data.body,
        )?;
        Ok(RCONResponse {
            id: response.response_id as isize,
            response_type: response.response_type as u8,
            body: response.response_body,
        })
    }
}

/// Make TCP request
fn execute(
    socket: &mut TcpStream,
    id: i32,
    request_type: i32,
    data: String,
) -> Result<ExecuteResponse, RCONError> {
    // Make request
    let request_length = (data.len() + 10) as i32;
    let mut request_buffer: Vec<u8> = Vec::with_capacity(request_length as usize);
    request_buffer.put_slice(&request_length.to_le_bytes());
    request_buffer.put_slice(&(id).to_le_bytes());
    request_buffer.put_slice(&(request_type).to_le_bytes());
    request_buffer.put_slice(data.as_bytes());
    request_buffer.put_slice(&[0x00, 0x00]);
    socket
        .write(&request_buffer[..])
        .map_err(|err| RCONError::TcpConnectionError(format!("TCP request error {}", err)))?;

    // Await response
    let mut response_buffer = [0u8; 4];
    socket
        .read_exact(&mut response_buffer)
        .map_err(|err| RCONError::TcpConnectionError(format!("TCP response error {}", err)))?;
    let response_length = i32::from_le_bytes(response_buffer);
    socket
        .read_exact(&mut response_buffer)
        .map_err(|err| RCONError::TcpConnectionError(format!("TCP response error {}", err)))?;
    let response_id = i32::from_le_bytes(response_buffer);
    socket
        .read_exact(&mut response_buffer)
        .map_err(|err| RCONError::TcpConnectionError(format!("TCP response error {}", err)))?;
    let response_type = i32::from_le_bytes(response_buffer);

    // Read response body with minimal changes
    let response_body_length = response_length - 10;
    let mut response_body_buffer = Vec::with_capacity(response_body_length as usize);
    let mut temp_buffer = vec![0; response_body_length as usize];
    let mut read_so_far = 0;

    while read_so_far < response_body_length as usize {
        match socket.read(&mut temp_buffer[read_so_far..]) {
            Ok(0) => break, // No more data
            Ok(n) => read_so_far += n,
            Err(e) => {
                eprintln!("Error reading response body: {}", e);
                break; // Preserve data read so far
            },
        }
    }

    // Append only the read data to the response body buffer
    response_body_buffer.extend_from_slice(&temp_buffer[..read_so_far]);

    let response_body = String::from_utf8(response_body_buffer)
        .map_err(|err| RCONError::TypeError(format!("TypeError: {}", err)))?;

    // Attempt to read terminating nulls without throwing an error on failure
    let mut terminating_nulls = [0u8; 2];
    match socket.read_exact(&mut terminating_nulls) {
        Ok(_) => {
            // Successfully read the terminating nulls, you can add additional logic here if needed
        },
        Err(e) => {
            // Log the error but do not throw it
            eprintln!("Non-fatal error reading terminating nulls: {}", e);
        },
    }

    Ok(ExecuteResponse {
        response_id,
        response_type,
        response_body,
    })
}
