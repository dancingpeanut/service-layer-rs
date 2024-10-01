#[cfg(test)]
mod tests {
    use service_layer_rs::Service;
    use std::convert::Infallible;
    use service_layer_rs::service_fn::FnService;

    #[tokio::test]
    async fn test_service_fn() {
        let services = vec![
            FnService::new(|_: String| async move { Ok(()) }).boxed(),
            FnService::new(|req: String| async move {
                assert_eq!(req, "hello");
                Ok(())
            })
                .boxed(),
            FnService::new(|req: String| async move {
                assert_eq!(req, "hello");
                Ok(())
            })
                .boxed(),
        ];

        for service in services {
            let req = "hello".to_owned();
            let res: Result<(), Infallible> = service.call(req).await;
            assert!(res.is_ok());
        }
    }

    fn assert_send_sync<T: Send + Sync + 'static>(_t: T) {}

    #[test]
    fn test_service_fn_without_usage() {
        assert_send_sync(FnService::new(|_: ()| async move { Ok::<_, Infallible>(()) }));
        assert_send_sync(FnService::new(
            |_req: String| async move { Ok::<_, Infallible>(()) },
        ));
        assert_send_sync(FnService::new(|_req: String| async move {
            Ok::<_, Infallible>(())
        }));
    }
}
