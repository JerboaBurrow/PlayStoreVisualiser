use axum::{body::Bytes, http::{HeaderMap, Request}, middleware::Next, response::{IntoResponse, Response}};
use openssl::{hash::MessageDigest, memcmp, pkey::PKey, sign::Signer};
use regex::Regex;
use reqwest::StatusCode;

use crate::util::{dump_bytes, read_bytes};

use self::config::read_config;

pub mod http;
pub mod https;
pub mod config;
pub mod throttle;
pub mod api;

async fn filter_cors_preflight<B>
(
    headers: HeaderMap,
    request: Request<B>,
    next: Next<B>
) -> Result<Response, StatusCode>
where B: axum::body::HttpBody<Data = Bytes>
{

    match headers.contains_key("Access-Control-Request-Headers")
    {
        false => return Ok(next.run(request).await),
        true => {}
    }

    let config = match read_config()
    {
        Some(c) => c,
        None =>
        {
            return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };

    match config.cors_allow_address
    {
        Some(a) =>
        {
            let mut response = String::new().into_response();
            response.headers_mut().insert("Access-Control-Allow-Origin", format!("{}",a).parse().unwrap());
            response.headers_mut().insert("Access-Control-Allow-Methods", "POST".parse().unwrap());
            response.headers_mut().insert("Access-Control-Allow-Headers", "api, content-type".parse().unwrap());
        
            Ok(response)
        },
        None =>
        {
            Ok(StatusCode::FORBIDDEN.into_response())
        }
    }
    

    
}
/// Uses openssl to verify the request body via the given hmac_token
///   - hmac_header_key is the location in the https header for the digest
pub fn is_authentic
(
    headers: HeaderMap,
    hmac_header_key: &str,
    hmac_token: String, 
    body: Bytes
) -> StatusCode
{
    match headers.contains_key(hmac_header_key)
    {
        false => 
        {
            crate::debug("no signature".to_string(), None);
            return StatusCode::UNAUTHORIZED
        },
        true => {}
    };

    let sender_hmac = match std::str::from_utf8(headers[hmac_header_key].as_bytes())
    {
        Ok(s) => s,
        Err(_) => 
        {
            crate::debug("signature utf8 parse failure".to_string(), None);
            return StatusCode::BAD_REQUEST
        }
    };

    let post_digest = Regex::new(r"sha256=").unwrap().replace_all(&sender_hmac, "").into_owned().to_uppercase();

    let key = match PKey::hmac(hmac_token.as_bytes())
    {
        Ok(k) => k,
        Err(_) => 
        {
            crate::debug("key creation failure".to_string(), None);
            return StatusCode::INTERNAL_SERVER_ERROR
        }
    };

    let mut signer = match Signer::new(MessageDigest::sha256(), &key)
    {
        Ok(k) => k,
        Err(_) => 
        {
            crate::debug("signer creation failure".to_string(), None);
            return StatusCode::INTERNAL_SERVER_ERROR
        }
    };
    
    match signer.update(&body)
    {
        Ok(k) => k,
        Err(_) => 
        {
            crate::debug("signing update failure".to_string(), None);
            return StatusCode::INTERNAL_SERVER_ERROR
        }
    };

    let hmac = match signer.sign_to_vec()
    {
        Ok(k) => k,
        Err(_) => 
        {
            crate::debug("sign failure".to_string(), None);
            return StatusCode::INTERNAL_SERVER_ERROR
        }
    };

    crate::debug(format!("post_digtest: {}, len: {}\nlocal hmac: {}, len: {}", post_digest, post_digest.len(), dump_bytes(&hmac), dump_bytes(&hmac).len()), None);

    match memcmp::eq(&hmac, &read_bytes(post_digest.clone()))
    {
        true => {},
        false => 
        {
            crate::debug(format!("bad signature: local/post\n{}\n{}", post_digest, dump_bytes(&hmac)), None);
            return StatusCode::UNAUTHORIZED
        }
    }

    // it is now safe to process the POST request

    StatusCode::ACCEPTED
}