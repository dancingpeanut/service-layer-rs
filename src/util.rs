use crate::service::Service;
use std::future::Future;
use std::marker::PhantomData;
use crate::Layer;

pub struct FnService<F, Request, R, Response, E>
where
    F: Fn(Request) -> R + Send + Sync + 'static,
    R: Future<Output=Result<Response, E>>,
{
    f: F,
    _t: PhantomData<fn(Request, R, Response) -> ()>,
}

impl<F, Request, R, Response, E> FnService<F, Request, R, Response, E>
where
    F: Fn(Request) -> R + Send + Sync + 'static,
    R: Future<Output=Result<Response, E>>,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _t: PhantomData,
        }
    }
}

impl<F, Request, R, Response, E> Clone for FnService<F, Request, R, Response, E>
where
    F: Fn(Request) -> R + Send + Sync + 'static + Clone,
    R: Future<Output=Result<Response, E>>,
{
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _t: PhantomData,
        }
    }
}

impl<F, Request, R, Response, E> Service<Request> for FnService<F, Request, R, Response, E>
where
    F: Fn(Request) -> R + Send + Sync + 'static,
    R: Future<Output=Result<Response, E>> + Send + 'static,
    Response: Send + 'static,
    E: Send + Sync + 'static,
    Request: 'static,
{
    type Response = Response;
    type Error = E;

    fn call(
        &self,
        request: Request,
    ) -> impl Future<Output=Result<Self::Response, Self::Error>> + Send + '_ {
        (self.f)(request)
    }
}


#[derive(Clone)]
pub struct MapResponse<S, F> {
    inner: S,
    f: F,
}


impl<S, F, Request, R> Service<Request> for MapResponse<S, F>
where
    S: Service<Request> + Clone + 'static,
    Request: Send + 'static,
    F: FnOnce(Result<S::Response, S::Error>) -> R + Clone + Sync + Send + 'static,
    R: Future<Output=Result<S::Response, S::Error>> + Send + 'static + Send,
{
    type Response = S::Response;
    type Error = S::Error;

    async fn call(
        &self,
        request: Request,
    ) -> Result<Self::Response, Self::Error> {
        let resp = self.inner.call(request).await;
        self.f.clone()(resp).await
    }
}

/// Layer that maps the response of a service.
#[derive(Clone)]
pub struct MapResponseLayer<F> {
    f: F,
}

impl<F> MapResponseLayer<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<S, F> Layer<S> for MapResponseLayer<F>
where
    F: Clone,
    S: Send + Sync + 'static
{
    type Service = MapResponse<S, F>;

    fn layer(self, inner: S) -> Self::Service {
        MapResponse {
            f: self.f,
            inner,
        }
    }
}

/// Layer that maps the request of a service.
#[derive(Clone)]
pub struct MapRequest<S, F> {
    inner: S,
    f: F,
}


impl<S, F, Request, R> Service<Request> for MapRequest<S, F>
where
    S: Service<Request> + Clone + 'static,
    Request: Send + 'static,
    F: FnOnce(Request) -> R + Clone + Sync + Send + 'static,
    R: Future<Output=Result<Request, S::Error>> + Send + 'static + Send,
{
    type Response = S::Response;
    type Error = S::Error;

    async fn call(
        &self,
        request: Request,
    ) -> Result<Self::Response, Self::Error> {
        let request = self.f.clone()(request).await?;
        self.inner.call(request).await
    }
}


#[derive(Clone)]
pub struct MapRequestLayer<F> {
    f: F,
}

impl<F> MapRequestLayer<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<S, F> Layer<S> for MapRequestLayer<F>
where
    F: Clone,
    S: Send + Sync + 'static
{
    type Service = MapRequest<S, F>;

    fn layer(self, inner: S) -> Self::Service {
        MapRequest {
            f: self.f,
            inner,
        }
    }
}
