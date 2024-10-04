

#[cfg(test)]
mod tests {
    use crate::log_svc::AppStrLayer;
    use service_layer_rs::util::FnService;
    use service_layer_rs::{Layer, Service, ServiceBuilder};
    use std::convert::Infallible;

    #[tokio::test]
    async fn builder() {
        let svc = FnService::new(|request: String| async move { Ok::<_, Infallible>(request) });
        let svc = ServiceBuilder::service(svc)
            .layer(AppStrLayer("1".to_string()))
            .layer(AppStrLayer("2".to_string()))
            .layer(AppStrLayer("3".to_string()))
            .build();
        let resp = svc.call("-".to_owned()).await.unwrap();
        assert_eq!("-:1:2:3", resp)
    }

    #[tokio::test]
    async fn builder_box() {
        let svc = FnService::new(|request: String| async move { Ok::<_, Infallible>(request) }).boxed();
        let svc = AppStrLayer("1".to_string()).layer(svc).boxed();
        let svc = AppStrLayer("2".to_string()).layer(svc).boxed();
        let svc = AppStrLayer("3".to_string()).layer(svc).boxed();
        let resp = svc.call("-".to_owned()).await.unwrap();
        assert_eq!("-:1:2:3", resp)
    }
}
