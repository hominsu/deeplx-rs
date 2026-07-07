use rand::Rng;
use reqwest::Url;

use crate::{
    Auth, ClientConfig, Endpoint, Error, TranslateRequest, TranslateResponse, WarmupMode, oneshot,
    transport::ReqwestTransport,
};

#[derive(Clone)]
pub struct Client {
    config: ClientConfig,
    transport: ReqwestTransport,
    instance_id: String,
}

#[derive(Clone, Debug, Default)]
pub struct ClientBuilder {
    config: ClientConfig,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn from_config(config: ClientConfig) -> Result<Self, Error> {
        let transport = ReqwestTransport::new(&config)?;

        Ok(Self {
            config,
            transport,
            instance_id: new_instance_id(),
        })
    }

    pub async fn translate(&self, request: TranslateRequest) -> Result<TranslateResponse, Error> {
        self.translate_with_auth(request, self.config.auth.clone())
            .await
    }

    pub async fn translate_with_auth(
        &self,
        request: TranslateRequest,
        auth: Auth,
    ) -> Result<TranslateResponse, Error> {
        let url = self.config.endpoint.url_for_auth(&auth)?;
        let headers = oneshot::headers(&self.config, &auth)?;
        let body = oneshot::build_body(&request, &self.config, &auth, &self.instance_id)?;
        let response = self.transport.post(url, headers, body).await?;

        oneshot::parse_response(response.status, response.body.as_str(), &request.target)
    }

    pub async fn warmup(&self) -> Result<(), Error> {
        self.transport.warmup().await
    }
}

impl ClientBuilder {
    pub fn config(mut self, config: ClientConfig) -> Self {
        self.config = config;
        self
    }

    pub fn auth(mut self, auth: Auth) -> Self {
        self.config.auth = auth;
        self
    }

    pub fn endpoint(mut self, endpoint: Endpoint) -> Self {
        self.config.endpoint = endpoint;
        self
    }

    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn proxy(mut self, proxy: Url) -> Self {
        self.config.proxy = Some(proxy);
        self
    }

    pub fn warmup(mut self, warmup: WarmupMode) -> Self {
        self.config.warmup = warmup;
        self
    }

    pub fn build(self) -> Result<Client, Error> {
        Client::from_config(self.config)
    }
}

fn new_instance_id() -> String {
    let mut bytes = [0u8; 16];
    rand::rng().fill(&mut bytes);

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
        bytes[9],
        bytes[10],
        bytes[11],
        bytes[12],
        bytes[13],
        bytes[14],
        bytes[15]
    )
}
