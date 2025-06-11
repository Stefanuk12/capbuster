use std::time::Duration;

use api_builder::{AsyncClient, AsyncQuery, Client, Query, RestClient, error::APIError};
use twocaptcha::{
    client::{TwoCaptchaClient, TwoCaptchaError},
    endpoints::{
        CreateTaskData, CreateTaskPayload, GetBalancePayload, GetBalanceResponse,
        GetTaskResultPayload, GetTaskResultResponse, GetTaskResultResponseReady,
    },
    response::TwoCaptchaResponse,
    services::{arkose_labs::ArkoseLabsTask, recaptcha_v2::RecaptchaV2Task},
};

use crate::{
    captchas::{
        arkose_labs::{ArkoseLabs, ArkoseLabsTwoCaptcha, Blob},
        recaptchav2::{RecaptchaV2, RecaptchaV2TwoCaptcha},
    },
    providers::{CaptchaSolver, GetBalance, GetBalanceAsync, PollSolverResult},
};

#[cfg(feature = "tokio")]
use crate::providers::{CaptchaSolverAsync, PollSolverResultAsync};

impl<T> GetBalance for TwoCaptchaClient<T>
where
    T: RestClient,
    <TwoCaptchaClient<T> as RestClient>::Error: From<TwoCaptchaError>,
{
    fn balance(&self) -> Result<f32, APIError<<TwoCaptchaClient<T> as RestClient>::Error>>
    where
        Self: Client,
    {
        let result: TwoCaptchaResponse<GetBalanceResponse> = GetBalancePayload::builder()
            .client_key(self.client_key())
            .build()
            .query(self)?;
        match result {
            TwoCaptchaResponse::Success(x) => Ok(x.balance),
            TwoCaptchaResponse::Error(err) => Err(APIError::Client(err.into())),
        }
    }
}

impl<T> GetBalanceAsync for TwoCaptchaClient<T>
where
    T: RestClient + Sync,
    <TwoCaptchaClient<T> as RestClient>::Error: From<TwoCaptchaError>,
{
    async fn balance_async(
        &self,
    ) -> Result<f32, APIError<<TwoCaptchaClient<T> as RestClient>::Error>>
    where
        Self: AsyncClient,
    {
        let result: TwoCaptchaResponse<GetBalanceResponse> = GetBalancePayload::builder()
            .client_key(self.client_key())
            .build()
            .query_async(self)
            .await?;
        match result {
            TwoCaptchaResponse::Success(x) => Ok(x.balance),
            TwoCaptchaResponse::Error(err) => Err(APIError::Client(err.into())),
        }
    }
}

impl<T> PollSolverResult for TwoCaptchaClient<T>
where
    T: RestClient,
    <TwoCaptchaClient<T> as RestClient>::Error: From<TwoCaptchaError>,
{
    fn poll(
        &self,
        task_id: u64,
    ) -> Result<GetTaskResultResponseReady, APIError<<Self as RestClient>::Error>>
    where
        Self: Client,
    {
        let payload = GetTaskResultPayload::builder()
            .client_key(self.client_key())
            .task_id(task_id)
            .build();

        loop {
            let result: TwoCaptchaResponse<GetTaskResultResponse> = payload.query(self)?;
            match result {
                TwoCaptchaResponse::Success(GetTaskResultResponse::Ready(x)) => return Ok(x),
                TwoCaptchaResponse::Success(GetTaskResultResponse::Processing) => {
                    std::thread::sleep(Duration::from_secs(10))
                }
                TwoCaptchaResponse::Error(err) => return Err(APIError::Client(err.into())),
            };
        }
    }
}

#[cfg(feature = "tokio")]
impl<T> PollSolverResultAsync for TwoCaptchaClient<T>
where
    T: RestClient + Sync,
    <TwoCaptchaClient<T> as RestClient>::Error: From<TwoCaptchaError>,
{
    async fn poll_async(
        &self,
        task_id: u64,
    ) -> Result<GetTaskResultResponseReady, APIError<<Self as RestClient>::Error>>
    where
        Self: AsyncClient,
    {
        let payload = GetTaskResultPayload::builder()
            .client_key(self.client_key())
            .task_id(task_id)
            .build();

        loop {
            let result: TwoCaptchaResponse<GetTaskResultResponse> =
                payload.query_async(self).await?;
            match result {
                TwoCaptchaResponse::Success(GetTaskResultResponse::Ready(x)) => return Ok(x),
                TwoCaptchaResponse::Success(GetTaskResultResponse::Processing) => {
                    tokio::time::sleep(Duration::from_secs(10)).await
                }
                TwoCaptchaResponse::Error(err) => return Err(APIError::Client(err.into())),
            };
        }
    }
}

fn arkose_labs_to_payload<'a, T>(
    client: &'a TwoCaptchaClient<T>,
    captcha: ArkoseLabs<'a>,
    data: ArkoseLabsTwoCaptcha<'a>,
) -> CreateTaskPayload<'a, Blob<'a>> {
    let task = CreateTaskData::ArkoseLabs(
        ArkoseLabsTask::builder()
            .data(Some(Blob { blob: captcha.blob }))
            .website_url(captcha.website_url)
            .website_public_key(captcha.website_public_key)
            .subdomain(captcha.subdomain)
            .user_agent(data.user_agent)
            .task_type(data.task_type.unwrap_or_default())
            .build(),
    );

    let payload = CreateTaskPayload::<'a, Blob<'a>>::builder()
        .client_key(client.client_key())
        .task(task)
        .build();
    payload
}

impl<'a, T> CaptchaSolver<ArkoseLabs<'a>> for TwoCaptchaClient<T>
where
    TwoCaptchaClient<T>: RestClient,
    <TwoCaptchaClient<T> as RestClient>::Error: From<TwoCaptchaError>,
    T: RestClient,
{
    type ExtraData = ArkoseLabsTwoCaptcha<'a>;
    type Solution = GetTaskResultResponseReady;

    fn solve(
        &self,
        captcha: ArkoseLabs<'_>,
        data: Self::ExtraData,
    ) -> Result<Self::Solution, APIError<<Self as RestClient>::Error>>
    where
        Self: Client,
    {
        let result: TwoCaptchaResponse<twocaptcha::endpoints::CreateTaskResponse> =
            arkose_labs_to_payload(self, captcha, data).query(self)?;

        match result {
            TwoCaptchaResponse::Success(x) => Ok(self.poll(x.task_id)?),
            TwoCaptchaResponse::Error(e) => Err(APIError::Client(e.into())),
        }
    }
}

#[cfg(feature = "tokio")]
impl<'a, T> CaptchaSolverAsync<ArkoseLabs<'a>> for TwoCaptchaClient<T>
where
    TwoCaptchaClient<T>: RestClient,
    <TwoCaptchaClient<T> as RestClient>::Error: From<TwoCaptchaError>,
    T: RestClient + Sync,
{
    type ExtraData = ArkoseLabsTwoCaptcha<'a>;
    type Solution = GetTaskResultResponseReady;

    async fn solve_async(
        &self,
        captcha: ArkoseLabs<'a>,
        data: Self::ExtraData,
    ) -> Result<Self::Solution, APIError<<Self as RestClient>::Error>>
    where
        Self: AsyncClient,
    {
        let result: TwoCaptchaResponse<twocaptcha::endpoints::CreateTaskResponse> =
            arkose_labs_to_payload(self, captcha, data)
                .query_async(self)
                .await?;

        match result {
            TwoCaptchaResponse::Success(x) => Ok(self.poll_async(x.task_id).await?),
            TwoCaptchaResponse::Error(e) => Err(APIError::Client(e.into())),
        }
    }
}

fn recaptcha_v2_to_payload<'a, T>(
    client: &'a TwoCaptchaClient<T>,
    captcha: RecaptchaV2<'a>,
    data: RecaptchaV2TwoCaptcha<'a>,
) -> CreateTaskPayload<'a, ()> {
    let task: CreateTaskData<'a, ()> = CreateTaskData::RecaptchaV2(
        RecaptchaV2Task::builder()
            .api_domain(captcha.api_domain)
            .cookies(data.cookies)
            .is_invisible(captcha.is_invisible)
            .recaptcha_data_s_value(captcha.recaptcha_data_s_value)
            .task_type(data.task_type.unwrap_or_default())
            .user_agent(data.user_agent)
            .website_key(captcha.website_key)
            .website_url(captcha.website_url)
            .build(),
    );

    let payload = CreateTaskPayload::<'a, ()>::builder()
        .client_key(client.client_key())
        .task(task)
        .build();
    payload
}

impl<'a, T> CaptchaSolver<RecaptchaV2<'a>> for TwoCaptchaClient<T>
where
    TwoCaptchaClient<T>: RestClient,
    <TwoCaptchaClient<T> as RestClient>::Error: From<TwoCaptchaError>,
    T: RestClient + Sync,
{
    type ExtraData = RecaptchaV2TwoCaptcha<'a>;
    type Solution = GetTaskResultResponseReady;

    fn solve(
        &self,
        captcha: RecaptchaV2<'a>,
        data: Self::ExtraData,
    ) -> Result<Self::Solution, APIError<<Self as RestClient>::Error>>
    where
        Self: Client,
    {
        let result: TwoCaptchaResponse<twocaptcha::endpoints::CreateTaskResponse> =
            recaptcha_v2_to_payload(self, captcha, data)
                .query(self)
                .map_err(APIError::from_api_error)?;

        match result {
            TwoCaptchaResponse::Success(x) => Ok(self.poll(x.task_id)?),
            TwoCaptchaResponse::Error(e) => Err(APIError::Client(e.into())),
        }
    }
}

#[cfg(feature = "tokio")]
impl<'a, T> CaptchaSolverAsync<RecaptchaV2<'a>> for TwoCaptchaClient<T>
where
    TwoCaptchaClient<T>: RestClient,
    <TwoCaptchaClient<T> as RestClient>::Error: From<TwoCaptchaError>,
    T: RestClient + Sync,
{
    type ExtraData = RecaptchaV2TwoCaptcha<'a>;
    type Solution = GetTaskResultResponseReady;

    async fn solve_async(
        &self,
        captcha: RecaptchaV2<'a>,
        data: Self::ExtraData,
    ) -> Result<Self::Solution, APIError<<Self as RestClient>::Error>>
    where
        Self: AsyncClient,
    {
        let result: TwoCaptchaResponse<twocaptcha::endpoints::CreateTaskResponse> =
            recaptcha_v2_to_payload(self, captcha, data)
                .query_async(self)
                .await
                .map_err(APIError::from_api_error)?;

        match result {
            TwoCaptchaResponse::Success(x) => Ok(self.poll_async(x.task_id).await?),
            TwoCaptchaResponse::Error(e) => Err(APIError::Client(e.into())),
        }
    }
}
