use serde::{Deserialize, Serialize};

/// Struct for the Directus Login Request.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectusLoginRequest {
    pub email: String,
    pub password: String,
}

impl DirectusLoginRequest {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

/// Struct for the Directus Login Response.
/// Has the [`DirectusLoginResponseData`] field.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectusLoginResponse {
    pub data: DirectusLoginResponseData,
}

/// Struct for the Directus Login Response Data.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectusLoginResponseData {
    pub access_token: String,
    pub expires: i64,
    pub refresh_token: String,
}
