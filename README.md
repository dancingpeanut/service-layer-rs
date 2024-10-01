# service-layer-rs

A simple alternative to the tower service layer, implemented using async trait, making the code more concise and easier to use.

## Example

```rust
use service_layer_rs::service_fn::FnService;
use service_layer_rs::{Layer, Service, ServiceBuilder};
use std::convert::Infallible;

struct LogService<S> {
    svc: S,
    name: String
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

struct LogLayer(String);

impl<S: Send + Sync + 'static> Layer<S, LogService<S>> for LogLayer {
    fn layer(self, svc: S) -> LogService<S> {
        LogService { svc, name: self.0 }
    }
}

#[tokio::main]
async fn main() {
    let svc = FnService::new(|req: String| async move {
        println!("handle: {}", req);
        Ok::<_, Infallible>(req)
    });

    let svc = ServiceBuilder::new(svc)
        .layer(LogLayer("Test".to_string()))
        .build();

    let res: Result<String, Infallible> = svc.call("hello".to_owned()).await;
    println!("{:?}", res);
}
```

### Dynamic Dispatch
```rust
let svc = FnService::new(|req: String| async move {
    println!("handle: {}", req);
    Ok::<_, Infallible>(req)
});

// Box this service to allow for dynamic dispatch.
let svc = ServiceBuilder::new(svc)
    .layer(LogLayer("Test".to_string()))
    .build()
    .boxed();

let res: Result<String, Infallible> = svc.call("hello".to_owned()).await;
println!("{:?}", res);
```

### ServiceBuilder add layer
```rust
let svc = ServiceBuilder::new(svc);

// by .layer
let svc = svc.layer(LogLayer("Test".to_string()));

// by add_layer
let svc = add_layer(svc, LogLayer("Test".to_string()));
```
