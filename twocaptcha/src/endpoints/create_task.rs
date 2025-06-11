use std::borrow::Cow;

use api_builder::{Endpoint, api_endpoint};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::services::{arkose_labs::ArkoseLabsTask, recaptcha_v2::RecaptchaV2Task};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
#[serde(untagged)]
pub enum CreateTaskData<'a, T = ()> {
    ArkoseLabs(ArkoseLabsTask<'a, T>),
    RecaptchaV2(RecaptchaV2Task<'a>),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CreateTaskLanguagePool {
    #[default]
    En,
    Rn,
}

/// [Reference](https://2captcha.com/api-docs/create-task#request-properties).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskPayload<'a, T> {
    client_key: &'a str,
    task: CreateTaskData<'a, T>,

    #[builder(default, setter(strip_option))]
    language_pool: Option<CreateTaskLanguagePool>,
    #[builder(default, setter(strip_option))]
    callback_url: Option<Cow<'a, str>>,
    #[builder(default, setter(strip_option))]
    soft_id: Option<i32>,
}

#[api_endpoint(method = GET, path = "\"/createTask\"", self_as_body = "application/json")]
impl<T: Serialize> Endpoint for CreateTaskPayload<'_, T> {}

/// [Reference](https://2captcha.com/api-docs/create-task#response-example).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
pub struct CreateTaskResponse {
    pub task_id: u64,
}
