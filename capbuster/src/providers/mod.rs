use ::twocaptcha::endpoints::GetTaskResultResponseReady;
use api_builder::{AsyncClient, Client, RestClient, error::APIError};

pub mod all;
pub mod twocaptcha;

pub trait CaptchaSolver<Captcha> {
    type ExtraData;
    type Solution;

    fn solve(
        &self,
        captcha: Captcha,
        data: Self::ExtraData,
    ) -> Result<Self::Solution, APIError<<Self as RestClient>::Error>>
    where
        Self: Client;
}

pub trait CaptchaSolverAsync<Captcha> {
    type ExtraData;
    type Solution;

    fn solve_async(
        &self,
        captcha: Captcha,
        data: Self::ExtraData,
    ) -> impl Future<Output = Result<Self::Solution, APIError<<Self as RestClient>::Error>>> + Send
    where
        Self: AsyncClient;
}

pub trait PollSolverResult {
    fn poll(
        &self,
        task_id: u64,
    ) -> Result<GetTaskResultResponseReady, APIError<<Self as RestClient>::Error>>
    where
        Self: Client;
}

pub trait PollSolverResultAsync {
    fn poll_async(
        &self,
        task_id: u64,
    ) -> impl Future<
        Output = Result<GetTaskResultResponseReady, APIError<<Self as RestClient>::Error>>,
    > + Send
    where
        Self: AsyncClient;
}

pub trait GetBalance {
    fn balance(&self) -> Result<f32, APIError<<Self as RestClient>::Error>>
    where
        Self: Client;
}

pub trait GetBalanceAsync {
    fn balance_async(
        &self,
    ) -> impl Future<Output = Result<f32, APIError<<Self as RestClient>::Error>>> + Send
    where
        Self: AsyncClient;
}
