use std::path::Path;

use serde::{Serialize, Deserialize};

use crate::util::read_file_utf8;

pub const CONFIG_PATH: &str = "config.json";

/// Configure the IP throttler
/// - ```max_requests_per_second```: includes all requests to html and resources per second per ip
/// - ```timeout_millis```: a cool off period between IP-blocks
/// - ```clear_period_seconds```: time period to clear all stored IPs
#[derive(Clone, Serialize, Deserialize)]
pub struct ThrottleConfig
{
    pub max_requests_per_second: f64,
    pub timeout_millis: u128,
    pub clear_period_seconds: u64
}

/// Configure the server
/// - ```port_https```: https port to serve on
/// - ```port_http```: http port to serve on
/// - ```cert_path```: ssl certificate
/// - ```key_path```: ssl key
/// - ```domain```: domain name for https redirect etc.
/// - ```throttle```: [ThrottleConfig]
/// - ```api_token```: token to use for the server's POST api
/// - ```cache_period_seconds```: max cache age for generated content
#[derive(Clone, Serialize, Deserialize)]
pub struct Config
{
    pub port_https: u16,
    pub port_http: u16,
    pub cert_path: String,
    pub key_path: String,
    pub domain: String,
    pub throttle: ThrottleConfig,
    pub api_token: String,
    pub cache_period_seconds: u16
}

pub fn read_config() -> Option<Config>
{
    if Path::new(CONFIG_PATH).exists()
    {
        let data = match read_file_utf8(CONFIG_PATH)
        {
            Some(d) => d,
            None =>
            {
                println!("Error reading configuration file {} no data", CONFIG_PATH);
                return None
            }
        };

        let config: Config = match serde_json::from_str(&data)
        {
            Ok(data) => {data},
            Err(why) => 
            {
                println!("Error reading configuration file {}\n{}", CONFIG_PATH, why);
                return None
            }
        };

        Some(config)
    }
    else 
    {
        println!("Error configuration file {} does not exist", CONFIG_PATH);
        None
    }
}