use super::errors::{ErrorResponse, JabraError};
use leptos::*;

pub enum HttpMethod {
    POST,
    GET,
    PATCH,
}
/// A utility function to send HTTP requests and parse the response
/// It uses the reqwest crate to send the request
///
/// # Arguments
///
/// * `request` - The request body, this should be leptos::Serializable
/// * `url` - The URL to send the request to
/// * `headers` - The headers to be sent with the request
/// * `method` - The HTTP method to be used
///
/// # Returns
///
/// The parsed response or ServerFnError
pub async fn call_and_parse<Request, Response>(
    request: Option<Request>,
    url: String,
    headers: reqwest::header::HeaderMap,
    method: HttpMethod,
) -> Result<Response, JabraError>
where
    Request: serde::Serialize,
    Response: Serializable,
{
    let client = reqwest::Client::new();
    let response = match method {
        HttpMethod::GET => {
            let path = match request {
                Some(req) => {
                    let query_string = serde_urlencoded::to_string(req);
                    match query_string {
                        Ok(query_string) => format!("{}?{}", url, query_string),
                        _ => url,
                    }
                }
                None => url,
            };
            client.get(&path).headers(headers).send().await
        }
        HttpMethod::POST => {
            client
                .post(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        }
        HttpMethod::PATCH => {
            client
                .patch(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        }
    };
    // log::info!("Response: {:?}", response);
    match response {
        Ok(res) => {
            if res.status() == reqwest::StatusCode::OK {
                let response = res.text().await.map_err(|e| JabraError::from(e))?;
                Response::de(&response).map_err(|e| JabraError::from(e))
            } else {
                // Err(JabraError::APIResponseError(res.status().to_string()))
                let res_text = res.text().await.map_err(|e| JabraError::from(e))?;
                if res_text.len() == 0 {
                    return Err(JabraError::APIResponseError(String::from("System is busy")));
                }
                let res_json: ErrorResponse = ErrorResponse::de(res_text.as_str()).map_err(|e| JabraError::from(e))?;
                log::info!("RES_JSON: {:?}", res_json);
                Err(JabraError::APIResponseError(res_json.errors[0].message.clone().to_string()))
            }
        }
        Err(e) => Err(JabraError::from(e)),
    }
}

/// A utility function to send HTTP requests and parse the HTTP status of the response
/// It uses the reqwest crate to send the request
///
/// # Arguments
///
/// * `request` - The request body, this should be leptos::Serializable
/// * `url` - The URL to send the request to
/// * `headers` - The headers to be sent with the request
/// * `method` - The HTTP method to be used
///
/// # Returns
///
/// The Result with the boolean value of the success of the request or a ServerError

pub async fn call<Request>(
    request: Option<Request>,
    url: String,
    headers: reqwest::header::HeaderMap,
    method: HttpMethod,
) -> Result<bool, JabraError>
where
    Request: serde::Serialize,
{
    let client = reqwest::Client::new();
    let response = match method {
        HttpMethod::GET => {
            let path = match request {
                Some(req) => {
                    let query_string = serde_urlencoded::to_string(req);
                    match query_string {
                        Ok(query_string) => format!("{}?{}", url, query_string),
                        _ => url,
                    }
                }
                None => url,
            };
            client.get(&path).headers(headers).send().await
        }
        HttpMethod::POST => {
            client
                .post(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        }
        HttpMethod::PATCH => {
            client
                .patch(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        }
    };
    match response {
        Ok(res) => {
            if res.status().is_success() {
                Ok(true)
            } else {
                let res_text = res.text().await.map_err(|e| JabraError::from(e))?;
                if res_text.len() == 0 {
                    return Err(JabraError::APIResponseError(String::from("System is busy")));
                }
                let res_json: ErrorResponse = ErrorResponse::de(res_text.as_str()).map_err(|e| JabraError::from(e))?;
                log::info!("RES_JSON: {:?}", res_json);
                Err(JabraError::APIResponseError(res_json.errors[0].message.clone().to_string()))
            }
        }
        Err(e) => Err(JabraError::from(e)),
    }
}
