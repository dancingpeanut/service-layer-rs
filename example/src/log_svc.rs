use service_layer_rs::{Layer, Service};

pub struct LogService<S> {
    inner: S,
    name: String,
}

impl<S, Request> Service<Request> for LogService<S>
where
    S: Service<Request>,
    Request: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;

    async fn call(
        &self,
        request: Request,
    ) -> Result<Self::Response, Self::Error> {
        println!("LogService<{}> start", self.name);
        let res = self.inner.call(request).await;
        println!("LogService<{}> end", self.name);
        res
    }
}

pub struct LogLayer(pub String);

impl<S> Layer<S> for LogLayer
where
    S: Send + Sync + 'static
{
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService { inner, name: self.0 }
    }
}

pub struct AppStrService<S> {
    inner: S,
    name: String,
}

impl<S> Service<String> for AppStrService<S>
where
    S: Service<String, Response=String>,
{
    type Response = S::Response;
    type Error = S::Error;

    async fn call(
        &self,
        request: String,
    ) -> Result<Self::Response, Self::Error> {
        let res = self.inner.call(request).await?;
        Ok(format!("{}:{}", res, self.name))
    }
}

pub struct AppStrLayer(pub String);

impl<S> Layer<S> for AppStrLayer {
    type Service = AppStrService<S>;

    fn layer(self, inner: S) -> Self::Service {
        AppStrService { inner, name: self.0 }
    }
}
