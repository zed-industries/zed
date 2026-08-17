use asynchronous_codec::{Bytes, Framed, LengthCodec};
use futures::io::Cursor;
use futures::{executor, SinkExt, StreamExt};

#[test]
fn same_msgs_are_received_as_were_sent() {
    let cur = Cursor::new(vec![0; 256]);
    let mut framed = Framed::new(cur, LengthCodec {});

    let send_msgs = async {
        framed.send(Bytes::from("msg1")).await.unwrap();
        framed.send(Bytes::from("msg2")).await.unwrap();
        framed.send(Bytes::from("msg3")).await.unwrap();
    };
    executor::block_on(send_msgs);

    let mut cur = framed.into_inner();
    cur.set_position(0);
    let framed = Framed::new(cur, LengthCodec {});

    let recv_msgs = framed
        .take(3)
        .map(|res| res.unwrap())
        .map(|buf| String::from_utf8(buf.to_vec()).unwrap())
        .collect::<Vec<_>>();
    let msgs: Vec<String> = executor::block_on(recv_msgs);

    assert!(msgs == vec!["msg1", "msg2", "msg3"]);
}
