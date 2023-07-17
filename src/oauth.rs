//! based on information provided at https://developers.notion.com/docs/authorization

use crate::models::oauth::token;
use crate::Error;
use reqwest::{Client, ClientBuilder, RequestBuilder, Url};
use tracing::Instrument;

static OAUTH_URL_BASE: &str = "https://api.notion.com/v1/oauth/authorize";
static RESPONSE_TYPE: &str = "code";
static OWNER: &str = "user";
static GRANT_TYPE: &str = "authorization_code";

pub struct OAuthClient {
    client: Client,
    client_id: String,
    client_secret: String,
    redirect_uri: Url,
}

impl OAuthClient {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: Url,
    ) -> Result<Self, Error> {
        let client = ClientBuilder::new()
            .build()
            .map_err(|source| Error::ErrorBuildingClient { source })?;

        Ok(Self {
            client,
            client_id,
            client_secret,
            redirect_uri,
        })
    }

    pub fn authorization_url(
        &self,
        state: &str,
    ) -> Url {
        let mut url = Url::parse(OAUTH_URL_BASE).unwrap();
        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", self.redirect_uri.as_ref())
            .append_pair("response_type", RESPONSE_TYPE)
            .append_pair("state", state)
            .append_pair("owner", OWNER);
        url
    }

    pub async fn exchange_code_for_token(
        &self,
        code: &str,
    ) -> Result<token::Token, Error> {
        let url = Url::parse("https://api.notion.com/v1/oauth/token").unwrap();

        let body = ExchangeCodeRequest {
            grant_type: GRANT_TYPE.to_string(),
            code: code.to_string(),
            redirect_uri: self.redirect_uri.clone(),
        };

        let builder = self
            .client
            .post(url)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .json(&body);

        self.make_json_request(builder).await
    }

    async fn make_json_request<T>(
        &self,
        request: RequestBuilder,
    ) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let request = request.build()?;
        let url = request.url();
        tracing::trace!(
            method = request.method().as_str(),
            url = url.as_str(),
            "Sending request"
        );

        let res = self
            .client
            .execute(request)
            .instrument(tracing::trace_span!("Sending request"))
            .await
            .map_err(|source| Error::RequestFailed { source })?;

        let success = res.status().is_success();

        let json = res
            .text()
            .instrument(tracing::trace_span!("Reading response"))
            .await
            .map_err(|source| Error::ResponseIoError { source })?;

        tracing::debug!("JSON Response: {}", json);
        #[cfg(test)]
        {
            dbg!(serde_json::from_str::<serde_json::Value>(&json)
                .map_err(|source| Error::JsonParseError { source })?);
        }

        if success {
            let result: T =
                serde_json::from_str(&json).map_err(|source| Error::JsonParseError { source })?;
            Ok(result)
        } else {
            let error = serde_json::from_str::<OAuthError>(&json)
                .map_err(|source| Error::JsonParseError { source })?;
            Err(Error::OAuthError { error })
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct OAuthError {
    error: String,
    error_description: String,
}

#[derive(serde::Serialize)]
struct ExchangeCodeRequest {
    grant_type: String,
    code: String,
    redirect_uri: Url,
}
