use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::proxy::{Proxy, ProxyTypeUppercase};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize)]
pub enum ArkoseLabsTaskType<'a> {
    /// [Reference](https://2captcha.com/api-docs/arkoselabs-funcaptcha#funcaptchatask-task-type-specification).
    FunCaptchaTask(Proxy<'a, ProxyTypeUppercase>),
    #[default]
    FunCaptchaTaskProxyless,
}

/// [Reference](https://2captcha.com/api-docs/arkoselabs-funcaptcha#task-specification).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ArkoseLabsTask<'a, T> {
    #[builder(default)]
    #[serde(rename = "type")]
    task_type: ArkoseLabsTaskType<'a>,
    #[serde(rename = "websiteURL")]
    website_url: Cow<'a, str>,
    website_public_key: Cow<'a, str>,
    #[serde(rename = "funcaptchaApiJSSubdomain")]
    subdomain: Option<Cow<'a, str>>,
    #[builder(default)]
    data: Option<T>,
    #[builder(default)]
    user_agent: Option<Cow<'a, str>>,
}
