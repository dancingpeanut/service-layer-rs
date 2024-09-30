use std::time::Duration;
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


////////////////////////////////////////////////////////////////////////////////////////////////////


struct Timeout<S> {
    svc: S,
    sec: u64,
}
impl<S, Request, Response> Service<Request, Response> for Timeout<S>
where
    S: Service<Request, Response>,
{
    async fn call(&self, req: Request) -> Response {
        println!("timeout{} sec", self.sec);
        if let Ok(resp) = tokio::time::timeout(Duration::from_secs(self.sec), self.svc.call(req)).await {
            println!("timeout end");
            return resp;
        } else {
            panic!("timeout")
        }
    }
}
struct TimeoutLayer {
    sec: u64,
}
impl<S> Layer<S, Timeout<S>> for TimeoutLayer {
    fn layer(self, svc: S) -> Timeout<S> {
        Timeout { svc, sec: self.sec }
    }
}

async fn hello(_: &str) -> Result<&'static str, std::io::Error> {
    println!("hello");
    Ok("hello")
}

#[tokio::main]
async fn main() {
    let svc = FnService::new(hello);
    let svc = ServiceBuilder::new(svc)
        .layer(LogMiddleLayer { log: "log middle".into() })
        .layer(TimeoutLayer { sec: 2 })
        .build();
    let resp = svc.call("1").await;
    println!("{:?}", resp);

    let svc = FnService::new(|req: String| async move {
        format!("hello {}", req)
    });
    let svc = ServiceBuilder::new(svc)
        .layer(LogMiddleLayer { log: "test".into() })
        .build();
    svc.call("1".to_string()).await;
}
