use consts::ErrorCode;
use std::error::Error;
use std::io;
use std::io::ErrorKind;


pub type ZmqResult<T> = Result<T, ZmqError>;

#[derive(Debug, Clone)]
pub struct ZmqError {
	pub code: ErrorCode,
    desc: String,
}

impl ZmqError {
    pub fn new(code: ErrorCode, desc: &'static str) -> ZmqError {
        ZmqError {
            code: code,
            desc: desc.to_string(),
        }
    }

    pub fn from_io_error(e: io::Error) -> ZmqError {
//        println!("{:?}", e.kind());
        ZmqError {
            code: match e.kind() {
                ErrorKind::PermissionDenied => ErrorCode::EACCES,
                ErrorKind::ConnectionRefused => ErrorCode::ECONNREFUSED,
                ErrorKind::ConnectionReset => ErrorCode::ECONNRESET,
                ErrorKind::ConnectionAborted => ErrorCode::ECONNABORTED,
                ErrorKind::NotConnected => ErrorCode::ENOTCONN,
                ErrorKind::TimedOut => ErrorCode::ETIMEDOUT,

				ErrorKind::AddrNotAvailable => ErrorCode::ECONNREFUSED,
				ErrorKind::AddrInUse => ErrorCode::ECONNREFUSED,

                _ => ErrorCode::EIOERROR,
            },
            desc: e.description().to_string(),
        }
    }
}
