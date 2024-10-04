# service-layer-rs

A simple alternative to the tower service layer, implemented using async trait, making the code more concise and easier to use.

## Example

```rust
use service_layer_rs::util::FnService;
use service_layer_rs::{FnService, Layer, Service, ServiceBuilder};
use std::convert::Infallible;

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

#[tokio::main]
async fn main() {
    let svc = FnService::new(|request: String| async move {
        println!("handle: {}", request);
        Ok::<_, Infallible>(request)
    });

    let svc = ServiceBuilder::service(svc)
        .layer(LogLayer("Test".to_string()))
        .build();

    let ss = svc.boxed();
    let res: Result<String, Infallible> = ss.call("hello".to_owned()).await;
    println!("{:?}", res);
}
```

### Dynamic Dispatch
```rust
let svc = FnService::new(|request: String| async move {
    println!("handle: {}", request);
    Ok::<_, Infallible>(request)
});

// Box this service to allow for dynamic dispatch.
let svc = ServiceBuilder::new(svc)
    .layer(LogLayer("Test".to_string()))
    .build()
    .boxed();

let res: Result<String, Infallible> = svc.call("hello".to_owned()).await;
println!("{:?}", res);
```
