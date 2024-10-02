mod test_service;
mod test_service_fn;
mod test_layer;
mod log_svc;

use service_layer_rs::service_fn::FnService;
use service_layer_rs::{add_layer, Service, ServiceBuilder};
use std::convert::Infallible;
use crate::log_svc::LogLayer;

#[tokio::main]
async fn main() {
    let svc = FnService::new(|req: String| async move {
        println!("handle: {}", req);
        Ok::<_, Infallible>(req)
    });

    let svc = ServiceBuilder::new(svc)
        .layer(LogLayer("Test".to_string()))
        .build();

    let svc = ServiceBuilder::new(svc);
    let svc = add_layer(svc, LogLayer("Test2".to_string()));
    let svc = svc.build();

    let ss = svc.boxed();
    let res: Result<String, Infallible> = ss.call("hello".to_owned()).await;
    println!("{:?}", res);
}
