use ntest::timeout;
use test_log::test;
use tracing::instrument;
use zbus::block_on;

use zbus::Result;

#[instrument]
#[test]
#[timeout(15000)]
fn issue_813() {
    // Our server-side handshake code was unable to handle FDs being sent in the first messages
    // if the client sent them too quickly after sending `BEGIN` command.
    //
    // We test this by manually sending out the auth commands together with 2 method calls with
    // 1 FD each. Before a fix for this issue, the server handshake would fail with an
    // `Unexpected FDs during handshake` error.
    use futures_util::try_join;
    use rustix::process::geteuid;
    #[cfg(not(feature = "tokio"))]
    use std::os::unix::net::UnixStream;
    use std::{os::fd::AsFd, vec};
    #[cfg(feature = "tokio")]
    use tokio::net::UnixStream;
    use zbus::{conn::socket::WriteHalf, connection::Builder};
    use zvariant::Fd;

    #[derive(Debug)]
    struct Issue813Iface {
        event: event_listener::Event,
        call_count: u8,
    }
    #[zbus::interface(interface = "org.zbus.Issue813")]
    impl Issue813Iface {
        #[instrument]
        fn pass_fd(&mut self, fd: Fd<'_>) {
            self.call_count += 1;
            tracing::debug!("`PassFd` called with {} {} times", fd, self.call_count);
            if self.call_count == 2 {
                self.event.notify(1);
            }
        }
    }
    #[zbus::proxy(
        gen_blocking = false,
        default_path = "/org/zbus/Issue813",
        interface = "org.zbus.Issue813"
    )]
    trait Issue813 {
        fn pass_fd(&self, fd: Fd<'_>) -> zbus::Result<()>;
    }

    block_on(async move {
        let guid = zbus::Guid::generate();
        let (p0, p1) = UnixStream::pair().unwrap();

        let client_event = event_listener::Event::new();
        let client_listener = client_event.listen();
        let server_event = event_listener::Event::new();
        let server_listener = server_event.listen();
        let server = async move {
            let _conn = Builder::unix_stream(p0)
                .server(guid)?
                .p2p()
                .serve_at(
                    "/org/zbus/Issue813",
                    Issue813Iface {
                        event: server_event,
                        call_count: 0,
                    },
                )?
                .name("org.zbus.Issue813")?
                .build()
                .await?;
            client_listener.await;

            Result::<()>::Ok(())
        };
        let client = async move {
            let commands = format!(
                "\0AUTH EXTERNAL {}\r\nNEGOTIATE_UNIX_FD\r\nBEGIN\r\n",
                hex::encode(geteuid().as_raw().to_string())
            );
            let mut bytes: Vec<u8> = commands.bytes().collect();
            let fd = std::io::stdin();
            let msg = zbus::message::Message::method_call("/org/zbus/Issue813", "PassFd")?
                .destination("org.zbus.Issue813")?
                .interface("org.zbus.Issue813")?
                .build(&(Fd::from(fd.as_fd())))?;
            let msg_data = msg.data();
            let mut fds = vec![];
            for _ in 0..2 {
                bytes.extend_from_slice(msg_data);
                fds.push(fd.as_fd());
            }

            #[cfg(feature = "tokio")]
            let mut split = zbus::conn::Socket::split(p1);
            #[cfg(not(feature = "tokio"))]
            let mut split = zbus::conn::Socket::split(async_io::Async::new(p1)?);
            split.write_mut().sendmsg(&bytes, &fds).await?;

            server_listener.await;
            client_event.notify(1);

            Ok(())
        };
        let (_, _) = try_join!(client, server)?;

        Result::<()>::Ok(())
    })
    .unwrap();
}
