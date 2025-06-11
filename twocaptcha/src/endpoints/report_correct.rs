use api_builder::{Endpoint, api_endpoint};
use serde::Serialize;
use typed_builder::TypedBuilder;

/// [Reference](https://2captcha.com/api-docs/report-correct#request-properties).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ReportCorrectPayload<'a> {
    client_key: &'a str,
    task_id: u64,
}

#[api_endpoint(method = GET, path = "\"/reportCorrect\"", self_as_body = "application/json")]
impl Endpoint for ReportCorrectPayload<'_> {}

/// [Reference](https://2captcha.com/api-docs/report-correct#response-example).
pub type ReportCorrectResponse = ();
