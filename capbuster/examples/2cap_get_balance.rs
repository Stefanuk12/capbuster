use api_builder::{ReqwestClient, RestClient, error::APIError};
use capbuster::{error::CaptchaError, providers::all::SolverProvider};
use reqwest::Proxy;

// Specify your own HTTP client and how it works.
// NOTE: There is a helper trait for `reqwest` clients. Other clients require more boilerplate
struct TwoCaptchaClient(reqwest::blocking::Client);
impl RestClient for TwoCaptchaClient {
    type Error = CaptchaError;
    fn rest_endpoint(&self, path: &str) -> Result<api_builder::Url, APIError<Self::Error>> {
        // The API base can be anything.
        // This means that you can use any API that has the exact same API as 2captcha instead,
        // so you're not locked into using specific API bases.
        let url = api_builder::Url::parse("https://api.2captcha.com/").unwrap();
        url.join(path).map_err(APIError::URL)
    }
}
impl ReqwestClient for TwoCaptchaClient {
    fn client(&self) -> &reqwest::blocking::Client {
        &self.0
    }
}

fn main() -> Result<(), APIError<CaptchaError>> {
    env_logger::init();
    let client_key = std::env::var("TWOCAPTCHA_CLIENT_KEY").expect("no 2captcha client key in env");

    // Declares a storage bucket for every provider,
    // and automatically adds the associated functions for various operations like
    // - balance
    // - solving
    let client = SolverProvider::new().twocaptcha(
        client_key,
        TwoCaptchaClient(
            reqwest::blocking::Client::builder()
                // fiddler for testing
                .proxy(Proxy::all("http://localhost:8888")?)
                .build()?,
        ),
    );

    println!("balance: ${}", client.balance_twocaptcha()?);

    Ok(())
}
