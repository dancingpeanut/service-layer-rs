

#[cfg(test)]
mod tests {
    use service_layer_rs::{Layer, Service, ServiceBuilder};
    use std::convert::Infallible;
    use service_layer_rs::service_fn::FnService;
    use crate::log_svc::AppStrLayer;

    #[tokio::test]
    async fn builder() {
        let svc = FnService::new(|req: String| async move { Ok::<_, Infallible>(req) });
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
        let svc = FnService::new(|req: String| async move { Ok::<_, Infallible>(req) }).boxed();
        let svc = AppStrLayer("1".to_string()).layer(svc).boxed();
        let svc = AppStrLayer("2".to_string()).layer(svc).boxed();
        let svc = AppStrLayer("3".to_string()).layer(svc).boxed();
        let resp = svc.call("-".to_owned()).await.unwrap();
        assert_eq!("-:1:2:3", resp)
    }
}
