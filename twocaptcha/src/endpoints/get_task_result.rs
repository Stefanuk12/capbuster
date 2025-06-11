use api_builder::{Endpoint, api_endpoint};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::services::arkose_labs::ArkoseLabsSolution;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskResultPayload<'a> {
    client_key: &'a str,
    task_id: u64,
}

#[api_endpoint(method = GET, path = "\"/getTaskResult\"", self_as_body = "application/json")]
impl Endpoint for GetTaskResultPayload<'_> {}

// TODO: `untagged` might cause issues, look into ways to make the user define the solution type
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
#[serde(untagged)]
pub enum GetTaskResultSolution {
    ArkoseLabs(ArkoseLabsSolution),
}

/// [Reference](https://2captcha.com/api-docs/get-task-result#response-specification).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskResultResponseReady {
    pub solution: GetTaskResultSolution,
    pub cost: String,
    pub ip: String,
    pub create_time: u64,
    pub end_time: u64,
    pub solve_count: u64,
}

/// [Reference](https://2captcha.com/api-docs/get-task-result#request-properties).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
#[serde(tag = "status")]
pub enum GetTaskResultResponse {
    /// [Reference](https://2captcha.com/api-docs/get-task-result#in-progress).
    Processing,
    Ready(GetTaskResultResponseReady),
}
