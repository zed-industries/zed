mod helpers;

use zeromq::__async_rt as async_rt;
use zeromq::prelude::*;

use futures::StreamExt;
use std::error::Error;
use std::time::Duration;

#[cfg(test)]
mod test {

    use super::*;
    #[async_rt::test]
    async fn test_req_rep_sockets() -> Result<(), Box<dyn Error>> {
        pretty_env_logger::try_init().ok();

        let mut rep_socket = zeromq::RepSocket::new();
        let monitor = rep_socket.monitor();
        let endpoint = rep_socket.bind("tcp://localhost:0").await?;
        println!("Started rep server on {}", endpoint);

        let num_messages = 10;

        async_rt::task::spawn(async move {
            let mut req_socket = zeromq::ReqSocket::new();
            req_socket
                .connect(endpoint.to_string().as_str())
                .await
                .unwrap();
            helpers::run_req_client(req_socket, num_messages)
                .await
                .unwrap();
        });

        helpers::run_rep_server(rep_socket, num_messages).await?;

        let events: Vec<_> = monitor.collect().await;
        assert_eq!(2, events.len(), "{:?}", &events);

        Ok(())
    }

    #[async_rt::test]
    async fn test_many_req_rep_sockets() -> Result<(), Box<dyn Error>> {
        pretty_env_logger::try_init().ok();

        let mut rep_socket = zeromq::RepSocket::new();
        let endpoint = rep_socket.bind("tcp://localhost:0").await?;
        println!("Started rep server on {}", endpoint);

        let num_req_sockets = 100;
        let num_messages = 100;

        for i in 0..num_req_sockets {
            let cloned_endpoint = endpoint.to_string();
            async_rt::task::spawn(async move {
                // yield for a moment to ensure that server has some time to open socket
                async_rt::task::sleep(Duration::from_millis(100)).await;
                let mut req_socket = zeromq::ReqSocket::new();
                req_socket.connect(&cloned_endpoint).await.unwrap();
                helpers::run_req_client_with_id(req_socket, i, num_messages)
                    .await
                    .unwrap();
            });
        }

        helpers::run_rep_server(rep_socket, num_req_sockets * num_messages).await?;

        Ok(())
    }
}
