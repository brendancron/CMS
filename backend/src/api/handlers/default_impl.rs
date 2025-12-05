use async_trait::async_trait;
use openapi::apis::default::{Default, HelloNameGetResponse};
use openapi::apis::ErrorHandler;
use openapi::models::HelloNameGetPathParams;
use axum_extra::extract::{CookieJar, Host};

pub struct DefaultImpl;

#[async_trait]
impl<E> ErrorHandler<E> for DefaultImpl
where
    E: std::fmt::Debug + Send + Sync + 'static,
{
}

#[async_trait]
impl<E> Default<E> for DefaultImpl
where
    E: std::fmt::Debug + Send + Sync + 'static,
{
    async fn hello_name_get(
        &self,
        _method: &http::Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &HelloNameGetPathParams,
    ) -> Result<HelloNameGetResponse, E> {
        Ok(HelloNameGetResponse::Status200_Greeting(
            format!("Hello, {}", path_params.name),
        ))
    }
}

