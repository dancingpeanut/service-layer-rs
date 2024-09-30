use std::future::Future;

/// # Example
///
/// ```rust
/// use service_layer_rs::{FnService, Layer, Service, ServiceBuilder};
///
/// struct LogMiddle<S> {
///     svc: S,
///     name: String,
/// }
///
/// impl<S, Request, Response> Service<Request, Response> for LogMiddle<S>
/// where
///     S: Service<Request, Response>,
/// {
///     async fn call(&self, req: Request) -> Response {
///         println!("start {} --->", self.name);
///         let resp = self.svc.call(req).await;
///         println!("end   {} <---", self.name);
///         resp
///     }
/// }
///
/// struct LogMiddleLayer {
///     log: String,
/// }
///
/// impl<S> Layer<S, LogMiddle<S>> for LogMiddleLayer {
///     fn layer(self, svc: S) -> LogMiddle<S> {
///         LogMiddle { svc, name: self.log }
///     }
/// }
///
///  let svc = FnService::new(|req: String| async move {
///     format!("hello {}", req);
///  });
///  let svc = ServiceBuilder::new(svc)
///     .layer(LogMiddleLayer { log: "test".into() })
///     .build();
///  svc.call("1".to_string()).await;
/// ```

// #[trait_variant::make(Send)]
#[allow(async_fn_in_trait)]
pub trait Service<Request, Response> {
    async fn call(&self, request: Request) -> Response;
}

pub trait Layer<S, T> {
    fn layer(self, svc: S) -> T;
}

pub struct ServiceBuilder<S> {
    inner: S,
}

impl<S> ServiceBuilder<S> {
    pub fn new<Request, Response>(inner: S) -> Self
    where
        S: Service<Request, Response>,
    {
        ServiceBuilder { inner }
    }

    pub fn layer<Request, Response, L, T>(self, l: L) -> ServiceBuilder<T>
    where
        S: Service<Request, Response>,
        T: Service<Request, Response>,
        L: Layer<S, T>,
    {
        let inner = l.layer(self.inner);
        ServiceBuilder { inner }
    }

    pub fn build<Request, Response>(self) -> S
    where
        S: Service<Request, Response>,
    {
        self.inner
    }

    #[allow(dead_code)]
    pub async fn call<Request, Response>(self, request: Request) -> Response
    where
        S: Service<Request, Response>,
    {
        self.inner.call(request).await
    }
}

pub struct FnService<F>(F);

impl<F> FnService<F> {
    pub fn new(f: F) -> Self {
        FnService(f)
    }
}

impl<Request, Response, F, Fut> Service<Request, Response> for FnService<F>
where
    F: Fn(Request) -> Fut,
    Fut: Future<Output = Response>,
{
    async fn call(&self, req: Request) -> Response {
        self.0(req).await
    }
}
