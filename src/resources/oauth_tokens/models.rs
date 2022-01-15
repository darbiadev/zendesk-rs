use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TokenResponseWrapper {
    pub(crate) token: Token,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Token {
    pub(crate) url: String,
    pub(crate) id: i64,
    pub(crate) user_id: i64,
    pub(crate) client_id: i64,
    pub(crate) token: String,
    pub(crate) refresh_token: Option<String>,
    pub(crate) created_at: String,
    pub(crate) expires_at: Option<String>,
    pub(crate) used_at: Option<String>,
    pub(crate) scopes: Vec<String>,
    pub(crate) full_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TokenRequestWrapper {
    pub(crate) token: TokenRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TokenRequest {
    pub(crate) client_id: String,
    pub(crate) scopes: Vec<String>,
}
