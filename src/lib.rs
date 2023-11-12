use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::resources::oauth_tokens::models::{
    TokenRequest, TokenRequestWrapper, TokenResponseWrapper,
};

pub mod resources;

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    base_url: String,
    email: String,
    api_token: String,
    oauth_client_id: String,
    oauth_token: Option<String>,
}

impl Client {
    #[must_use]
    pub fn new(
        base_url: String,
        email: String,
        api_token: String,
        oauth_client_id: String,
    ) -> Client {
        Client {
            base_url,
            email,
            api_token,
            oauth_client_id,
            oauth_token: None,
        }
    }

    /// # Panics
    ///
    /// Will panic if the if the HTTP request fails.
    pub async fn update_token(&mut self) {
        let token_request_data = TokenRequestWrapper {
            token: TokenRequest {
                client_id: self.oauth_client_id.parse().unwrap(),
                scopes: vec![String::from("read")],
            },
        };
        let token_wrapper = reqwest::Client::new()
            .request(
                reqwest::Method::POST,
                format!("{}/{}", &self.base_url, "oauth/tokens.json"),
            )
            .basic_auth(format!("{}/token", &self.email), Some(&self.api_token))
            .json(&token_request_data)
            .send()
            .await
            .unwrap()
            .json::<TokenResponseWrapper>()
            .await
            .unwrap();
        self.oauth_token = Some(token_wrapper.token.full_token);
    }

    /// # Errors
    ///
    /// Will return `reqwest::Error` if the HTTP request fails.
    ///
    /// # Panics
    ///
    /// Will panic if the `oauth_token` is `None`.
    pub async fn make_request<T>(self, method: Method, resource: String) -> reqwest::Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", &self.base_url, resource);

        let request = reqwest::Client::new()
            .request(method, url)
            .bearer_auth(self.oauth_token.as_ref().unwrap());

        request.send().await?.json::<T>().await
    }
}
