#![cfg(not(target_os = "unknown"))]

use async_std::{fs, io, net::ToSocketAddrs, task};

#[test]
fn open_file() {
    task::block_on(async {
        let non_existing_file = "/ashjudlkahasdasdsikdhajik/asdasdasdasdasdasd/fjuiklashdbflasas";
        let res = fs::File::open(non_existing_file).await;
        match res {
            Ok(_) => panic!("Found file with random name: We live in a simulation"),
            Err(e) => assert_eq!(
                "could not open `/ashjudlkahasdasdsikdhajik/asdasdasdasdasdasd/fjuiklashdbflasas`",
                &format!("{}", e)
            ),
        }
    })
}

#[test]
fn resolve_address() {
    task::block_on(async {
        let non_existing_addr = "ashjudlkahasdasdsikdhajik.asdasdasdasdasdasd.fjuiklashdbflasas:80";
        let res: Result<_, io::Error> = non_existing_addr.to_socket_addrs().await;
        match res {
            Ok(_) => panic!("Found address with random name: We live in a simulation"),
            Err(e) => assert_eq!(
                "could not resolve address `\"ashjudlkahasdasdsikdhajik.asdasdasdasdasdasd.fjuiklashdbflasas:80\"`",
                &format!("{}", e)
            ),
        }
    })
}
