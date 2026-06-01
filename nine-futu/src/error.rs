use std::fmt;

/// Error types for Futu API operations
#[derive(Debug, Clone)]
pub enum FutuError {
    /// Connection to FutuOpenD was lost
    ConnectionLost,
    /// Operation timed out
    Timeout,
    /// Not connected to FutuOpenD
    NotConnected,
    /// Invalid packet data
    PacketDataErr,
    /// Connection was closed
    ConnectionClosed,
    /// Invalid parameter
    ParamErr(String),
    /// Protocol error from FutuOpenD
    ProtoError { ret_type: i32, msg: String },
    /// IO error
    IoError(String),
    /// Protobuf encoding/decoding error
    ProtobufError(String),
}

impl fmt::Display for FutuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionLost => write!(f, "Connection lost"),
            Self::Timeout => write!(f, "Timeout"),
            Self::NotConnected => write!(f, "Not connected"),
            Self::PacketDataErr => write!(f, "Packet data error"),
            Self::ConnectionClosed => write!(f, "Connection closed"),
            Self::ParamErr(msg) => write!(f, "Parameter error: {}", msg),
            Self::ProtoError { ret_type, msg } => {
                write!(f, "Protocol error ({}): {}", ret_type, msg)
            }
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::ProtobufError(msg) => write!(f, "Protobuf error: {}", msg),
        }
    }
}

impl std::error::Error for FutuError {}

impl From<std::io::Error> for FutuError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e.to_string())
    }
}

impl From<prost::DecodeError> for FutuError {
    fn from(e: prost::DecodeError) -> Self {
        Self::ProtobufError(e.to_string())
    }
}

impl FutuError {
    /// Create error from FutuOpenD return type
    ///
    /// # Arguments
    /// * `ret_type` - Return type from FutuOpenD (0=success, negative=error)
    /// * `msg` - Error message
    pub fn from_ret_type(ret_type: i32, msg: String) -> Self {
        match ret_type {
            0 => unreachable!("ret_type 0 is success"),
            -1 => Self::ProtoError { ret_type, msg },
            -100 => Self::Timeout,
            -200 => Self::ConnectionLost,
            -400 => Self::ProtoError { ret_type, msg },
            -500 => Self::PacketDataErr,
            _ => Self::ProtoError { ret_type, msg },
        }
    }
}
