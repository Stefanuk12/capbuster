use api_builder::error::APIError;
use serde::Deserialize;

use crate::client::TwoCaptchaError;

/// [Error Reference](https://2captcha.com/api-docs/get-task-result#task-could-not-be-completed).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TwoCaptchaResponse<T> {
    Success(T),
    Error(TwoCaptchaError),
}
impl<'de, T> Deserialize<'de> for TwoCaptchaResponse<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Helper<U> {
            #[serde(default)]
            error_id: i32,
            #[serde(flatten)]
            data: Option<U>,
        }
        let helper = Helper::<T>::deserialize(deserializer)?;

        let error = TwoCaptchaError::from_code(helper.error_id);
        if let TwoCaptchaError::Unknown = error {
            match helper.data {
                Some(data) => Ok(TwoCaptchaResponse::Success(data)),
                None => Err(serde::de::Error::missing_field("data")),
            }
        } else {
            Ok(TwoCaptchaResponse::Error(error))
        }
    }
}

impl<T> TwoCaptchaResponse<T> {
    pub fn to_result(self) -> Result<T, TwoCaptchaError> {
        match self {
            TwoCaptchaResponse::Success(x) => Ok(x),
            TwoCaptchaResponse::Error(x) => Err(x),
        }
    }

    pub fn to_api_result(self) -> Result<T, APIError<TwoCaptchaError>> {
        self.to_result().map_err(APIError::Client)
    }
}
