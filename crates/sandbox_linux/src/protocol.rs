//! Messages between Zed, the supervisor and the helper.
//!
//! Every channel is an anonymous `SOCK_SEQPACKET` socketpair, so each message
//! is one packet: a JSON body plus any descriptors as `SCM_RIGHTS`. Packets
//! are capped by the socket buffer (~200 KiB by default), so larger bodies
//! (e.g. a big environment) travel in a memfd passed as the first fd.
//!
//! ```text
//! Zed    <--control-->  supervisor  ZedMessage / SupervisorMessage
//! Zed    --setup----->  helper      HelperSetup (once)
//! helper --attach---->  supervisor  AttachInfo + the seccomp listener (once)
//! helper <-attach---->  supervisor  FactoryRequest / FactoryResponse (then)
//! ```

use std::io::{IoSlice, IoSliceMut};
use std::net::SocketAddr;
use std::os::fd::{AsRawFd as _, BorrowedFd, FromRawFd as _, OwnedFd, RawFd};
use std::path::PathBuf;

use anyhow::{Context as _, Result, bail};
use nix::sys::socket::{ControlMessage, ControlMessageOwned, MsgFlags, recvmsg, sendmsg};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::UntrustedCommand;
use crate::net::SocketKind;
use crate::policy::{HandlerId, RequestedAccess, WirePolicy, WireRule};

const MAX_FDS: usize = 8;
const MAX_INLINE_SIZE: usize = 64 * 1024;
const MAX_MESSAGE_SIZE: usize = 16 * 1024 * 1024;

const INLINE_BODY: u8 = 0;
const MEMFD_BODY: u8 = 1;

pub(crate) type SandboxId = u64;
pub(crate) type RequestId = u64;
pub(crate) type PromptId = u64;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ZedMessage {
    Request {
        id: RequestId,
        request: Request,
    },
    PromptReply {
        prompt: PromptId,
        allow: bool,
        /// Appended to the sandbox's policy before the syscall is answered.
        remember: Vec<WireRule>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Request {
    /// `sandbox` is chosen by Zed, so it can register the policy's handlers
    /// under it before the supervisor can ask any of them.
    CreateSandbox {
        sandbox: SandboxId,
        policy: WirePolicy,
    },
    UpdatePolicy {
        sandbox: SandboxId,
        policy: WirePolicy,
    },
    DestroySandbox {
        sandbox: SandboxId,
    },
    /// Descriptors: the supervisor's end of a fresh attach socketpair, whose
    /// other end goes to the helper of one execution in `sandbox`.
    AttachExecution {
        sandbox: SandboxId,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum SupervisorMessage {
    Response {
        id: RequestId,
        response: Response,
    },
    Prompt(PromptRequest),
    /// The syscall behind a prompt was abandoned or timed out; a reply is no
    /// longer needed.
    PromptCancelled {
        prompt: PromptId,
    },
    Denied {
        sandbox: SandboxId,
        denial: Denial,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Response {
    PolicyUpdated { still_open: Vec<OpenFile> },
    Done,
    Error { message: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct PromptRequest {
    pub prompt: PromptId,
    pub sandbox: SandboxId,
    /// The handler (in Zed's table) that decides.
    pub handler: HandlerId,
    pub requests: Vec<RequestedAccess>,
    pub pid: i32,
    pub executable: Option<PathBuf>,
}

/// An operation the policy, or a handler answering an [`Access::Ask`],
/// denied.
///
/// [`Access::Ask`]: crate::Access::Ask
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Denial {
    /// What the denied syscall needed that wasn't granted.
    pub requests: Vec<RequestedAccess>,
    /// The thread that made the syscall.
    pub pid: i32,
    pub executable: Option<PathBuf>,
}

/// A file a sandboxed process still has open with access the current policy
/// no longer grants. Revoking a grant never affects open descriptors.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenFile {
    pub pid: i32,
    pub path: PathBuf,
    pub read: bool,
    pub write: bool,
}

/// Sent by the helper's command (before it execs) with its seccomp listener,
/// followed by the sandbox's DNS sockets.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AttachInfo {
    /// Every process of the execution descends from the helper, which is a
    /// child subreaper.
    pub helper_pid: i32,
}

/// Asks the helper for a socket in the sandbox's network namespace, where
/// only it can create them, or for a duplicate of a descriptor of one of its
/// processes. Answered with a [`FactoryResponse`], carrying the socket if
/// there was one.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum FactoryRequest {
    CreateSocket {
        kind: SocketKind,
        address: SocketAddr,
    },
    /// Fetching a descriptor is `pidfd_getfd(2)`, which the kernel gates on
    /// ptrace access. The supervisor may be denied that as a mere sibling of
    /// the command (by Yama's `ptrace_scope`, or AppArmor's restriction of
    /// unprivileged user namespaces), so it asks the helper, every command's
    /// parent, instead.
    FetchFd { pid: i32, fd: i32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum FactoryResponse {
    Created,
    Fetched,
    Failed { errno: i32 },
}

/// Everything the helper needs to set up the sandbox and run the command.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct HelperSetup {
    pub command: UntrustedCommand,
}

pub(crate) fn send_message<T: Serialize>(
    socket: BorrowedFd<'_>,
    message: &T,
    fds: &[RawFd],
) -> Result<()> {
    let body = serde_json::to_vec(message).context("serializing sandbox message")?;
    if body.len() > MAX_MESSAGE_SIZE {
        bail!("sandbox message too large ({} bytes)", body.len());
    }
    if fds.len() >= MAX_FDS {
        bail!(
            "too many descriptors in one sandbox message ({})",
            fds.len()
        );
    }
    let memfd;
    let mut all_fds = Vec::with_capacity(fds.len() + 1);
    let packet = if body.len() <= MAX_INLINE_SIZE {
        let mut packet = vec![INLINE_BODY];
        packet.extend_from_slice(&body);
        packet
    } else {
        memfd = body_memfd(&body)?;
        all_fds.push(memfd.as_raw_fd());
        vec![MEMFD_BODY]
    };
    all_fds.extend_from_slice(fds);
    let iov = [IoSlice::new(&packet)];
    let control = [ControlMessage::ScmRights(&all_fds)];
    let control: &[ControlMessage] = if all_fds.is_empty() { &[] } else { &control };
    sendmsg::<()>(
        socket.as_raw_fd(),
        &iov,
        control,
        MsgFlags::MSG_NOSIGNAL,
        None,
    )
    .context("sending sandbox message")?;
    Ok(())
}

/// Receive one message. `Ok(None)` means the peer closed the channel.
pub(crate) fn receive_message<T: DeserializeOwned>(
    socket: BorrowedFd<'_>,
) -> Result<Option<(T, Vec<OwnedFd>)>> {
    let mut body = vec![0u8; MAX_INLINE_SIZE + 1];
    let mut control = nix::cmsg_space!([RawFd; MAX_FDS]);
    let mut fds = Vec::new();
    let (length, flags) = {
        let mut iov = [IoSliceMut::new(&mut body)];
        let message = recvmsg::<()>(
            socket.as_raw_fd(),
            &mut iov,
            Some(&mut control),
            MsgFlags::MSG_CMSG_CLOEXEC,
        )
        .context("receiving sandbox message")?;
        for control_message in message.cmsgs().context("reading descriptors")? {
            if let ControlMessageOwned::ScmRights(raw_fds) = control_message {
                for raw_fd in raw_fds {
                    // SAFETY: the kernel installed each received descriptor
                    // fresh in our table; we take sole ownership here.
                    fds.push(unsafe { OwnedFd::from_raw_fd(raw_fd) });
                }
            }
        }
        (message.bytes, message.flags)
    };
    if flags.intersects(MsgFlags::MSG_TRUNC | MsgFlags::MSG_CTRUNC) {
        bail!("sandbox message was truncated");
    }
    if length == 0 && fds.is_empty() {
        return Ok(None);
    }
    let message = match body.first() {
        Some(&INLINE_BODY) => serde_json::from_slice(&body[1..length]),
        Some(&MEMFD_BODY) if length == 1 && !fds.is_empty() => {
            let mut file = std::fs::File::from(fds.remove(0));
            let mut contents = Vec::new();
            std::io::Read::read_to_end(
                &mut std::io::Read::take(&mut file, MAX_MESSAGE_SIZE as u64),
                &mut contents,
            )?;
            serde_json::from_slice(&contents)
        }
        _ => bail!("malformed sandbox message"),
    }
    .context("parsing sandbox message")?;
    Ok(Some((message, fds)))
}

fn body_memfd(body: &[u8]) -> Result<OwnedFd> {
    // SAFETY: valid NUL-terminated name.
    let fd = crate::sys::cvt(unsafe {
        libc::memfd_create(c"zed-sandbox-message".as_ptr(), libc::MFD_CLOEXEC)
    })?;
    // SAFETY: a freshly returned descriptor that nothing else owns.
    let mut file = std::fs::File::from(unsafe { OwnedFd::from_raw_fd(fd) });
    std::io::Write::write_all(&mut file, body)?;
    std::io::Seek::rewind(&mut file)?;
    Ok(file.into())
}
