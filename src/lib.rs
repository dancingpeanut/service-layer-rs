pub mod service;
pub mod service_fn;

pub use service::{Service, BoxService};

pub trait Layer<S, T> {
    fn layer(self, svc: S) -> T;
}

pub struct ServiceBuilder<S> {
    inner: S,
}

impl<S> ServiceBuilder<S> {
    pub fn new<Request>(inner: S) -> Self
    where
        S: Service<Request>,
    {
        ServiceBuilder { inner }
    }

    pub fn layer<Request, L, T>(self, l: L) -> ServiceBuilder<T>
    where
        S: Service<Request>,
        T: Service<Request>,
        L: Layer<S, T>,
    {
        let inner = l.layer(self.inner);
        ServiceBuilder { inner }
    }

    pub fn build<Request>(self) -> S
    where
        S: Service<Request>,
    {
        self.inner
    }

    pub async fn call<Request>(self, request: Request) -> Result<S::Response, S::Error>
    where
        S: Service<Request>,
    {
        self.inner.call(request).await
    }
}

pub fn add_layer<Request, L, T, S>(svc_builder: ServiceBuilder<S>, l: L) -> ServiceBuilder<T>
where
    S: Service<Request>,
    T: Service<Request>,
    L: Layer<S, T>,
{
    let inner = l.layer(svc_builder.build());
    ServiceBuilder::new(inner)
}
