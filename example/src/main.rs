mod test_service;
mod test_service_fn;
mod test_layer;
mod log_svc;

use crate::log_svc::LogLayer;
use service_layer_rs::{FnService, Service, ServiceBuilder};
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    let svc = FnService::new(|req: String| async move {
        println!("handle: {}", req);
        Ok::<_, Infallible>(req)
    });

    let svc = ServiceBuilder::service(svc)
        .layer(LogLayer("Test".to_string()))
        .build();

    let ss = svc.boxed();
    let res: Result<String, Infallible> = ss.call("hello".to_owned()).await;
    println!("{:?}", res);
}
