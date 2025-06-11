use api_builder::{AsyncClient, Client, RestClient, error::APIError};
use twocaptcha::client::TwoCaptchaClient;

use crate::providers::{CaptchaSolver, CaptchaSolverAsync, GetBalance, GetBalanceAsync};

type GetBalanceResult<T> = Result<f32, APIError<<T as RestClient>::Error>>;
type GetBalanceAsyncResult<T> = GetBalanceResult<T>;
type CaptchaSolverResult<T, C> =
    Result<<T as CaptchaSolver<C>>::Solution, APIError<<T as RestClient>::Error>>;
type CaptchaSolverAsyncResult<T, C> =
    Result<<T as CaptchaSolverAsync<C>>::Solution, APIError<<T as RestClient>::Error>>;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SolverProvider<AvailableProviders = ((),)> {
    pub providers: AvailableProviders,
}

impl SolverProvider<((),)> {
    pub fn new() -> SolverProvider<((),)> {
        SolverProvider { providers: ((),) }
    }
}

impl SolverProvider<((),)> {
    pub fn twocaptcha<C>(
        self,
        client_key: String,
        client: C,
    ) -> SolverProvider<(TwoCaptchaClient<C>,)> {
        let client = TwoCaptchaClient::builder()
            .client_key(client_key)
            .client(client)
            .build();
        SolverProvider {
            providers: (client,),
        }
    }
}

impl<T> SolverProvider<(TwoCaptchaClient<T>,)>
where
    TwoCaptchaClient<T>: Client + GetBalance,
{
    pub fn balance_twocaptcha(&self) -> GetBalanceResult<TwoCaptchaClient<T>> {
        self.providers.0.balance()
    }
}
impl<T> SolverProvider<(TwoCaptchaClient<T>,)>
where
    T: Sync,
    TwoCaptchaClient<T>: AsyncClient + GetBalanceAsync,
{
    pub async fn balance_twocaptcha_async(&self) -> GetBalanceAsyncResult<TwoCaptchaClient<T>> {
        self.providers.0.balance_async().await
    }
}
impl<T> SolverProvider<(TwoCaptchaClient<T>,)>
where
    TwoCaptchaClient<T>: Client,
{
    pub fn solve_twocaptcha<C>(
        &self,
        captcha: C,
        data: <TwoCaptchaClient<T> as CaptchaSolver<C>>::ExtraData,
    ) -> CaptchaSolverResult<TwoCaptchaClient<T>, C>
    where
        TwoCaptchaClient<T>: CaptchaSolver<C>,
    {
        self.providers.0.solve(captcha, data)
    }
}
impl<T> SolverProvider<(TwoCaptchaClient<T>,)>
where
    TwoCaptchaClient<T>: AsyncClient,
{
    pub async fn solve_twocaptcha_async<C>(
        &self,
        captcha: C,
        data: <TwoCaptchaClient<T> as CaptchaSolverAsync<C>>::ExtraData,
    ) -> CaptchaSolverAsyncResult<TwoCaptchaClient<T>, C>
    where
        TwoCaptchaClient<T>: CaptchaSolverAsync<C>,
    {
        self.providers.0.solve_async(captcha, data).await
    }
}
