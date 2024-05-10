use leptos::*;
use serde::{Deserialize, Serialize};

/// Enum representing the different errors that can occur when interacting with the Jabra API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JabraError {
    /// Error when fetching the cookie from the Jabra API.
    #[serde(rename = "CookieFetchError")]
    CookieFetchError,
    /// Error when the username or password does not match.
    #[serde(rename = "LoginError")]
    LoginError,
    /// Error when no data is found.
    #[serde(rename = "NoDataFoundError")]
    NoDataFoundError,
    /// Error when serializing or deserializing data.
    #[serde(rename = "SerializationError")]
    SerializationError(String),
    /// Error when making a request to the Jabra API.
    #[serde(rename = "ReqwestError")]
    ReqwestError(String),
    /// Error when the API response is not successful.
    #[serde(rename = "APIResponseError")]
    APIResponseError(String),
}
impl ToString for JabraError {
    /// Convert the JabraError to a string.
    ///
    /// # Returns
    ///
    /// The string representation of the JabraError.
    fn to_string(&self) -> String {
        match self {
            JabraError::CookieFetchError => "Cookie not found".to_string(),
            JabraError::LoginError => "Username or Password does not matched".to_string(),
            JabraError::NoDataFoundError => "Data does not load correctly".to_string(),
            JabraError::SerializationError(e) => e.to_string(),
            JabraError::ReqwestError(e) => e.to_string(),
            JabraError::APIResponseError(message) => message.to_string(),
        }
    }
}

// Create From implementation for reqwest::Error, since it does not allow Clone
impl From<reqwest::Error> for JabraError {
    fn from(error: reqwest::Error) -> Self {
        JabraError::ReqwestError(error.to_string())
    }
}

impl From<SerializationError> for JabraError {
    fn from(error: SerializationError) -> Self {
        JabraError::SerializationError(error.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorResponse {
    pub errors: Vec<Error>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Error {
    pub message: String,
    pub extensions: Extension,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Extension {
    pub code: String,
}
