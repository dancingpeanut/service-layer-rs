#[cfg(test)]
mod tests {
    use service_layer_rs::Service;
    use std::convert::Infallible;

    #[derive(Debug)]
    struct AddSvc(usize);

    impl Service<usize> for AddSvc {
        type Response = usize;
        type Error = Infallible;

        async fn call(
            &self,
            request: usize,
        ) -> Result<Self::Response, Self::Error> {
            Ok(self.0 + request)
        }
    }

    #[derive(Debug)]
    struct MulSvc(usize);

    impl Service<usize> for MulSvc {
        type Response = usize;
        type Error = Infallible;

        async fn call(
            &self,
            request: usize,
        ) -> Result<Self::Response, Self::Error> {
            Ok(self.0 * request)
        }
    }

    #[tokio::test]
    async fn add_svc() {
        let svc = AddSvc(1);


        let response = svc.call(1).await.unwrap();
        assert_eq!(response, 2);
    }

    #[tokio::test]
    async fn static_dispatch() {
        let services = vec![AddSvc(1), AddSvc(2), AddSvc(3)];

        for (i, svc) in services.into_iter().enumerate() {
            let response = svc.call(i).await.unwrap();
            assert_eq!(response, i * 2 + 1);
        }
    }

    #[tokio::test]
    async fn dynamic_dispatch() {
        let services = vec![
            AddSvc(1).boxed(),
            AddSvc(2).boxed(),
            AddSvc(3).boxed(),
            MulSvc(4).boxed(),
            MulSvc(5).boxed(),
        ];

        for (i, svc) in services.into_iter().enumerate() {
            let response = svc.call(i).await.unwrap();
            if i < 3 {
                assert_eq!(response, i * 2 + 1);
            } else {
                assert_eq!(response, i * (i + 1));
            }
        }
    }

    #[tokio::test]
    async fn service_arc() {
        let svc = std::sync::Arc::new(AddSvc(1));

        let response = svc.call(1).await.unwrap();
        assert_eq!(response, 2);
    }

    #[tokio::test]
    async fn box_service_arc() {
        let svc = std::sync::Arc::new(AddSvc(1)).boxed();

        let response = svc.call(1).await.unwrap();
        assert_eq!(response, 2);
    }
}
