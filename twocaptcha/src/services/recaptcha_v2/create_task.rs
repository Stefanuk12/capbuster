use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::proxy::{Proxy, ProxyTypeLowercase};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize)]
pub enum RecaptchaV2TaskType<'a> {
    /// [Reference](https://2captcha.com/api-docs/recaptcha-v2#recaptchav2task-task-type-specification).
    RecaptchaV2Task(Proxy<'a, ProxyTypeLowercase>),
    #[default]
    RecaptchaV2TaskProxyless,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize)]
pub enum RecaptchaV2ApiDomain {
    #[serde(rename = "google.com")]
    #[default]
    Google,
    #[serde(rename = "recaptcha.net")]
    Recaptcha,
}

/// [Reference](https://2captcha.com/api-docs/recaptcha-v2#recaptchav2taskproxyless-task-type-specification)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaV2Task<'a> {
    #[builder(default)]
    #[serde(rename = "type")]
    task_type: RecaptchaV2TaskType<'a>,
    #[serde(rename = "websiteURL")]
    website_url: Cow<'a, str>,
    website_key: Cow<'a, str>,

    #[builder(default)]
    recaptcha_data_s_value: Option<Cow<'a, str>>,
    #[builder(default)]
    is_invisible: Option<bool>,
    #[builder(default)]
    user_agent: Option<Cow<'a, str>>,
    #[builder(default)]
    cookies: Option<Cow<'a, str>>,
    #[builder(default)]
    api_domain: Option<RecaptchaV2ApiDomain>,
}
