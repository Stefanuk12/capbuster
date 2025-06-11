use std::borrow::Cow;

use serde::{Serialize, Serializer};

pub trait ProxyTypeFormatter {
    fn fmt(&self, proxy_type: &ProxyType) -> &'static str;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ProxyTypeLowercase;
impl ProxyTypeFormatter for ProxyTypeLowercase {
    fn fmt(&self, proxy_type: &ProxyType) -> &'static str {
        match proxy_type {
            ProxyType::Http => "http",
            ProxyType::Socks4 => "socks4",
            ProxyType::Socks5 => "socks5",
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ProxyTypeUppercase;
impl ProxyTypeFormatter for ProxyTypeUppercase {
    fn fmt(&self, proxy_type: &ProxyType) -> &'static str {
        match proxy_type {
            ProxyType::Http => "HTTP",
            ProxyType::Socks4 => "SOCKS4",
            ProxyType::Socks5 => "SOCKS5",
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize)]
pub enum ProxyType {
    #[default]
    Http,
    Socks4,
    Socks5,
}

fn serialize_proxy_type<S, T>(proxy_type: &(ProxyType, T), serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ProxyTypeFormatter,
{
    serializer.serialize_str(proxy_type.1.fmt(&proxy_type.0))
}

/// [Reference](https://2captcha.com/api-docs/proxy#post-parameters-for-proxies).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct Proxy<'a, T: ProxyTypeFormatter> {
    #[serde(serialize_with = "serialize_proxy_type")]
    proxy_type: (ProxyType, T),
    address: Cow<'a, str>,
    port: u16,
    login: Option<Cow<'a, str>>,
    password: Option<Cow<'a, str>>,
}
