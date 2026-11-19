//! The sandbox's DNS server.
//!
//! Every DNS query from the sandbox reaches the supervisor: port 53 of every
//! address in the sandbox's namespace is a socket it serves, whichever server
//! the command thinks it's asking. Names are answered with fake IPv4
//! addresses from `198.18.0.0/15` (one per name per sandbox), so a later
//! connection to one says which name it's for, and the policy is checked
//! against the name.
//!
//! - A name no rule could allow doesn't exist (`NXDOMAIN`).
//! - A name only `Ask` rules cover gets an address without a host lookup:
//!   nothing about it leaves the machine before the user decides.
//! - A name an `Allow` rule covers is looked up on the host first, so names
//!   that don't exist still fail to resolve.
//!
//! Only `A` queries get answers: `AAAA` and other types get none, so clients
//! fall back to IPv4. `PTR` queries for fake addresses return their names.

use std::collections::HashMap;
use std::io::{self, Read as _, Write as _};
use std::net::{Ipv4Addr, TcpListener, TcpStream, ToSocketAddrs as _};
use std::os::fd::{AsFd as _, OwnedFd};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use crate::net::{self, lock};
use crate::policy::{NameVerdict, normalize_domain};
use crate::supervisor::{Execution, SandboxState};

const FAKE_BASE: u32 = u32::from_be_bytes([198, 18, 0, 0]);
const FAKE_SIZE: u32 = 1 << 17;
/// Short, so a fake address isn't used long after a policy change.
const TTL: u32 = 10;
const HOST_LOOKUP_CACHE: Duration = Duration::from_secs(10);
const MAX_CONCURRENT_QUERIES: usize = 32;
const TCP_IDLE_TIMEOUT: Duration = Duration::from_secs(10);

const TYPE_A: u16 = 1;
const TYPE_PTR: u16 = 12;
const TYPE_ANY: u16 = 255;
const CLASS_IN: u16 = 1;
const FORMAT_ERROR: u16 = 1;
const NAME_ERROR: u16 = 3;
const NOT_IMPLEMENTED: u16 = 4;

pub(crate) fn is_fake(address: Ipv4Addr) -> bool {
    u32::from(address).wrapping_sub(FAKE_BASE) < FAKE_SIZE
}

/// A sandbox's fake addresses, which last as long as it does.
#[derive(Default)]
pub(crate) struct Names {
    by_name: HashMap<String, Ipv4Addr>,
    by_address: HashMap<Ipv4Addr, String>,
    allocated: u32,
    /// Whether names exist on the host, as recently looked up.
    host_lookups: HashMap<String, (Instant, bool)>,
}

impl Names {
    fn address_for(&mut self, name: &str) -> Option<Ipv4Addr> {
        if let Some(address) = self.by_name.get(name) {
            return Some(*address);
        }
        if self.allocated + 1 >= FAKE_SIZE {
            return None;
        }
        self.allocated += 1;
        let address = Ipv4Addr::from(FAKE_BASE + self.allocated);
        self.by_name.insert(name.to_string(), address);
        self.by_address.insert(address, name.to_string());
        Some(address)
    }

    pub fn name_for(&self, address: Ipv4Addr) -> Option<String> {
        self.by_address.get(&address).cloned()
    }
}

pub(crate) fn serve_udp(execution: &Arc<Execution>, socket: &Arc<OwnedFd>) {
    let in_flight = Arc::new(AtomicUsize::new(0));
    let mut buffer = vec![0; 4096];
    while !execution.network.is_closed() {
        let datagram = match net::receive_datagram(socket.as_fd(), &mut buffer) {
            Ok(Some(datagram)) => datagram,
            Ok(None) => continue,
            Err(error) if error.raw_os_error() == Some(libc::EINTR) => continue,
            Err(error) => {
                eprintln!("zed sandbox supervisor: receiving DNS queries: {error}");
                return;
            }
        };
        let query = buffer[..datagram.length].to_vec();
        let respond = {
            let execution = execution.clone();
            let socket = socket.clone();
            move || {
                if let Some(response) = answer(&execution.sandbox, &query) {
                    // The reply comes from the address that was asked.
                    if let Err(error) = net::send_datagram(
                        socket.as_fd(),
                        &response,
                        datagram.source,
                        datagram.destination,
                    ) {
                        eprintln!("zed sandbox supervisor: answering a DNS query: {error}");
                    }
                }
            }
        };
        // Answering may wait for a host lookup, so answer concurrently (up
        // to a limit).
        if in_flight.fetch_add(1, Ordering::SeqCst) < MAX_CONCURRENT_QUERIES {
            let in_flight = in_flight.clone();
            let spawned = std::thread::Builder::new()
                .name("zed-sandbox-dns-query".to_string())
                .spawn(move || {
                    respond();
                    in_flight.fetch_sub(1, Ordering::SeqCst);
                });
            if let Err(error) = spawned {
                eprintln!("zed sandbox supervisor: answering a DNS query: {error}");
            }
        } else {
            in_flight.fetch_sub(1, Ordering::SeqCst);
            respond();
        }
    }
}

pub(crate) fn serve_tcp(execution: &Arc<Execution>, socket: &Arc<OwnedFd>) {
    let listener = match socket.try_clone() {
        Ok(listener) => TcpListener::from(listener),
        Err(error) => {
            eprintln!("zed sandbox supervisor: serving DNS over TCP: {error}");
            return;
        }
    };
    while !execution.network.is_closed() {
        match listener.accept() {
            Ok((stream, _)) => {
                let execution = execution.clone();
                let spawned = std::thread::Builder::new()
                    .name("zed-sandbox-dns-tcp".to_string())
                    .spawn(move || serve_tcp_connection(&execution.sandbox, stream));
                if let Err(error) = spawned {
                    eprintln!("zed sandbox supervisor: serving DNS over TCP: {error}");
                }
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            // Shut down with the execution.
            Err(_) => return,
        }
    }
}

/// Length-prefixed queries and answers, until the client stops.
fn serve_tcp_connection(sandbox: &SandboxState, mut stream: TcpStream) {
    let mut serve = || -> io::Result<()> {
        stream.set_read_timeout(Some(TCP_IDLE_TIMEOUT))?;
        loop {
            let mut length = [0; 2];
            stream.read_exact(&mut length)?;
            let mut query = vec![0; usize::from(u16::from_be_bytes(length))];
            stream.read_exact(&mut query)?;
            let Some(response) = answer(sandbox, &query) else {
                return Ok(());
            };
            let mut message = (response.len() as u16).to_be_bytes().to_vec();
            message.extend(response);
            stream.write_all(&message)?;
        }
    };
    // Ending (or timing out, or a malformed length) closes the connection,
    // which is all a DNS client needs to know.
    serve().ok();
}

struct Question {
    name: String,
    record_type: u16,
    class: u16,
    /// Where the question ends in the query.
    end: usize,
}

/// The response to one query, if it is one.
fn answer(sandbox: &SandboxState, query: &[u8]) -> Option<Vec<u8>> {
    let flags = u16::from_be_bytes(query.get(2..4)?.try_into().ok()?);
    if flags & 0x8000 != 0 {
        return None;
    }
    let opcode = (flags >> 11) & 0xf;
    let question_count = u16::from_be_bytes(query.get(4..6)?.try_into().ok()?);
    if opcode != 0 {
        return Some(response(query, NOT_IMPLEMENTED, None, None));
    }
    let question = match question_count {
        1 => parse_question(query),
        _ => None,
    };
    let Some(question) = question else {
        return Some(response(query, FORMAT_ERROR, None, None));
    };
    let echoed = &query[12..question.end];
    if question.class != CLASS_IN {
        return Some(response(query, NOT_IMPLEMENTED, Some(echoed), None));
    }
    if question.record_type == TYPE_PTR {
        let name = reverse_lookup(sandbox, &question.name);
        return Some(match name {
            Some(name) => response(query, 0, Some(echoed), Some((TYPE_PTR, encode_name(&name)))),
            None => response(query, NAME_ERROR, Some(echoed), None),
        });
    }
    Some(match resolve(sandbox, &question.name) {
        None => response(query, NAME_ERROR, Some(echoed), None),
        Some(address) if matches!(question.record_type, TYPE_A | TYPE_ANY) => response(
            query,
            0,
            Some(echoed),
            Some((TYPE_A, address.octets().to_vec())),
        ),
        // The name exists, but has no records of this type.
        Some(_) => response(query, 0, Some(echoed), None),
    })
}

fn parse_question(query: &[u8]) -> Option<Question> {
    let mut position = 12;
    let mut labels = Vec::new();
    loop {
        let length = usize::from(*query.get(position)?);
        position += 1;
        if length == 0 {
            break;
        }
        // Longer "labels" are compression pointers, which questions don't use.
        if length > 63 {
            return None;
        }
        let label = query.get(position..position + length)?;
        position += length;
        if label.contains(&b'.') {
            return None;
        }
        labels.push(String::from_utf8_lossy(label).into_owned());
    }
    let record_type = u16::from_be_bytes(query.get(position..position + 2)?.try_into().ok()?);
    let class = u16::from_be_bytes(query.get(position + 2..position + 4)?.try_into().ok()?);
    Some(Question {
        name: normalize_domain(&labels.join(".")),
        record_type,
        class,
        end: position + 4,
    })
}

fn response(
    query: &[u8],
    code: u16,
    question: Option<&[u8]>,
    answer: Option<(u16, Vec<u8>)>,
) -> Vec<u8> {
    let query_flags = u16::from_be_bytes([query[2], query[3]]);
    // A response, with the query's opcode and "recursion desired", and
    // recursion available.
    let flags = 0x8000 | (query_flags & 0x7800) | (query_flags & 0x0100) | 0x0080 | code;
    let mut message = query[0..2].to_vec();
    message.extend(flags.to_be_bytes());
    message.extend(u16::from(question.is_some()).to_be_bytes());
    message.extend(u16::from(answer.is_some()).to_be_bytes());
    message.extend([0; 4]);
    if let Some(question) = question {
        message.extend(question);
    }
    if let Some((record_type, data)) = answer {
        // The name: a pointer to the question's, right after the header.
        message.extend([0xc0, 12]);
        message.extend(record_type.to_be_bytes());
        message.extend(CLASS_IN.to_be_bytes());
        message.extend(TTL.to_be_bytes());
        message.extend((data.len() as u16).to_be_bytes());
        message.extend(data);
    }
    message
}

fn encode_name(name: &str) -> Vec<u8> {
    let mut encoded = Vec::new();
    for label in name.split('.').filter(|label| !label.is_empty()) {
        encoded.push(label.len() as u8);
        encoded.extend(label.as_bytes());
    }
    encoded.push(0);
    encoded
}

/// The fake address for `name`, if it should resolve.
fn resolve(sandbox: &SandboxState, name: &str) -> Option<Ipv4Addr> {
    let exists = match sandbox.policy().name_verdict(name) {
        NameVerdict::Deny => false,
        NameVerdict::Ask => true,
        NameVerdict::Allow => exists_on_host(sandbox, name),
    };
    if !exists {
        return None;
    }
    lock(&sandbox.names).address_for(name)
}

fn exists_on_host(sandbox: &SandboxState, name: &str) -> bool {
    if let Some((looked_up, exists)) = lock(&sandbox.names).host_lookups.get(name)
        && looked_up.elapsed() < HOST_LOOKUP_CACHE
    {
        return *exists;
    }
    let exists = !name.is_empty()
        && (name, 0)
            .to_socket_addrs()
            .is_ok_and(|mut addresses| addresses.next().is_some());
    let mut names = lock(&sandbox.names);
    names
        .host_lookups
        .retain(|_, (looked_up, _)| looked_up.elapsed() < HOST_LOOKUP_CACHE);
    names
        .host_lookups
        .insert(name.to_string(), (Instant::now(), exists));
    exists
}

/// The name of a fake address, from its `in-addr.arpa` name.
fn reverse_lookup(sandbox: &SandboxState, name: &str) -> Option<String> {
    let reversed = name.strip_suffix(".in-addr.arpa")?;
    let mut octets = [0u8; 4];
    let mut parts = reversed.split('.');
    for octet in octets.iter_mut().rev() {
        *octet = parts.next()?.parse().ok()?;
    }
    if parts.next().is_some() {
        return None;
    }
    lock(&sandbox.names).name_for(Ipv4Addr::from(octets))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query(name: &str, record_type: u16) -> Vec<u8> {
        let mut query = vec![0x12, 0x34, 0x01, 0x00, 0, 1, 0, 0, 0, 0, 0, 0];
        query.extend(encode_name(name));
        query.extend(record_type.to_be_bytes());
        query.extend(CLASS_IN.to_be_bytes());
        query
    }

    #[test]
    fn test_question_round_trip() {
        let query = query("Example.COM", TYPE_A);
        let question = parse_question(&query).unwrap();
        assert_eq!(question.name, "example.com");
        assert_eq!(question.record_type, TYPE_A);
        assert_eq!(question.end, query.len());

        let response = response(
            &query,
            0,
            Some(&query[12..question.end]),
            Some((TYPE_A, vec![198, 18, 0, 1])),
        );
        assert_eq!(&response[0..2], &[0x12, 0x34]);
        assert_eq!(u16::from_be_bytes([response[2], response[3]]), 0x8180);
        assert_eq!(&response[6..8], &[0, 1]);
        assert_eq!(&response[response.len() - 4..], &[198, 18, 0, 1]);
        assert!(is_fake(Ipv4Addr::new(198, 19, 255, 255)));
        assert!(!is_fake(Ipv4Addr::new(198, 20, 0, 0)));
    }
}
