#[cfg(test)]
mod tests {
    use service_layer_rs::util::{FnService, MapRequestLayer, MapResponseLayer};
    use service_layer_rs::{Service, ServiceBuilder};
    use std::convert::Infallible;

    #[tokio::test]
    async fn builder() {
        let svc = FnService::new(|request: String| async move {
            Ok::<_, Infallible>(request + "-base")
        });
        let svc = ServiceBuilder::service(svc)
            .layer(MapResponseLayer::new(|result: Result<_, _>| async move {
                result.map(|s| s + "-after1")
            }))
            .layer(MapResponseLayer::new(|result: Result<_, _>| async move {
                result.map(|s| s + "-after2")
            }))
            .layer(MapRequestLayer::new(|request: String| async move {
                Ok(request + "-before")
            }));
        let svc = svc.build();
        let resp = svc.call("start".to_owned()).await.unwrap();
        assert_eq!(resp, "start-before-base-after1-after2")
    }
}
