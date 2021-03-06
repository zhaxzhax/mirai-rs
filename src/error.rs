use reqwest::Error as ReqError;
use std::io::Error as IOError;
use std::fmt::Formatter;
use crate::Code;

pub use MiraiError::ServerError;
pub use MiraiError::HttpError;
pub use MiraiError::ImpossibleError;
pub use MiraiError::ClientError;
use tokio::io::Error;

pub type Result<T> = std::result::Result<T, MiraiError>;

#[derive(Debug)]
pub enum MiraiError {
    ServerError(Code, String),
    ImpossibleError(String),
    HttpError(ReqError),
    IOError(IOError),
    ClientError(String),
}

const SUCCESS: Code = 0;
const WRONG_AUTH_KEY: Code = 1;
const NO_SUCH_BOT: Code = 2;
const WRONG_SESSION: Code = 3;
const UNAUTHORIZED: Code = 4;
const NO_SUCH_TARGET: Code = 5;
const NO_SUCH_FILE: Code = 6;
const PERMISSION_DENIED: Code = 10;
const MUTED: Code = 20;
const MESSAGE_TOO_LONG: Code = 30;
const BAD_REQUEST: Code = 400;

pub(crate) fn assert(code: Code, action: &str) -> Result<()> {
    let msg = match code {
        SUCCESS => return Ok(()),
        WRONG_AUTH_KEY => "Wrong auth key",
        NO_SUCH_BOT => "No such bot",
        WRONG_SESSION => "Wrong session",
        UNAUTHORIZED => "Session wasn't authorized",
        NO_SUCH_TARGET => "No such target",
        NO_SUCH_FILE => "No such file",
        PERMISSION_DENIED => "Bot permission denied",
        MUTED => "Bot was muted",
        MESSAGE_TOO_LONG => "Message is too long",
        BAD_REQUEST => "Bad request",

        _ => "Unknown code"
    };

    Err(MiraiError::ServerError(code, format!("[{}] {}", action, msg)))
}

impl std::fmt::Display for MiraiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            MiraiError::ServerError(code, s) => f.write_str(&format!("{}: {}", code, s)),
            MiraiError::ImpossibleError(s) => f.write_str(&format!("{}", s)),
            MiraiError::HttpError(e) => f.write_str(&e.to_string()),
            MiraiError::IOError(e) => f.write_str(&e.to_string()),
            MiraiError::ClientError(e) => f.write_str(e),
        }
    }
}

impl From<ReqError> for MiraiError {
    fn from(e: ReqError) -> Self {
        MiraiError::HttpError(e)
    }
}

impl From<IOError> for MiraiError {
    fn from(e: Error) -> Self {
        MiraiError::IOError(e)
    }
}