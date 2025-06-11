use api_builder::{Query, ReqwestClient, RestClient, api_rest_client, error::APIError};
use reqwest::Proxy;
use twocaptcha::{
    client::{TwoCaptchaClient, TwoCaptchaError},
    endpoints::{GetBalancePayload, GetBalanceResponse},
    response::TwoCaptchaResponse,
};

#[derive(ReqwestClient)]
struct Client {
    client: reqwest::blocking::Client,
}
#[api_rest_client(error = TwoCaptchaError, base = "\"https://api.2captcha.com/\"")]
impl RestClient for Client {}

fn main() -> Result<(), APIError<TwoCaptchaError>> {
    env_logger::init();
    let client_key = std::env::var("TWOCAPTCHA_CLIENT_KEY").expect("no 2captcha client key in env");
    let twocaptcha_client = TwoCaptchaClient::builder()
        .client(Client {
            client: reqwest::blocking::Client::builder()
                // fiddler for testing
                .proxy(Proxy::all("http://localhost:8888")?)
                .build()?,
        })
        .client_key(client_key)
        .build();

    let result: TwoCaptchaResponse<GetBalanceResponse> = GetBalancePayload::builder()
        .client_key(twocaptcha_client.client_key())
        .build()
        .query(&twocaptcha_client)?;
    let result = result.to_api_result()?;

    println!("Your balance is: ${}", result.balance);

    Ok(())
}
