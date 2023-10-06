use anyhow::anyhow;
use cli::{ipc::IpcSender, CliRequest, CliResponse};
use futures::channel::mpsc;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use std::ffi::OsStr;
use std::os::unix::prelude::OsStrExt;
use std::sync::atomic::Ordering;
use std::{path::PathBuf, sync::atomic::AtomicBool};
use util::channel::parse_zed_link;
use util::ResultExt;

use crate::connect_to_cli;

pub enum OpenRequest {
    Paths {
        paths: Vec<PathBuf>,
    },
    CliConnection {
        connection: (mpsc::Receiver<CliRequest>, IpcSender<CliResponse>),
    },
    JoinChannel {
        channel_id: u64,
    },
}

pub struct OpenListener {
    tx: UnboundedSender<OpenRequest>,
    pub triggered: AtomicBool,
}

impl OpenListener {
    pub fn new() -> (Self, UnboundedReceiver<OpenRequest>) {
        let (tx, rx) = mpsc::unbounded();
        (
            OpenListener {
                tx,
                triggered: AtomicBool::new(false),
            },
            rx,
        )
    }

    pub fn open_urls(&self, urls: Vec<String>) {
        self.triggered.store(true, Ordering::Release);
        let request = if let Some(server_name) =
            urls.first().and_then(|url| url.strip_prefix("zed-cli://"))
        {
            self.handle_cli_connection(server_name)
        } else if let Some(request_path) = urls.first().and_then(|url| parse_zed_link(url)) {
            self.handle_zed_url_scheme(request_path)
        } else {
            self.handle_file_urls(urls)
        };

        if let Some(request) = request {
            self.tx
                .unbounded_send(request)
                .map_err(|_| anyhow!("no listener for open requests"))
                .log_err();
        }
    }

    fn handle_cli_connection(&self, server_name: &str) -> Option<OpenRequest> {
        if let Some(connection) = connect_to_cli(server_name).log_err() {
            return Some(OpenRequest::CliConnection { connection });
        }

        None
    }

    fn handle_zed_url_scheme(&self, request_path: &str) -> Option<OpenRequest> {
        let mut parts = request_path.split("/");
        if parts.next() == Some("channel") {
            if let Some(slug) = parts.next() {
                if let Some(id_str) = slug.split("-").last() {
                    if let Ok(channel_id) = id_str.parse::<u64>() {
                        return Some(OpenRequest::JoinChannel { channel_id });
                    }
                }
            }
        }
        None
    }

    fn handle_file_urls(&self, urls: Vec<String>) -> Option<OpenRequest> {
        let paths: Vec<_> = urls
            .iter()
            .flat_map(|url| url.strip_prefix("file://"))
            .map(|url| {
                let decoded = urlencoding::decode_binary(url.as_bytes());
                PathBuf::from(OsStr::from_bytes(decoded.as_ref()))
            })
            .collect();

        Some(OpenRequest::Paths { paths })
    }
}
