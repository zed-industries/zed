#![deny(warnings)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::test]
async fn remote_addr_missing() {
    let extract_remote_addr = warp::addr::remote();

    let req = warp::test::request();
    let resp = req.filter(&extract_remote_addr).await.unwrap();
    assert_eq!(resp, None)
}

#[tokio::test]
async fn remote_addr_present() {
    let extract_remote_addr = warp::addr::remote();

    let req = warp::test::request().remote_addr("1.2.3.4:5678".parse().unwrap());
    let resp = req.filter(&extract_remote_addr).await.unwrap();
    assert_eq!(
        resp,
        Some(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)), 5678))
    )
}
