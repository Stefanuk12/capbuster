use serde::Deserialize;

/// [Reference](https://2captcha.com/api-docs/arkoselabs-funcaptcha#response-example).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
pub struct ArkoseLabsSolution {
    pub token: String,
}
