use std::convert::TryInto;

/// NOTE: This will block. Careful when using in async code.
pub fn get_monitor_event(monitor: &zmq2::Socket) -> (zmq2::SocketEvent, u32, String) {
    assert_eq!(monitor.get_socket_type().unwrap(), zmq2::PAIR);
    let mut msgs = monitor.recv_multipart(0).expect("Monitor couldn't recv");
    assert_eq!(msgs.len(), 2);

    assert_eq!(msgs[0].len(), 6);
    let event: [u8; 2] = msgs[0][..2].try_into().unwrap();
    let event_value: [u8; 4] = msgs[0][2..].try_into().unwrap();
    // TODO: what is the endianness of zmq here? Is it platform dependent?
    let event = zmq2::SocketEvent::from_raw(u16::from_le_bytes(event));
    let event_value = u32::from_le_bytes(event_value);
    let remote_endpoint = String::from_utf8(msgs.pop().unwrap()).unwrap();

    (event, event_value, remote_endpoint)
}

/// Configures `their_sock` with a socket monitor, and returns the monitor
pub fn setup_monitor(
    ctx: &zmq2::Context,
    their_sock: &zmq2::Socket,
    monitor_endpoint: &str,
) -> zmq2::Socket {
    their_sock
        .monitor(monitor_endpoint, zmq2::SocketEvent::ALL.to_raw().into())
        .expect("Failed to set up monitor");
    let their_monitor = ctx.socket(zmq2::PAIR).expect("Couldnt make pair socket");
    their_monitor
        .connect(monitor_endpoint)
        .expect("Failed to connect monitor");
    their_monitor
}
