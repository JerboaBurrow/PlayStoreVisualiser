use std::str::from_utf8;

use axum::{
    body::Bytes, http::{HeaderMap, Request}, middleware::Next, response::{Html, IntoResponse, Response}
};
use openssl::conf;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{generate::generate_mockup, model::UserAppEntry};

use super::{config::read_config, is_authentic};

/// A trait representing an API request to the server
///  - For example [crate::server::api::stats::Generate]
pub trait ApiRequest
{
    /// Validate a request's hmac given a token read from config.json 
    ///   - See [crate::config::Config] and [crate::web::is_authentic]
    fn is_authentic(headers: HeaderMap, body: Bytes) -> StatusCode;
    /// Deserialise the Bytes body from JSON
    fn deserialise_payload(&mut self, headers: HeaderMap, body: Bytes) -> StatusCode;
    /// Formulate a response form the server returned as a String
    ///   - Also perform any actions inherent to this Api call
    async fn into_response(&self) -> (Option<String>, StatusCode);
    /// Axum middleware to 
    ///     1. check headers for an api request type
    ///     2. authenticate the request (HMAC)
    ///     3. respond to it
    ///     4. continue on to the next reqeust
    async fn filter<B>
    (
        headers: HeaderMap,
        request: Request<B>,
        next: Next<B>
    ) -> Result<Response, StatusCode>
    where B: axum::body::HttpBody<Data = Bytes>;

}

/// Payload for [Generate] Api request
///  - ```from_utc```: takes a utc date to compile statistics from
///  - ```to_utc```: takes a utc date to compile statistics to
///  - ```post_discord```: whether to post to dicsord or not
#[derive(Deserialize, Debug)]
pub struct GeneratePayload
{
    pub app: UserAppEntry,
    pub query: Option<String>,
    pub position: Option<usize>
}

/// Payload for [Generate] Api request, see [GeneratePayload]
///  - Takes a utc date to compile statistics from, and a switch to post a discord message
///  - All saved hit statistics after from_utc will be included
pub struct Generate 
{
    payload: GeneratePayload
}

impl Generate
{
    pub fn new() -> Generate
    {
        Generate 
        { 
            payload: GeneratePayload 
            {
                app: UserAppEntry::new("FEATURE", "ICON", "TITLE", "DEVELOPER", "STARS", "LINK"),
                query: Some("particles".to_string()),
                position: Some(0)
            } 
        }
    }
}

impl ApiRequest for Generate
{
    fn is_authentic(headers: HeaderMap, body: Bytes) -> StatusCode
    {

        let config = match read_config()
        {
            Some(c) => c,
            None =>
            {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        };

        match config.api_token
        {
            Some(t) =>
            {
                is_authentic
                (
                    headers, 
                    "psv-token", 
                    t, 
                    body
                )
            },
            None => StatusCode::ACCEPTED
        }
    }

    fn deserialise_payload(&mut self, _headers: HeaderMap, body: Bytes) -> StatusCode
    {
        
        self.payload = match from_utf8(&body)
        {
            Ok(s) => 
            {
                match serde_json::from_str(s)
                {
                    Ok(p) => p,
                    Err(e) =>
                    {
                        crate::debug(format!("{} deserialising POST payload",e), Some("Stats Digest".to_string()));
                        return StatusCode::BAD_REQUEST
                    }
                }
            }
            Err(e) => 
            {
                crate::debug(format!("{} deserialising POST payload",e), Some("Stats Digest".to_string()));
                return StatusCode::BAD_REQUEST
            }
        };

        StatusCode::OK
    }

    async fn into_response(&self) -> (Option<String>, StatusCode)
    {
        let config = match read_config()
        {
            Some(c) => c,
            None =>
            {
                return (None, StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        let query = match self.payload.query.clone()
        {
            Some(q) => q,
            None => "particles".to_string()
        };

        // get a live example from the Play Store
        let html = reqwest::get(format!("https://play.google.com/store/search?q={}&c=apps",query))
        .await.unwrap()
        .text()
        .await.unwrap();

        let position = match self.payload.position
        {
            Some(p) => p,
            None => 0
        };

        crate::debug(format!("Generating from\n {:?}\n{}", self.payload.app.clone(), position), None);

        let generated = match generate_mockup
        (
            html, 
            self.payload.app.clone(),
            Some(position)
        )
        {
            Ok(g) => g,
            Err(e) => {println!("{}", e); std::process::exit(1);}
        };

        (Some(generated), StatusCode::OK)
    }

    async fn filter<B>
    (
        headers: HeaderMap,
        request: Request<B>,
        next: Next<B>
    ) -> Result<Response, StatusCode>
    where B: axum::body::HttpBody<Data = Bytes>
    {

        if !headers.contains_key("api")
        {
            return Ok(next.run(request).await)
        }

        let config = match read_config()
        {
            Some(c) => c,
            None =>
            {
                return Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        };

        let api = match std::str::from_utf8(headers["api"].as_bytes())
        {
            Ok(u) => u,
            Err(_) =>
            {
                crate::debug("no/mangled user agent".to_string(), None);
                return Ok(next.run(request).await)
            }
        };

        match api == "Generate"
        {
            true => {},
            false => { return Ok(next.run(request).await) }
        }

        let body = request.into_body();
        let bytes = match body.collect().await {
            Ok(collected) => collected.to_bytes(),
            Err(_) => {
                return Err(StatusCode::BAD_REQUEST)
            }
        };

        match Generate::is_authentic(headers.clone(), bytes.clone())
        {
            StatusCode::ACCEPTED => {},
            e => { return Ok(e.into_response()) }
        }

        let mut response = Generate::new();

        match response.deserialise_payload(headers, bytes)
        {
            StatusCode::OK => {},
            e => { return Ok(e.into_response()) }
        }

        let (result, status) = response.into_response().await;

        match result
        {
            Some(s) => 
            {
                let mut response = Html(s).into_response();
                let time_stamp = chrono::offset::Utc::now().to_rfc3339();
                response.headers_mut().insert("date", time_stamp.parse().unwrap());
                response.headers_mut().insert("cache-control", format!("public, max-age={}", config.cache_period_seconds).parse().unwrap());
                
                match config.cors_allow_address
                {
                    Some(a) => 
                    {
                        response.headers_mut().insert("Access-Control-Allow-Origin", format!("{}",a).parse().unwrap());
                        response.headers_mut().insert("Access-Control-Allow-Methods", "POST".parse().unwrap());
                    },
                    None => {}
                }
                Ok(response)
            },
            None => { Err(status) }
        }
    }

}
