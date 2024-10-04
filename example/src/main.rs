mod test_service;
mod test_service_fn;
mod test_layer;
mod log_svc;

use crate::log_svc::LogLayer;
use service_layer_rs::util::FnService;
use service_layer_rs::{Service, ServiceBuilder};
use std::convert::Infallible;

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
