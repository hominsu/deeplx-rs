use std::{sync::Arc, time::Duration};

use ::reqwest::{StatusCode, Url, header::HeaderMap};
use impit::{
    cookie::Jar,
    errors::ImpitError,
    fingerprint::{
        BrowserFingerprint, CertificateCompressionAlgorithm, CipherSuite, EchConfig, EchMode,
        ExtensionType, HpkeKemId, Http2Fingerprint, KeyExchangeGroup, SignatureAlgorithm,
        TlsExtensions, TlsFingerprint,
    },
    impit::Impit,
    request::RequestOptions,
};

use super::{ClientConfig, Error};

#[macro_export]
macro_rules! impersonated_chrome_version {
    () => {
        "120"
    };
}
const WARMUP_TIMEOUT: Duration = Duration::from_secs(5);

/// Chrome 120 fingerprint module
mod chrome_120 {
    use super::*;

    /// Returns the complete Chrome 120 fingerprint
    pub fn fingerprint() -> BrowserFingerprint {
        BrowserFingerprint::new(
            "Chrome",
            impersonated_chrome_version!(),
            tls_fingerprint(),
            http2_fingerprint(),
            headers(),
        )
    }

    /// Chrome 120 TLS fingerprint
    fn tls_fingerprint() -> TlsFingerprint {
        TlsFingerprint::new(
            // Cipher suites in Chrome 120 preference order
            vec![
                CipherSuite::Grease,
                CipherSuite::TLS13_AES_128_GCM_SHA256,
                CipherSuite::TLS13_AES_256_GCM_SHA384,
                CipherSuite::TLS13_CHACHA20_POLY1305_SHA256,
                CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
                CipherSuite::TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
                CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
                CipherSuite::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
                CipherSuite::TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,
                CipherSuite::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
                CipherSuite::TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA,
                CipherSuite::TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA,
                CipherSuite::TLS_RSA_WITH_AES_128_GCM_SHA256,
                CipherSuite::TLS_RSA_WITH_AES_256_GCM_SHA384,
                CipherSuite::TLS_RSA_WITH_AES_128_CBC_SHA,
                CipherSuite::TLS_RSA_WITH_AES_256_CBC_SHA,
            ],
            // Key exchange groups in uTLS HelloChrome_120 order.
            vec![
                KeyExchangeGroup::Grease,
                KeyExchangeGroup::X25519,
                KeyExchangeGroup::Secp256r1,
                KeyExchangeGroup::Secp384r1,
            ],
            // Signature algorithms
            vec![
                SignatureAlgorithm::EcdsaSecp256r1Sha256,
                SignatureAlgorithm::RsaPssRsaSha256,
                SignatureAlgorithm::RsaPkcs1Sha256,
                SignatureAlgorithm::EcdsaSecp384r1Sha384,
                SignatureAlgorithm::RsaPssRsaSha384,
                SignatureAlgorithm::RsaPkcs1Sha384,
                SignatureAlgorithm::RsaPssRsaSha512,
                SignatureAlgorithm::RsaPkcs1Sha512,
            ],
            // TLS extensions configuration
            TlsExtensions::new(
                true,                                                // server_name
                true,                                                // status_request
                true,                                                // supported_groups
                true,                                                // signature_algorithms
                true, // application_layer_protocol_negotiation
                true, // signed_certificate_timestamp
                true, // key_share
                true, // psk_key_exchange_modes
                true, // supported_versions
                Some(vec![CertificateCompressionAlgorithm::Brotli]), // compress_certificate
                true, // application_settings
                false, // delegated_credentials (Chrome doesn't use)
                None, // record_size_limit (Chrome doesn't use)
                // Extension order (critical for fingerprinting)
                vec![
                    ExtensionType::Grease,
                    ExtensionType::ServerName,
                    ExtensionType::ExtendedMasterSecret,
                    ExtensionType::RenegotiationInfo,
                    ExtensionType::SupportedGroups,
                    ExtensionType::EcPointFormats,
                    ExtensionType::SessionTicket,
                    ExtensionType::ApplicationLayerProtocolNegotiation,
                    ExtensionType::StatusRequest,
                    ExtensionType::SignatureAlgorithms,
                    ExtensionType::SignedCertificateTimestamp,
                    ExtensionType::KeyShare,
                    ExtensionType::PskKeyExchangeModes,
                    ExtensionType::SupportedVersions,
                    ExtensionType::CompressCertificate,
                    ExtensionType::ApplicationSettings,
                ],
            ),
            // ECH configuration (GREASE mode)
            Some(EchConfig::new(
                EchMode::Grease {
                    hpke_suite: HpkeKemId::DhKemX25519HkdfSha256,
                },
                None,
            )),
            // ALPN protocols
            vec![b"h2".to_vec(), b"http/1.1".to_vec()],
        )
    }

    /// Chrome 120 HTTP/2 fingerprint
    fn http2_fingerprint() -> Http2Fingerprint {
        Http2Fingerprint {
            pseudo_header_order: vec![
                ":method".to_string(),
                ":authority".to_string(),
                ":scheme".to_string(),
                ":path".to_string(),
                ":protocol".to_string(),
                ":status".to_string(),
            ],
            initial_stream_window_size: Some(6_291_456),
            // h2 treats this as the target connection window. Chrome's captured
            // WINDOW_UPDATE increment is 15_663_105 on top of the default 65_535.
            initial_connection_window_size: Some(15_728_640),
            max_header_list_size: Some(262_144),
        }
    }

    /// Chrome 120 HTTP headers
    fn headers() -> Vec<(String, String)> {
        vec![
            (
                "sec-ch-ua".to_string(),
                "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\""
                    .to_string(),
            ),
            ("sec-ch-ua-mobile".to_string(), "?0".to_string()),
            ("sec-ch-ua-platform".to_string(), "\"macOS\"".to_string()),
            (
                "user-agent".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            ),
            (
                "accept".to_string(),
                "text/html,application/xhtml+xml,application/xml,application/json;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".to_string(),
            ),
            ("sec-fetch-site".to_string(), "none".to_string()),
            ("sec-fetch-mode".to_string(), "navigate".to_string()),
            ("sec-fetch-dest".to_string(), "document".to_string()),
            ("accept-language".to_string(), "zh-CN,zh;q=0.9".to_string()),
        ]
    }
}

pub(crate) struct TransportResponse {
    pub status: StatusCode,
    pub body: String,
}

#[derive(Clone)]
pub(crate) struct ImpitTransport {
    client: Arc<Impit<Jar>>,
    timeout: Duration,
}

impl ImpitTransport {
    pub fn new(config: &ClientConfig) -> Result<Self, Error> {
        let mut builder = Impit::<Jar>::builder()
            .with_fingerprint(chrome_120::fingerprint())
            .with_cookie_store(Jar::default())
            .with_default_timeout(config.timeout);

        if let Some(proxy) = &config.proxy {
            builder = builder.with_proxy(proxy.as_str().to_string());
        }

        Ok(Self {
            client: Arc::new(builder.build()?),
            timeout: config.timeout,
        })
    }

    pub async fn post(
        &self,
        url: Url,
        headers: HeaderMap,
        body: Vec<u8>,
    ) -> Result<TransportResponse, Error> {
        let headers = headers_to_pairs(headers)?;
        let response = self
            .client
            .post(
                url.to_string(),
                Some(body),
                Some(RequestOptions {
                    headers,
                    timeout: None,
                    http3_prior_knowledge: false,
                }),
            )
            .await
            .map_err(|err| self.map_impit_error(err, self.timeout))?;
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|err| self.map_reqwest_error(err, self.timeout))?;

        Ok(TransportResponse { status, body })
    }

    pub async fn warmup(&self) -> Result<(), Error> {
        self.client
            .get(
                "https://www.deepl.com/translator".to_string(),
                None,
                Some(RequestOptions {
                    headers: Vec::new(),
                    timeout: Some(Some(WARMUP_TIMEOUT)),
                    http3_prior_knowledge: false,
                }),
            )
            .await
            .map_err(|err| self.map_impit_error(err, WARMUP_TIMEOUT))?
            .error_for_status()
            .map_err(|err| self.map_reqwest_error(err, WARMUP_TIMEOUT))?;

        Ok(())
    }

    fn map_reqwest_error(&self, err: ::reqwest::Error, timeout: Duration) -> Error {
        if err.is_timeout() {
            Error::Timeout(timeout)
        } else {
            Error::Transport(err)
        }
    }

    fn map_impit_error(&self, err: ImpitError, timeout: Duration) -> Error {
        match err {
            ImpitError::TimeoutException(_)
            | ImpitError::ConnectTimeout
            | ImpitError::ReadTimeout
            | ImpitError::WriteTimeout
            | ImpitError::PoolTimeout => Error::Timeout(timeout),
            err => Error::Impit(err),
        }
    }
}

fn headers_to_pairs(headers: HeaderMap) -> Result<Vec<(String, String)>, Error> {
    headers
        .iter()
        .map(|(name, value)| {
            let value = value.to_str().map_err(|_| Error::InvalidHeader)?;
            Ok((name.as_str().to_string(), value.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oneshot;
    use crate::{Auth, SourceLang, TargetLang, TranslateRequest};

    #[test]
    fn chrome_120_headers_match_default_fingerprint() {
        let headers = chrome_120::fingerprint().headers;

        assert!(headers.contains(&("accept-language".to_string(), "zh-CN,zh;q=0.9".to_string())));
        assert!(headers.contains(&("sec-ch-ua-mobile".to_string(), "?0".to_string())));
        assert!(headers.contains(&("sec-ch-ua-platform".to_string(), "\"macOS\"".to_string())));
        assert!(
            headers.contains(&(
                "sec-ch-ua".to_string(),
                "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\""
                    .to_string()
            ))
        );
        assert!(headers.contains(&(
            "user-agent".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
                .to_string()
        )));
    }

    #[tokio::test]
    #[ignore = "sends a live request to capture server-observed headers"]
    async fn live_capture_request_headers() {
        let transport = ImpitTransport::new(&ClientConfig::default()).unwrap();
        let request = TranslateRequest::builder()
            .text("hello from dlx header logging test")
            .unwrap()
            .source(SourceLang::parse("EN").unwrap())
            .target(TargetLang::parse("ZH").unwrap())
            .build()
            .unwrap();
        let body = oneshot::build_body(
            &request,
            &Auth::Anonymous,
            "00000000-0000-0000-0000-000000000000",
        )
        .unwrap();
        let headers = oneshot::post_headers(&Auth::Anonymous, body.len()).unwrap();
        let response = transport
            .post(
                Url::parse("https://tls.peet.ws/api/all").unwrap(),
                headers,
                body,
            )
            .await
            .unwrap();

        println!("status={}", response.status);
        println!("{}", response.body);

        assert!(response.status.is_success());
    }
}
