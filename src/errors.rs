/// Errors enum for project
pub enum RCONError {
    /// Error with TCP connection
    TcpConnectionError(String),
    /// Error with types converting
    TypeError(String),
}
