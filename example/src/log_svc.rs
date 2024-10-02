use service_layer_rs::{Layer, Service};

pub struct LogService<S> {
    svc: S,
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
        req: Request,
    ) -> Result<Self::Response, Self::Error> {
        println!("LogService<{}> start", self.name);
        let res = self.svc.call(req).await;
        println!("LogService<{}> end", self.name);
        res
    }
}

pub struct LogLayer(pub String);

impl<S: Send + Sync + 'static> Layer<S, LogService<S>> for LogLayer {
    fn layer(self, svc: S) -> LogService<S> {
        LogService { svc, name: self.0 }
    }
}

pub struct AppStrService<S> {
    svc: S,
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
        req: String,
    ) -> Result<Self::Response, Self::Error> {
        let res = self.svc.call(req).await?;
        Ok(format!("{}:{}", res, self.name))
    }
}

pub struct AppStrLayer(pub String);

impl<S: Send + Sync + 'static> Layer<S, AppStrService<S>> for AppStrLayer {
    fn layer(self, svc: S) -> AppStrService<S> {
        AppStrService { svc, name: self.0 }
    }
}
