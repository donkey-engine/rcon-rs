use crate::errors::RCONError;
use crate::types::{ExecuteResponse, RCONRequest, RCONResponse};
use crate::{AuthRequest, AuthResponse};
use bytes::BufMut;
use std::io::{Read, Write};
use std::net::TcpStream;

/// Simple RCON client
#[derive(Debug)]
pub struct RCONClient {
    pub url: String,
    pub(self) socket: TcpStream,
}

/// RCON client
impl RCONClient {
    /// Create new connection
    pub fn new(url: String) -> Result<Self, RCONError> {
        let socket = TcpStream::connect(&url).map_err(|err| {
            RCONError::TcpConnectionError(format!("TCP connection error: {}", err))
        })?;
        Ok(Self { url, socket })
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
    let response_body_length = response_length - 10;
    let mut response_body_buffer = Vec::with_capacity(response_body_length as usize);
    socket
        .take(response_body_length as u64)
        .read_to_end(&mut response_body_buffer)
        .map_err(|err| RCONError::TcpConnectionError(format!("TCP response error {}", err)))?;
    let response_body = String::from_utf8(response_body_buffer)
        .map_err(|err| RCONError::TypeError(format!("TypeError {}", err)))?;

    Ok(ExecuteResponse {
        response_id,
        response_type,
        response_body,
    })
}
