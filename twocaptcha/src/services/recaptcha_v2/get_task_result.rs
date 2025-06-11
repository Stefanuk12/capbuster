use serde::Deserialize;

/// [Reference](https://2captcha.com/api-docs/arkoselabs-funcaptcha#response-example).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaV2Solution {
    pub g_recaptcha_response: String,
    pub token: String,
}
