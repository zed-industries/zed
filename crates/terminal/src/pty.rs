use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    sync::{Arc, Mutex},
    time::Duration,
};

use alacritty_terminal::{
    event::OnResize,
    tty::{EventedPty, EventedReadWrite},
};
use futures::{
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    AsyncReadExt, SinkExt,
};
use gpui::{AsyncAppContext, WeakModel};
use polling::{Event, PollMode, Poller};
use smol::stream::StreamExt;

pub struct RemotePty {
    reader: UnixStream,
    writer: UnixStream,
}

impl RemotePty {
    pub async fn new(
        cx: &AsyncAppContext,
    ) -> (Self, UnboundedSender<Vec<u8>>, UnboundedReceiver<Vec<u8>>) {
        let (host_tx, host_rx) = mpsc::unbounded::<Vec<u8>>();
        let (mut input_tx, input_rx) = mpsc::unbounded();

        // let reader = VecDeque::new();
        // println!("before open fifo");
        // let reader = smol::unblock(|| File::open("/tmp/fifo").unwrap()).await;
        // println!("after open fifo");
        // let writer = VecDeque::new();

        let (mut sender, reader) = UnixStream::pair().unwrap();
        let (receiver, writer) = UnixStream::pair().unwrap();

        reader.set_nonblocking(true).unwrap();
        // reader.set_read_timeout(Some(Duration::from_secs(2)));
        let result = Self { reader, writer };

        cx.spawn(|_| async move {
            // let mut write = File::create("/tmp/fifo").unwrap();
            let mut rx = host_rx;
            loop {
                if let Some(data) = rx.next().await {
                    print!("!!on data ");
                    for byte in data.iter() {
                        print!("{:02x} ", byte);
                    }
                    println!();
                    let _ = sender.write_all(data.as_ref());
                    println!("!!data written");
                } else {
                    println!("!!break");
                    break;
                }
            }
        })
        .detach();
        cx.spawn(|_| async move {
            let mut buffer = [0u8; 1024];
            let mut receiver = smol::Unblock::new(receiver);
            loop {
                let n = receiver.read(&mut buffer).await.unwrap();
                if n == 0 {
                    break;
                }
                print!("!!read data ");
                for byte in buffer[..n].iter() {
                    print!("{:02x} ", byte);
                }
                println!();
                input_tx.send(buffer[..n].to_vec()).await.unwrap();
            }
        })
        .detach();

        (result, host_tx, input_rx)
    }

    // pub fn write_to_reader(&mut self, data: &[u8]) -> std::io::Result<()> {
    //     self.reader.write_all(data)?;
    //     if let Some(poll) = &*self.poller.lock().unwrap() {
    //         let _ = poll.notify();
    //     }

    //     Ok(())
    // }
}

impl EventedReadWrite for RemotePty {
    // type Reader = VecDeque<u8>;

    type Reader = UnixStream;
    type Writer = UnixStream;

    unsafe fn register(
        &mut self,
        poll: &std::sync::Arc<Poller>,
        mut interest: Event,
        mode: PollMode,
    ) -> std::io::Result<()> {
        println!("register");
        interest.key = 0; // PTY_READ_WRITE_TOKEN
        poll.add_with_mode(&self.reader, interest, mode)?;
        // if !self.reader.is_empty() {
        //     poll.notify()?;
        // }
        Ok(())
    }

    fn reregister(
        &mut self,
        poll: &std::sync::Arc<Poller>,
        interest: Event,
        mode: PollMode,
    ) -> std::io::Result<()> {
        println!("reregister");
        poll.modify_with_mode(&self.reader, interest, mode)?;
        // if !self.reader.is_empty() {
        //     poll.notify()?;
        // }
        Ok(())
    }

    fn deregister(&mut self, poll: &std::sync::Arc<Poller>) -> std::io::Result<()> {
        println!("deregister");
        poll.delete(&self.reader)?;
        Ok(())
    }

    fn reader(&mut self) -> &mut Self::Reader {
        println!("reader");
        &mut self.reader
    }

    fn writer(&mut self) -> &mut Self::Writer {
        println!("writer");
        &mut self.writer
    }
}

impl EventedPty for RemotePty {
    fn next_child_event(&mut self) -> Option<alacritty_terminal::tty::ChildEvent> {
        None
    }
}

impl OnResize for RemotePty {
    fn on_resize(&mut self, window_size: alacritty_terminal::event::WindowSize) {
        // todo!()
        println!("resize")
    }
}
