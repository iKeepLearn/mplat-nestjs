use crate::backend::SimpleInput;
use actix_web::ResponseError;
use actix_web::dev::ServiceRequest;
use std::future::{Ready, ready};
use std::net::{AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use thiserror::Error;

type CustomFn = Box<dyn Fn(&ServiceRequest) -> Result<String, actix_web::Error>>;

pub type SimpleInputFuture<T = String> = Ready<Result<SimpleInput<T>, actix_web::Error>>;

/// Utility to create a input function that produces a [SimpleInput].
///
/// You should take care to ensure that you are producing unique keys per backend.
///
/// This will not be of any use if you want to use dynamic interval/request policies
/// or perform an asynchronous option; you should instead write your own input function.
pub struct SimpleInputFunctionBuilder {
    interval: Duration,
    max_requests: u64,
    real_ip_key: bool,
    peer_ip_key: bool,
    path_key: bool,
    custom_key: Option<String>,
    custom_fn: Option<CustomFn>,
}

impl SimpleInputFunctionBuilder {
    pub fn new(interval: Duration, max_requests: u64) -> Self {
        Self {
            interval,
            max_requests,
            real_ip_key: false,
            peer_ip_key: false,
            path_key: false,
            custom_key: None,
            custom_fn: None,
        }
    }

    /// Adds the client's real IP to the rate limiting key.
    ///
    /// # Security
    ///
    /// This calls
    /// [ConnectionInfo::realip_remote_addr()](actix_web::dev::ConnectionInfo::realip_remote_addr)
    /// internally which is only suitable for Actix applications deployed behind a proxy that you
    /// control.
    ///
    /// # IPv6
    ///
    /// IPv6 addresses will be grouped into a single key per /64
    pub fn real_ip_key(mut self) -> Self {
        self.real_ip_key = true;
        self
    }

    /// Adds the connection peer IP to the rate limiting key.
    ///
    /// This is suitable when clients connect directly to the Actix application.
    ///
    /// # IPv6
    ///
    /// IPv6 addresses will be grouped into a single key per /64
    pub fn peer_ip_key(mut self) -> Self {
        self.peer_ip_key = true;
        self
    }

    /// Add the request path to the rate limiting key
    pub fn path_key(mut self) -> Self {
        self.path_key = true;
        self
    }

    /// Add a custom component to the rate limiting key
    pub fn custom_key(mut self, key: &str) -> Self {
        self.custom_key = Some(key.to_owned());
        self
    }

    /// Dynamically add a custom component to the rate limiting key
    pub fn custom_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&ServiceRequest) -> Result<String, actix_web::Error> + 'static,
    {
        self.custom_fn = Some(Box::new(f));
        self
    }

    pub fn build(self) -> impl Fn(&ServiceRequest) -> SimpleInputFuture + 'static {
        move |req| {
            ready((|| {
                let mut components = Vec::new();
                let info = req.connection_info();
                if let Some(custom) = &self.custom_key {
                    components.push(custom.clone());
                }
                if self.real_ip_key {
                    components.push(string_ip_key(info.realip_remote_addr()))
                }
                if self.peer_ip_key {
                    components.push(string_ip_key(info.peer_addr()))
                }
                if self.path_key {
                    components.push(req.path().to_owned());
                }
                if let Some(f) = &self.custom_fn {
                    components.push(f(req)?)
                }
                let key = components.join("-");

                Ok(SimpleInput {
                    interval: self.interval,
                    max_requests: self.max_requests,
                    key,
                })
            })())
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to parse remote IP address: {0}")]
    InvalidIpError(
        #[source]
        #[from]
        AddrParseError,
    ),
}

impl ResponseError for Error {}

/// Generate a string key for backend. Uses more memory but can easily be combined with other
/// data like path or custom keys.
///
/// Groups IPv6 addresses together, see:
/// https://adam-p.ca/blog/2022/02/ipv6-rate-limiting/
/// https://support.cloudflare.com/hc/en-us/articles/115001635128-Configuring-Cloudflare-Rate-Limiting
pub fn string_ip_key(ip_str: Option<&str>) -> String {
    let ip = parse_ip(ip_str);
    match ip {
        IpAddr::V4(v4) => v4.to_string(),
        IpAddr::V6(v6) => {
            if let Some(v4) = v6.to_ipv4() {
                return v4.to_string();
            }
            let zeroes = [0u16; 4];
            let concat = [&v6.segments()[0..4], &zeroes].concat();
            let concat: [u16; 8] = concat.try_into().unwrap();
            let subnet = Ipv6Addr::from(concat);
            format!("{}/64", subnet)
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum RateLimitIpAddr {
    V4(Ipv4Addr),
    V6([u16; 4]),
}

impl From<IpAddr> for RateLimitIpAddr {
    fn from(value: IpAddr) -> Self {
        match value {
            IpAddr::V4(addr) => RateLimitIpAddr::V4(addr),
            IpAddr::V6(addr) => RateLimitIpAddr::V6(addr.segments()[..4].try_into().unwrap()),
        }
    }
}

/// Generate a raw byte key for backend which uses less memory.
pub fn raw_ip_key(ip_str: Option<&str>) -> RateLimitIpAddr {
    parse_ip(ip_str).into()
}

fn parse_ip(addr: Option<&str>) -> IpAddr {
    if let Some(addr) = addr {
        if let Ok(ip) = IpAddr::from_str(addr) {
            return ip;
        } else if let Ok(socket) = SocketAddr::from_str(addr) {
            return socket.ip();
        }
    }
    Ipv4Addr::new(127, 0, 0, 1).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_key() {
        // Check that IPv4 addresses are preserved
        assert_eq!(string_ip_key(Some("142.250.187.206")), "142.250.187.206");
        // Check that IPv4 mapped addresses are preserved
        assert_eq!(
            string_ip_key(Some("::FFFF:142.250.187.206")),
            "142.250.187.206"
        );
        // Check that IPv6 addresses are grouped into /64 subnets
        assert_eq!(
            string_ip_key(Some("2a00:1450:4009:81f::200e")),
            "2a00:1450:4009:81f::/64"
        );
    }
    #[test]
    fn test_get_ip() {
        // Check that IPv4 addresses are preserved
        assert_eq!(
            raw_ip_key(Some("142.250.187.206")),
            "142.250.187.206".parse::<IpAddr>().unwrap().into()
        );
        // Check that IPv6 addresses are grouped into /64 subnets
        assert_eq!(
            dbg!(raw_ip_key(Some("2a00:1450:4009:81f::200e"))),
            RateLimitIpAddr::V6([0x2a00, 0x1450, 0x4009, 0x81f])
        );
        assert_eq!(
            raw_ip_key(Some("[2a00:1450:4009:81f::200e]:123")),
            RateLimitIpAddr::V6([0x2a00, 0x1450, 0x4009, 0x81f])
        );
    }
}
