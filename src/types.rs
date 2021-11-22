use rand::Rng;

/// Common type for TCP response
pub(crate) struct ExecuteResponse {
    pub(crate) response_id: i32,
    pub(crate) response_type: i32,
    pub(crate) response_body: String,
}

/// Request for auth in RCON
pub struct AuthRequest {
    pub id: usize,
    pub request_type: u8,
    pub password: String,
}

impl AuthRequest {
    /// Create new auth request data
    pub fn new(password: String) -> Self {
        Self {
            id: rand::thread_rng().gen::<usize>(),
            request_type: 3,
            password,
        }
    }
}

/// Response from auth request
pub struct AuthResponse {
    pub id: isize,
    pub response_type: u8,
}

impl AuthResponse {
    /// Is auth success
    pub fn is_success(&self) -> bool {
        if self.id == -1 {
            return false;
        }
        true
    }
}

/// Request for RCON command
pub struct RCONRequest {
    pub id: usize,
    pub request_type: u8,
    pub body: String,
}

/// Response for RCON command
pub struct RCONResponse {
    pub id: isize,
    pub response_type: u8,
    pub body: String,
}
