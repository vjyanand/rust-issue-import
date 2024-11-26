use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ControllerError {
    pub message: String,
    pub code: String,
}

// Optional: Enum for more detailed error handling
#[derive(Debug, Error)]
pub enum SMSServiceError {
    #[error("Invalid phone number: {0}")]
    InvalidPhoneNumber(String),
    
    #[error("Twilio service error: {0}")]
    TwilioError(String),
    
    #[error("Internal service error: {0}")]
    InternalError(String),
}