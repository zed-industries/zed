mod helpers;

#[cfg(test)]
mod test {

    use super::helpers;
    use zeromq::__async_rt as async_rt;
    use zeromq::prelude::*;

    use futures::StreamExt;
    use std::error::Error;
    use std::time::Duration;

    #[async_rt::test]
    async fn test_req_router_dealer_rep_sockets() -> Result<(), Box<dyn Error>> {
        pretty_env_logger::try_init().ok();

        let mut router_socket = zeromq::RouterSocket::new();
        let router_monitor = router_socket.monitor();
        let router_endpoint = router_socket.bind("tcp://localhost:0").await?;

        let mut dealer_socket = zeromq::DealerSocket::new();
        let dealer_monitor = dealer_socket.monitor();
        let dealer_endpoint = dealer_socket.bind("tcp://localhost:0").await?;

        let mut rep_socket = zeromq::RepSocket::new();
        let rep_monitor = rep_socket.monitor();
        rep_socket
            .connect(dealer_endpoint.to_string().as_str())
            .await?;

        let num_messages = 10;

        async_rt::task::spawn(async move {
            helpers::run_rep_server(rep_socket, num_messages)
                .await
                .unwrap();
        });

        let req_task = async_rt::task::spawn(async move {
            let mut req_socket = zeromq::ReqSocket::new();
            req_socket
                .connect(router_endpoint.to_string().as_str())
                .await
                .unwrap();
            helpers::run_req_client(req_socket, num_messages)
                .await
                .unwrap();
        });

        helpers::run_proxy(router_socket, dealer_socket, 100).await;

        req_task.await.unwrap();

        let router_events: Vec<_> = router_monitor.collect().await;
        let dealer_events: Vec<_> = dealer_monitor.collect().await;
        let rep_events: Vec<_> = rep_monitor.collect().await;
        assert_eq!(2, router_events.len(), "{:?}", &router_events);
        assert_eq!(2, dealer_events.len(), "{:?}", &dealer_events);
        assert_eq!(1, rep_events.len(), "{:?}", &rep_events);

        Ok(())
    }

    #[async_rt::test]
    async fn test_many_req_router_dealer_rep_sockets() -> Result<(), Box<dyn Error>> {
        pretty_env_logger::try_init().ok();

        let mut router_socket = zeromq::RouterSocket::new();
        let router_endpoint = router_socket.bind("tcp://localhost:0").await?;

        let mut dealer_socket = zeromq::DealerSocket::new();
        let dealer_endpoint = dealer_socket.bind("tcp://localhost:0").await?;

        let num_req_sockets = 100;
        let num_messages = 100;

        async_rt::task::spawn(async move {
            let mut rep_socket = zeromq::RepSocket::new();
            rep_socket
                .connect(dealer_endpoint.to_string().as_str())
                .await
                .unwrap();
            helpers::run_rep_server(rep_socket, num_req_sockets * num_messages)
                .await
                .unwrap();
        });

        let mut req_tasks = Vec::with_capacity(num_req_sockets as usize);
        for i in 0..num_req_sockets {
            let router_endpoint_clone = router_endpoint.to_string();
            let req_task = async_rt::task::spawn(async move {
                // yield for a moment to ensure that server has some time to open socket
                async_rt::task::sleep(Duration::from_millis(100)).await;
                let mut req_socket = zeromq::ReqSocket::new();
                req_socket.connect(&router_endpoint_clone).await.unwrap();
                helpers::run_req_client_with_id(req_socket, i, num_messages)
                    .await
                    .unwrap();
            });
            req_tasks.push(req_task);
        }

        helpers::run_proxy(router_socket, dealer_socket, 5000).await;

        for req_task in req_tasks {
            req_task.await.unwrap();
        }

        Ok(())
    }
}
