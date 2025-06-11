use api_builder::{
    Client, RestClient,
    error::{APIClientError, APIError},
};
use bytes::Bytes;
use http::Response;
use serde::Deserialize;
use typed_builder::TypedBuilder;
use url::Url;

/// [Reference](https://2captcha.com/api-docs/error-codes#api-error-codes-reference).
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, thiserror::Error, Deserialize,
)]
pub enum TwoCaptchaError {
    #[error("Unknown error")]
    Unknown = 0,
    #[error(
        "Your API key is incorrect. Make sure you set the key correctly and copied it from the dashboard in Customer or Developer mode"
    )]
    KeyDoesNotExist = 1,
    #[error(
        "Your bid is too low for the captcha you submit or the queue of your captchas is loo long and we temporary do not accept more captchas from you"
    )]
    NoSlotAvailable = 2,
    #[error("Image size is less than 100 bytes")]
    ZeroCaptchaFilesize = 3,
    #[error("Image size is more than 100 kB or image is bigger than 600px on any side")]
    TooBigCaptchaFilesize = 4,
    #[error(
        "The value of websiteURL parameter is missing or has incorrect format. Set it to the page url value"
    )]
    PageUrl = 5,
    #[error("You don't have funds on your account")]
    ZeroBalance = 10,
    #[error("The request is sent from the IP that is not on the list of your trusted IPs")]
    IpNotAllowed = 11,
    #[error(
        "We are unable to solve your captcha - three of our workers were unable solve it. The captcha price is automatically returned to your balance"
    )]
    CaptchaUnsolvable = 12,
    #[error(
        "The error is returned when 100% accuracy feature is enabled. The error means that max numbers of tries is reached but min number of matches not found"
    )]
    BadDuplicates = 13,
    #[error("Request made to API with a method that does not exist")]
    NoSuchMethod = 14,
    #[error(
        "The image can not be processed due to an incorrect format or size, or the image is corrupted. Please check the image in your request payload"
    )]
    ImageTypeNotSupported = 15,
    #[error("You've provided incorrect captcha ID in the request")]
    NoSuchCapchaId = 16,
    #[error("Your IP address is banned due to improper use of the API")]
    IpBlocked = 21,
    #[error("task property is missing in your createTask method call")]
    TaskAbsent = 22,
    #[error(
        "task property in your createTask method call contains the type of task that is not supported by our API or you have an error in type property"
    )]
    TaskNotSupported = 23,
    #[error("The sitekey value provided in your request is not valid")]
    RecaptchaInvalidSitekey = 31,
    #[error(
        "Your API access was blocked for improper use of the API. Please contact our support team to resolve the issue"
    )]
    AccountSuspended = 55,
    #[error(
        "The required captcha parameters in your reques are missing or have incorrect format. Please make sure your request payload has proper format for selected task type"
    )]
    BadParameters = 110,
    #[error(
        "The error is returned in cases when imgInstructions contains unsupported file type, corrupted file or the size of the image is over the limits. The limits are described in the corresponding task type specification."
    )]
    BadImgInstructions = 115,
    #[error("Incorrect proxy parameters or can not establish connection through the proxy")]
    BadProxy = 130,
}
impl APIClientError for TwoCaptchaError {}

impl TwoCaptchaError {
    pub fn from_code(code: i32) -> Self {
        match code {
            0 => Self::Unknown,
            1 => Self::KeyDoesNotExist,
            2 => Self::NoSlotAvailable,
            3 => Self::ZeroCaptchaFilesize,
            4 => Self::TooBigCaptchaFilesize,
            5 => Self::PageUrl,
            10 => Self::ZeroBalance,
            11 => Self::IpNotAllowed,
            12 => Self::CaptchaUnsolvable,
            13 => Self::BadDuplicates,
            14 => Self::NoSuchMethod,
            15 => Self::ImageTypeNotSupported,
            16 => Self::NoSuchCapchaId,
            21 => Self::IpBlocked,
            22 => Self::TaskAbsent,
            23 => Self::TaskNotSupported,
            31 => Self::RecaptchaInvalidSitekey,
            55 => Self::AccountSuspended,
            110 => Self::BadParameters,
            115 => Self::BadImgInstructions,
            130 => Self::BadProxy,
            _ => Self::Unknown,
        }
    }
}

#[derive(TypedBuilder)]
pub struct TwoCaptchaClient<T> {
    client: T,
    client_key: String,
    #[builder(default, setter(strip_option))]
    base_url: Option<Url>,
}

impl<T> TwoCaptchaClient<T> {
    pub fn client_key(&self) -> &str {
        &self.client_key
    }
}

impl<T: RestClient> RestClient for TwoCaptchaClient<T> {
    type Error = <T as RestClient>::Error;

    fn rest_endpoint(&self, path: &str) -> Result<Url, APIError<Self::Error>> {
        if let Some(base_url) = &self.base_url {
            base_url.join(path).map_err(APIError::URL)
        } else {
            self.client.rest_endpoint(path)
        }
    }
}

impl<T: Client> Client for TwoCaptchaClient<T> {
    fn rest(
        &self,
        request: http::Request<Vec<u8>>,
    ) -> Result<Response<Bytes>, APIError<Self::Error>> {
        self.client.rest(request)
    }
}
