use api_builder::error::APIClientError;
use twocaptcha::client::TwoCaptchaError;

#[derive(Debug, thiserror::Error)]
pub enum CaptchaError {
    #[error(transparent)]
    TwoCaptcha(#[from] TwoCaptchaError),
}
impl APIClientError for CaptchaError {}
