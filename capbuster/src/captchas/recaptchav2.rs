use std::borrow::Cow;

use twocaptcha::services::recaptcha_v2::{RecaptchaV2ApiDomain, RecaptchaV2TaskType};
use typed_builder::TypedBuilder;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
pub struct RecaptchaV2TwoCaptcha<'a> {
    pub task_type: Option<RecaptchaV2TaskType<'a>>,
    pub cookies: Option<Cow<'a, str>>,
    pub user_agent: Option<Cow<'a, str>>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
pub struct RecaptchaV2<'a> {
    pub website_url: Cow<'a, str>,
    pub website_key: Cow<'a, str>,

    pub recaptcha_data_s_value: Option<Cow<'a, str>>,
    pub is_invisible: Option<bool>,
    pub api_domain: Option<RecaptchaV2ApiDomain>,
}
