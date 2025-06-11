use api_builder::{Endpoint, api_endpoint};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// [Reference](https://2captcha.com/api-docs/get-balance#request-properties).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetBalancePayload<'a> {
    client_key: &'a str,
}

#[api_endpoint(method = GET, path = "\"/getBalance\"", self_as_body = "application/json")]
impl Endpoint for GetBalancePayload<'_> {}

/// [Reference](https://2captcha.com/api-docs/get-balance#response-example).
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub balance: f32,
}
