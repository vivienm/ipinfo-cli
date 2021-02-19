use std::net::IpAddr;

use derive_more::From;
use futures::StreamExt;
use hyper::client::connect::HttpConnector;
use hyper::{Method, Request, Uri};
use hyper_tls::HttpsConnector;

use crate::error::{Error, Result};

#[derive(Debug, From, PartialEq)]
pub struct IpInfo(pub serde_json::Value);

impl IpInfo {
    pub fn into_value(self) -> serde_json::Value {
        let IpInfo(value) = self;
        value
    }

    pub fn as_value(&self) -> &serde_json::Value {
        let IpInfo(value) = self;
        value
    }
}

pub struct Client {
    http_client: hyper::Client<HttpsConnector<HttpConnector>, hyper::Body>,
    auth_header: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    #[inline]
    pub fn new() -> Self {
        Builder::default().build()
    }

    #[inline]
    pub fn builder() -> Builder {
        Builder::default()
    }

    async fn get_uri(&self, uri: Uri) -> Result<IpInfo> {
        // Build the query.
        let mut req_builder = Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header("Accept", "application/json");
        if let Some(auth_header) = &self.auth_header {
            req_builder = req_builder.header("Authorization", auth_header)
        }
        let req = req_builder.body(hyper::Body::empty())?;

        // Run the query.
        let resp = self.http_client.request(req).await?;
        if !resp.status().is_success() {
            return Err(Error::HyperStatus(resp.status()));
        }

        // Retrieve the response body.
        let mut stream = resp.into_body();
        let mut body = Vec::<u8>::new();
        while let Some(chunk) = stream.next().await {
            let bytes = chunk?;
            body.extend(&bytes[..]);
        }

        // Deserialize the response body.
        let info: IpInfo = serde_json::from_slice::<serde_json::Value>(&body)?.into();
        Ok(info)
    }

    pub async fn get(&self) -> Result<IpInfo> {
        self.get_uri(Uri::from_static("https://ipinfo.io")).await
    }

    pub async fn get_ip(&self, ip: &IpAddr) -> Result<IpInfo> {
        let uri = format!("https://ipinfo.io/{}", ip).parse::<Uri>()?;
        self.get_uri(uri).await
    }
}

pub struct Builder {
    http_builder: hyper::client::Builder,
    token: Option<String>,
}

impl Default for Builder {
    fn default() -> Self {
        Builder::new()
    }
}

impl Builder {
    pub fn new() -> Self {
        Self {
            http_builder: hyper::Client::builder(),
            token: None,
        }
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub fn build(&self) -> Client {
        Client {
            http_client: self.http_builder.build(HttpsConnector::new()),
            auth_header: self.token.as_ref().map(|token| format!("Bearer {}", token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Client, IpInfo};

    #[tokio::test]
    async fn test_get_ip() {
        let client = Client::new();
        let ip = "8.8.8.8".parse().unwrap();
        let info = client.get_ip(&ip).await.unwrap();
        assert_eq!(
            info,
            IpInfo(
                serde_json::from_str::<serde_json::Value>(
                    r#"{
                        "anycast": true,
                        "city": "Mountain View",
                        "country": "US",
                        "hostname": "dns.google",
                        "ip": "8.8.8.8",
                        "loc": "37.4056,-122.0775",
                        "org": "AS15169 Google LLC",
                        "postal": "94043",
                        "readme": "https://ipinfo.io/missingauth",
                        "region": "California",
                        "timezone": "America/Los_Angeles"
                    }"#
                )
                .unwrap()
            )
        );
    }
}
