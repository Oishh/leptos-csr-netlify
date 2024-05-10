use leptos::*;
use leptos_use::{use_cookie, use_cookie_with_options, utils::FromToStringCodec, UseCookieOptions};
use serde::{Deserialize, Serialize};

use crate::models::login::DirectusLoginResponse;

use super::errors::JabraError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct JabraCookie {
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

impl JabraCookie {
    pub fn new(
        user_id: String,
        access_token: String,
        refresh_token: String,
        expires_in: i64,
    ) -> Self {
        Self {
            user_id,
            access_token,
            refresh_token,
            expires_in,
        }
    }

    pub fn encrypt(&self) -> String {
        let cookie_string = serde_json::to_string(self).unwrap();
        super::encryption::enc(cookie_string)
    }

    pub fn decrypt(encrypted_text: String) -> Result<Self, JabraError> {
        let decrypted_text = super::encryption::dec(encrypted_text);
        match serde_json::from_str(&decrypted_text) {
            Ok(cookie) => Ok(cookie),
            Err(_e) => Err(JabraError::CookieFetchError),
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp_millis();
        now > self.expires_in
    }

    pub fn from_string(cookie_string: String) -> Result<Self, JabraError> {
        match serde_json::from_str(&cookie_string) {
            Ok(cookie) => Ok(cookie),
            Err(_e) => Err(JabraError::CookieFetchError),
        }
    }
}

pub fn set_jabra_cookie(jabra_cookie: JabraCookie, cookie_name: String) {
    let (_cookie, set_cookie) = use_cookie_with_options::<String, FromToStringCodec>(&cookie_name, UseCookieOptions::default().path("/"));
    let encrypted_cookie = jabra_cookie.encrypt();

    log::info!("SET JABRA COOKIE!");
    set_cookie(Some(encrypted_cookie));
}

pub fn get_jabra_cookie(cookie_name: String) -> String {
    let (cookie, _set_cookie) =
        leptos_use::use_cookie::<String, leptos_use::utils::FromToStringCodec>(&cookie_name);
    let val = cookie.get_untracked().unwrap_or_default();
    val
}

pub fn logout_expired_token(cookie_name: String) {
    let navigate = leptos_router::use_navigate();
    let (_cookie, set_cookie) = use_cookie_with_options::<String, FromToStringCodec>(&cookie_name, UseCookieOptions::default().path("/"));
    set_cookie(Some("".to_string()));
    use_context::<crate::Refetcher>().unwrap().0.update(|s| *s = !*s);
    navigate("/login", Default::default());
}

pub async fn get_bearer_token(cookie_name: String) -> String {
    let cookie = get_jabra_cookie(cookie_name);
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    bearer
}
pub async fn check_server_cookie(cookie_name: String) -> Result<bool, ServerFnError> {
    let (cookie, _set_cookie) = use_cookie::<String, FromToStringCodec>(cookie_name.as_str());
    match cookie.get_untracked() {
        Some(val) => {
            if val.len() > 0 {
                match JabraCookie::decrypt(val) {
                    Ok(e) => Ok(!e.is_expired()),
                    Err(_) => Ok(false),
                }
                // Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}
pub async fn refresh_token(
    owner: String,
    refresh_token: String,
) -> Result<JabraCookie, JabraError> {
    // let encryption_key = if let Ok(var) = std::env::var("JABRAKEY") {
    //     var
    // } else {
    //     "".to_string()
    // };
    let url = option_env!("DIRECTUSURL");

    let path = format!("{}/auth/refresh", url.unwrap_or_default());
    let json_body = serde_json::json!({
        "refresh_token": refresh_token,
        "mode": "json"
    });
    let client = reqwest::Client::new();
    let response = client
        .post(&path)
        .json(&json_body)
        .send()
        .await
        .map_err(|e| JabraError::from(e))?;

    if response.status().is_success() {
        let response_body = response.text().await.map_err(|e| JabraError::from(e))?;
        let directus_login_response =
            DirectusLoginResponse::de(&response_body).map_err(|e| JabraError::from(e));
        match directus_login_response {
            Ok(res) => {
                let expiration_time =
                    chrono::Utc::now().timestamp_millis() + res.data.expires - 60_000;
                let jabra_cookie = JabraCookie::new(
                    owner,
                    res.data.access_token,
                    res.data.refresh_token,
                    expiration_time,
                );
                Ok(jabra_cookie)
            }
            Err(e) => Err(JabraError::from(e)),
        }
    } else {
        Err(JabraError::APIResponseError(response.status().to_string()))
    }
}
