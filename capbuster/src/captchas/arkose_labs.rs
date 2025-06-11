use std::borrow::Cow;

use serde::Serialize;
use twocaptcha::services::arkose_labs::ArkoseLabsTaskType;
use typed_builder::TypedBuilder;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize)]
pub struct Blob<'a> {
    pub blob: Cow<'a, str>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct ArkoseLabsTwoCaptcha<'a> {
    pub task_type: Option<ArkoseLabsTaskType<'a>>,
    pub user_agent: Option<Cow<'a, str>>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, TypedBuilder)]
pub struct ArkoseLabs<'a> {
    pub blob: Cow<'a, str>,
    pub website_url: Cow<'a, str>,
    pub website_public_key: Cow<'a, str>,
    #[builder(default, setter(strip_option))]
    pub subdomain: Option<Cow<'a, str>>,
}
