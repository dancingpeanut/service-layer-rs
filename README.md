# service-layer-rs

A simple alternative to the tower service layer, implemented using async trait, making the code more concise and easier to use.

## Example
[example/src/main.rs](example/src/main.rs)

```rust
use service_layer_rs::{FnService, Layer, Service, ServiceBuilder};

struct LogMiddle<S> {
    svc: S,
    name: String,
}

impl<S, Request, Response> Service<Request, Response> for LogMiddle<S>
where
    S: Service<Request, Response>,
{
    async fn call(&self, req: Request) -> Response {
        println!("start {} --->", self.name);
        let resp = self.svc.call(req).await;
        println!("end   {} <---", self.name);
        resp
    }
}

struct LogMiddleLayer {
    log: String,
}

impl<S> Layer<S, LogMiddle<S>> for LogMiddleLayer {
    fn layer(self, svc: S) -> LogMiddle<S> {
        LogMiddle { svc, name: self.log }
    }
}

#[tokio::main]
async fn main() {
    let svc = FnService::new(|req: String| async move {
        format!("hello {}", req);
    });
    let svc = ServiceBuilder::new(svc)
        .layer(LogMiddleLayer { log: "test".into() })
        .build();
    svc.call("1".to_string()).await;
}
```
