//! Service Layer
//!

//! # Example
//!
//! ```rust
//! use service_layer_rs::util::FnService;
//! use service_layer_rs::{Layer, Service, ServiceBuilder};
//! use std::convert::Infallible;
//!
//! pub struct LogService<S> {
//!     inner: S,
//!     name: String,
//! }
//!
//! impl<S, Request> Service<Request> for LogService<S>
//! where
//!     S: Service<Request>,
//!     Request: Send + 'static,
//! {
//!     type Response = S::Response;
//!     type Error = S::Error;
//!
//!     async fn call(
//!         &self,
//!         request: Request,
//!     ) -> Result<Self::Response, Self::Error> {
//!         println!("LogService<{}> start", self.name);
//!         let res = self.inner.call(request).await;
//!         println!("LogService<{}> end", self.name);
//!         res
//!     }
//! }
//!
//! pub struct LogLayer(pub String);
//!
//! impl<S> Layer<S> for LogLayer
//! where
//!     S: Send + Sync + 'static
//! {
//!     type Service = LogService<S>;
//!
//!     fn layer(self, inner: S) -> Self::Service {
//!         LogService { inner, name: self.0 }
//!     }
//! }
//!
//! async fn main() {
//!     let svc = FnService::new(|request: String| async move {
//!         println!("handle: {}", request);
//!         Ok::<_, Infallible>(request)
//!     });
//!
//!     let svc = ServiceBuilder::service(svc)
//!         .layer(LogLayer("Test".to_string()))
//!         .build();
//!
//!     let ss = svc.boxed();
//!     let res: Result<String, Infallible> = ss.call("hello".to_owned()).await;
//!     println!("{:?}", res);
//! }
//! ```

pub mod service;
pub mod util;

pub use service::{BoxService, Service};

/// Decorates a Service
pub trait Layer<S> {
    /// The wrapped service
    type Service;
    /// Wrap the given service with the middleware, returning a new service that has been decorated with the middleware.
    fn layer(self, inner: S) -> Self::Service;
}

/// Builder types to compose layers and services
pub struct ServiceBuilder<S> {
    inner: S,
}

impl<S> ServiceBuilder<S> {
    pub fn service<Request>(inner: S) -> Self
    where
        S: Service<Request>,
    {
        ServiceBuilder { inner }
    }

    pub fn layer<L, T, Request>(self, l: L) -> ServiceBuilder<T>
    where
        S: Service<Request>,
        T: Service<Request>,
        L: Layer<S, Service=T>,
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
