use crate::service::Service;
use std::future::Future;
use std::marker::PhantomData;

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
