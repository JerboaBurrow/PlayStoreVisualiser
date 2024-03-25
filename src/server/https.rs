use crate::
{
    server::config::{read_config, Config}, 
    server::throttle::{handle_throttle, IpThrottler}
};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::
{
    middleware, response::IntoResponse, routing::post, Router
};
use axum_server::tls_rustls::RustlsConfig;

use super::{api::{ApiRequest, Generate}, filter_cors_preflight};

/// An https server that reads a directory configured with [Config]
/// ```.html``` pages and resources, then serves them.
/// # Example
/// ```no_run
/// use psv::server::https::Server;
/// #[tokio::main]
/// async fn main() 
/// {
///     let server = Server::new(0,0,0,0);
///     server.serve().await;
/// }
/// ```
pub struct Server
{
    addr: SocketAddr,
    router: Router,
    config: Config
}

/// Checks a uri has a leading /, adds it if not
pub fn parse_uri(uri: String, path: String) -> String
{
    if uri.starts_with(&path)
    {
        uri.replace(&path, "/")
    }
    else if uri.starts_with("/")
    {
        uri
    }
    else
    {
        "/".to_string()+&uri
    }
}

impl Server 
{
    pub fn new 
    (
        a: u8,
        b: u8,
        c: u8,
        d: u8
    ) 
    -> Server
    {

        let config = match read_config()
        {
            Some(c) => c,
            None =>
            {
                std::process::exit(1)
            }
        };

        let requests: IpThrottler = IpThrottler::new
        (
            config.throttle.max_requests_per_second, 
            config.throttle.timeout_millis,
            config.throttle.clear_period_seconds
        );

        let throttle_state = Arc::new(Mutex::new(requests));

        let mut router: Router<(), axum::body::Body> = Router::new();

        router = router.layer(middleware::from_fn_with_state(throttle_state.clone(), handle_throttle));
        router = router.layer(middleware::from_fn(Generate::filter));
        router = router.layer(middleware::from_fn(filter_cors_preflight));

        Server
        {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(a,b,c,d)), config.port_https),
            router,
            config
        }
    }

    pub fn get_addr(self: Server) -> SocketAddr
    {
        self.addr
    }

    pub async fn serve(self: Server)
    {

        // configure https

        let cert_path = self.config.cert_path;
        let key_path = self.config.key_path;

        let config = match RustlsConfig::from_pem_file(
            PathBuf::from(cert_path.clone()),
            PathBuf::from(key_path.clone())
        )
        .await
        {
            Ok(c) => c,
            Err(e) => 
            {
                println!("error while reading certificates in {} and key {}\n{}", cert_path, key_path, e);
                std::process::exit(1);
            }
        };

        axum_server::bind_rustls(self.addr, config)
        .serve(self.router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
    }

}