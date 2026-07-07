use std::time::Duration;

use ::reqwest::{Client as HttpClient, StatusCode, Url, header::HeaderMap};

use crate::{ClientConfig, Error};

#[derive(Clone)]
pub(crate) struct ReqwestTransport {
    client: HttpClient,
    timeout: Duration,
}

pub(crate) struct TransportResponse {
    pub status: StatusCode,
    pub body: String,
}

impl ReqwestTransport {
    pub fn new(config: &ClientConfig) -> Result<Self, Error> {
        let mut builder = HttpClient::builder()
            .timeout(config.timeout)
            .cookie_store(true)
            .user_agent(config.fingerprint.user_agent());

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(proxy) = &config.proxy {
            builder = builder.proxy(::reqwest::Proxy::all(proxy.as_str())?);
        }

        Ok(Self {
            client: builder.build()?,
            timeout: config.timeout,
        })
    }

    pub async fn post(
        &self,
        url: Url,
        headers: HeaderMap,
        body: Vec<u8>,
    ) -> Result<TransportResponse, Error> {
        let response = self
            .client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await
            .map_err(|err| self.map_reqwest_error(err))?;
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|err| self.map_reqwest_error(err))?;

        Ok(TransportResponse { status, body })
    }

    pub async fn warmup(&self) -> Result<(), Error> {
        self.client
            .get("https://www.deepl.com/translator")
            .send()
            .await
            .map_err(|err| self.map_reqwest_error(err))?
            .error_for_status()
            .map_err(|err| self.map_reqwest_error(err))?;

        Ok(())
    }

    fn map_reqwest_error(&self, err: ::reqwest::Error) -> Error {
        if err.is_timeout() {
            Error::Timeout(self.timeout)
        } else {
            Error::Transport(err)
        }
    }
}
