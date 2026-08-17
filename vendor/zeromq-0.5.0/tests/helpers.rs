use zeromq::__async_rt as async_rt;
use zeromq::prelude::*;
use zeromq::ZmqMessage;

use std::error::Error;
use std::str;
use std::time::Duration;

#[allow(dead_code)] // Rust #46379
pub async fn run_proxy(
    router_socket: zeromq::RouterSocket,
    dealer_socket: zeromq::DealerSocket,
    duration: u64,
) {
    let _ = async_rt::task::timeout(
        Duration::from_millis(duration),
        zeromq::proxy(router_socket, dealer_socket, None),
    )
    .await;
}

pub async fn run_rep_server(
    mut rep_socket: zeromq::RepSocket,
    num_messages: u32,
) -> Result<(), Box<dyn Error>> {
    for i in 0..num_messages {
        let mess = rep_socket.recv().await?;
        let m = format!(
            "{}, Rep - {}",
            str::from_utf8(mess.get(0).unwrap().as_ref()).unwrap(),
            i
        );
        let repl = ZmqMessage::from(m);
        rep_socket.send(repl).await?;
    }
    let errs = rep_socket.close().await;
    if !errs.is_empty() {
        panic!("Could not unbind socket: {:?}", errs);
    }
    Ok(())
}

pub async fn run_req_client(
    mut req_socket: zeromq::ReqSocket,
    num_messages: u32,
) -> Result<(), Box<dyn Error>> {
    for i in 0..num_messages {
        let ms: String = format!("Req - {}", i);
        let m = ZmqMessage::from(ms);
        req_socket.send(m).await.unwrap();
        let repl = req_socket.recv().await.unwrap();
        assert_eq!(
            format!("Req - {}, Rep - {}", i, i),
            String::from_utf8(repl.get(0).unwrap().to_vec()).unwrap()
        );
    }
    req_socket.close().await;
    Ok(())
}

pub async fn run_req_client_with_id(
    mut req_socket: zeromq::ReqSocket,
    id: u32,
    num_messages: u32,
) -> Result<(), Box<dyn Error>> {
    for i in 0..num_messages {
        let ms: String = format!("Socket - {}, Req - {}", id, i);
        let m = ZmqMessage::from(ms);
        req_socket.send(m).await.unwrap();
        let repl = req_socket.recv().await.unwrap();
        // With many parallel req sockets reply order cannot be quaranteed.
        let expected_ms = format!("Socket - {}, Req - {}, Rep - ", id, i);
        let actual_ms = String::from_utf8(repl.get(0).unwrap().to_vec()).unwrap();
        assert_eq!(expected_ms, actual_ms[..expected_ms.len()]);
    }
    req_socket.close().await;
    Ok(())
}
