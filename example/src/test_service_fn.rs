#[cfg(test)]
mod tests {
    use service_layer_rs::util::FnService;
    use service_layer_rs::Service;
    use std::convert::Infallible;

    #[tokio::test]
    async fn test_service_fn() {
        let services = vec![
            FnService::new(|_: String| async move { Ok(()) }).boxed(),
            FnService::new(|request: String| async move {
                assert_eq!(request, "hello");
                Ok(())
            })
                .boxed(),
            FnService::new(|request: String| async move {
                assert_eq!(request, "hello");
                Ok(())
            })
                .boxed(),
        ];

        for service in services {
            let request = "hello".to_owned();
            let res: Result<(), Infallible> = service.call(request).await;
            assert!(res.is_ok());
        }
    }

    fn assert_send_sync<T: Send + Sync + 'static>(_t: T) {}

    #[test]
    fn test_service_fn_without_usage() {
        assert_send_sync(FnService::new(|_: ()| async move { Ok::<_, Infallible>(()) }));
        assert_send_sync(FnService::new(
            |_request: String| async move { Ok::<_, Infallible>(()) },
        ));
        assert_send_sync(FnService::new(|_request: String| async move {
            Ok::<_, Infallible>(())
        }));
    }
}
