//! Bindings for [`SCNetworkReachability`]
//!
//! [`SCNetworkReachability`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkreachability-g7d

use core_foundation::{
    base::{TCFType, ToVoid},
    runloop::CFRunLoop,
    string::{CFString, CFStringRef},
};
use system_configuration_sys::{
    libc,
    network_reachability::{
        kSCNetworkReachabilityFlagsConnectionOnDemand,
        kSCNetworkReachabilityFlagsConnectionOnTraffic,
        kSCNetworkReachabilityFlagsConnectionRequired,
        kSCNetworkReachabilityFlagsInterventionRequired, kSCNetworkReachabilityFlagsIsDirect,
        kSCNetworkReachabilityFlagsIsLocalAddress, kSCNetworkReachabilityFlagsIsWWAN,
        kSCNetworkReachabilityFlagsReachable, kSCNetworkReachabilityFlagsTransientConnection,
        SCNetworkReachabilityContext, SCNetworkReachabilityCreateWithAddress,
        SCNetworkReachabilityCreateWithAddressPair, SCNetworkReachabilityCreateWithName,
        SCNetworkReachabilityFlags, SCNetworkReachabilityGetFlags, SCNetworkReachabilityGetTypeID,
        SCNetworkReachabilityRef, SCNetworkReachabilityScheduleWithRunLoop,
        SCNetworkReachabilitySetCallback, SCNetworkReachabilityUnscheduleFromRunLoop,
    },
};

use std::{
    error::Error,
    ffi::{c_void, CStr},
    fmt::{self, Display},
    net::SocketAddr,
    ptr,
    sync::Arc,
};

/// Failure to determine reachability
#[derive(Debug)]
pub enum ReachabilityError {
    /// `SCNetworkReachabilityGetFlags` call failed.
    FailedToDetermineReachability,
    ///  `SCNetworkReachabilityGetFlags` call returned unrecognized flags.
    UnrecognizedFlags(u32),
}

/// Failure to schedule a reachability callback on a runloop.
#[derive(Debug)]
pub struct SchedulingError(());

impl Display for SchedulingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to schedule a reachability callback on a runloop")
    }
}

impl Error for SchedulingError {}

/// Failure to unschedule a reachability callback on a runloop.
#[derive(Debug)]
pub struct UnschedulingError(());

impl Display for UnschedulingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to unschedule a reachability callback on a runloop"
        )
    }
}

impl Error for UnschedulingError {}

/// Failure to set a callback for changes in reachability.
#[derive(Debug)]
pub struct SetCallbackError {}

impl Display for SetCallbackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to set a callback for reachability")
    }
}

impl Error for SetCallbackError {}

bitflags::bitflags! {
    /// Rustier interface for [`SCNetworkReachabilityFlags`].
    ///
    /// [`SCNetworkReachability`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkreachabilityflags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ReachabilityFlags: u32 {
        /// The specified node name or address can be reached via a transient connection, such as
        /// PPP.
        const TRANSIENT_CONNECTION = kSCNetworkReachabilityFlagsTransientConnection;
        /// The specified node name or address can be reached using the current network
        /// configuration.
        const REACHABLE = kSCNetworkReachabilityFlagsReachable;
        /// The specified node name or address can be reached using the current network
        /// configuration, but a connection must first be established. If this flag is set, the
        /// `CONNECTION_ON_TRAFFIC` flag, `CONNECTION_ON_DEMAND` flag, or `IS_WANN` flag is also
        /// typically set to indicate the type of connection required. If the user must manually
        /// make the connection, the `INTERVENTION_REQUIRED` flag is also set.
        const CONNECTION_REQUIRED = kSCNetworkReachabilityFlagsConnectionRequired;
        /// The specified node name or address can be reached using the current network
        /// configuration, but a connection must first be established. Any traffic directed to the
        /// specified name or address will initiate the connection.
        const CONNECTION_ON_TRAFFIC = kSCNetworkReachabilityFlagsConnectionOnTraffic;
        /// The specified node name or address can be reached using the current network
        /// configuration, but a connection must first be established.
        const INTERVENTION_REQUIRED = kSCNetworkReachabilityFlagsInterventionRequired;
        /// The specified node name or address can be reached using the current network
        /// configuration, but a connection must first be established.
        const CONNECTION_ON_DEMAND = kSCNetworkReachabilityFlagsConnectionOnDemand;
        /// The specified node name or address is one that is associated with a network interface on the current system.
        const IS_LOCAL_ADDRESS = kSCNetworkReachabilityFlagsIsLocalAddress;
        /// Network traffic to the specified node name or address will not go through a gateway, but
        /// is routed directly to one of the interfaces in the system.
        const IS_DIRECT = kSCNetworkReachabilityFlagsIsDirect;
        /// The specified node name or address can be reached via a cellular connection, such as EDGE or GPRS.
        const IS_WWAN = kSCNetworkReachabilityFlagsIsWWAN;
    }
}

core_foundation::declare_TCFType!(
    /// A network address or host for which the connectivity can be determined.
    ///
    /// See [`SCNetworkReachability`]  for details.
    ///
    /// [`SCNetworkReachability`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkreachability-g7d
    SCNetworkReachability,
    SCNetworkReachabilityRef
);

core_foundation::impl_TCFType!(
    SCNetworkReachability,
    SCNetworkReachabilityRef,
    SCNetworkReachabilityGetTypeID
);

impl SCNetworkReachability {
    /// Construct a SCNetworkReachability struct with a local and a remote socket address.
    ///
    /// See [`SCNetworkReachabilityCreateWithAddressPair`] for details.
    ///
    /// [`SCNetworkReachabilityCreateWithAddressPair`]: https://developer.apple.com/documentation/systemconfiguration/1514908-scnetworkreachabilitycreatewitha?language=objc
    pub fn from_addr_pair(local: SocketAddr, remote: SocketAddr) -> SCNetworkReachability {
        let ptr = unsafe {
            SCNetworkReachabilityCreateWithAddressPair(
                std::ptr::null(),
                &*to_c_sockaddr(local),
                &*to_c_sockaddr(remote),
            )
        };

        unsafe { Self::wrap_under_create_rule(ptr) }
    }

    /// Construct a Reachability from either a hostname or a network node
    ///
    /// See [`SCNetworkReachabilityCreateWithName`] for details.
    ///
    /// [`SCNetworkReachabilityCreateWithName`]: https://developer.apple.com/documentation/systemconfiguration/1514904-scnetworkreachabilitycreatewithn?language=objc
    pub fn from_host(host: &CStr) -> Option<Self> {
        let ptr = unsafe { SCNetworkReachabilityCreateWithName(ptr::null(), host.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(Self::wrap_under_create_rule(ptr)) }
        }
    }

    /// Return a flag indicating whether the specified network address is reachable.
    ///
    /// See [`SCNetworkReachabilityGetFlags`] for details.
    ///
    /// [`SCNetworkReachabilityGetFlags`]: https://developer.apple.com/documentation/systemconfiguration/1514924-scnetworkreachabilitygetflags?language=objc
    pub fn reachability(&self) -> Result<ReachabilityFlags, ReachabilityError> {
        let mut raw_flags = 0u32;
        if unsafe { SCNetworkReachabilityGetFlags(self.0, &mut raw_flags) } == 0u8 {
            return Err(ReachabilityError::FailedToDetermineReachability);
        }

        ReachabilityFlags::from_bits(raw_flags)
            .ok_or(ReachabilityError::UnrecognizedFlags(raw_flags))
    }

    /// Schedule callback with runloop.
    ///
    /// See [`SCNetworkReachabilityScheduleFromRunLoop`] for details.
    ///
    /// [`SCNetworkReachabilityScheduleFromRunLoop`]: https://developer.apple.com/documentation/systemconfiguration/1514894-scnetworkreachabilityschedulewit?language=objc
    ///
    /// # Safety
    ///
    /// The `run_loop_mode` must not be NULL and must be a pointer to a valid run loop mode.
    /// Use `core_foundation::runloop::kCFRunLoopCommonModes` if you are unsure.
    pub unsafe fn schedule_with_runloop(
        &self,
        run_loop: &CFRunLoop,
        run_loop_mode: CFStringRef,
    ) -> Result<(), SchedulingError> {
        if SCNetworkReachabilityScheduleWithRunLoop(
            self.0,
            run_loop.to_void() as *mut _,
            run_loop_mode,
        ) == 0u8
        {
            Err(SchedulingError(()))
        } else {
            Ok(())
        }
    }

    /// Unschedule from run loop.
    ///
    /// See [`SCNetworkReachabilityUnscheduleFromRunLoop`] for details.
    ///
    /// [`SCNetworkReachabilityUnscheduleFromRunLoop`]: https://developer.apple.com/documentation/systemconfiguration/1514899-scnetworkreachabilityunschedulef?language=objc
    ///
    /// # Safety
    ///
    /// The `run_loop_mode` must not be NULL and must be a pointer to a valid run loop mode.
    /// Use `core_foundation::runloop::kCFRunLoopCommonModes` if you are unsure.
    pub unsafe fn unschedule_from_runloop(
        &self,
        run_loop: &CFRunLoop,
        run_loop_mode: CFStringRef,
    ) -> Result<(), UnschedulingError> {
        if SCNetworkReachabilityUnscheduleFromRunLoop(
            self.0,
            run_loop.to_void() as *mut _,
            run_loop_mode,
        ) == 0u8
        {
            Err(UnschedulingError(()))
        } else {
            Ok(())
        }
    }

    /// Sets callback that is run whenever network connectivity changes. For the callback to be
    /// invoked, the `SCNetworkReachability` has to be registered on a run loop. Calling this
    /// function multiple times will clear the subsequently set callback.
    ///
    /// See [`SCNetworkReachabilityContext`] for details.
    ///
    /// [`SCNetworkReachabilityContext`]: https://developer.apple.com/documentation/systemconfiguration/1514903-scnetworkreachabilitysetcallback?language=objc
    pub fn set_callback<F: Fn(ReachabilityFlags) + Sync + Send>(
        &mut self,
        callback: F,
    ) -> Result<(), SetCallbackError> {
        let callback = Arc::new(NetworkReachabilityCallbackContext::new(
            self.clone(),
            callback,
        ));

        let mut callback_context = SCNetworkReachabilityContext {
            version: 0,
            info: Arc::into_raw(callback) as *mut _,
            retain: Some(NetworkReachabilityCallbackContext::<F>::retain_context),
            release: Some(NetworkReachabilityCallbackContext::<F>::release_context),
            copyDescription: Some(NetworkReachabilityCallbackContext::<F>::copy_ctx_description),
        };

        let result = unsafe {
            SCNetworkReachabilitySetCallback(
                self.0,
                Some(NetworkReachabilityCallbackContext::<F>::callback),
                &mut callback_context,
            )
        };

        // The call to SCNetworkReachabilitySetCallback will call the
        // `retain` callback which will increment the reference count on
        // `callback`. Therefore, although the count is decremented below,
        // the reference count will still be >0.
        //
        // When `SCNetworkReachability` is dropped, `release` is called
        // which will drop the reference count on `callback` to 0.
        //
        // Assumes the pointer pointed to by the `info` member of `callback_context` is still valid.
        unsafe { Arc::decrement_strong_count(callback_context.info) };

        if result == 0u8 {
            Err(SetCallbackError {})
        } else {
            Ok(())
        }
    }
}

impl From<SocketAddr> for SCNetworkReachability {
    fn from(addr: SocketAddr) -> Self {
        unsafe {
            let ptr =
                SCNetworkReachabilityCreateWithAddress(std::ptr::null(), &*to_c_sockaddr(addr));
            SCNetworkReachability::wrap_under_create_rule(ptr)
        }
    }
}

struct NetworkReachabilityCallbackContext<T: Fn(ReachabilityFlags) + Sync + Send> {
    _host: SCNetworkReachability,
    callback: T,
}

impl<T: Fn(ReachabilityFlags) + Sync + Send> NetworkReachabilityCallbackContext<T> {
    fn new(host: SCNetworkReachability, callback: T) -> Self {
        Self {
            _host: host,
            callback,
        }
    }

    extern "C" fn callback(
        _target: SCNetworkReachabilityRef,
        flags: SCNetworkReachabilityFlags,
        context: *mut c_void,
    ) {
        let context: &mut Self = unsafe { &mut (*(context as *mut _)) };
        (context.callback)(ReachabilityFlags::from_bits_retain(flags));
    }

    extern "C" fn copy_ctx_description(_ctx: *const c_void) -> CFStringRef {
        let description = CFString::from_static_string("NetworkRechability's callback context");
        let description_ref = description.as_concrete_TypeRef();
        std::mem::forget(description);
        description_ref
    }

    extern "C" fn release_context(ctx: *const c_void) {
        unsafe {
            Arc::decrement_strong_count(ctx as *mut Self);
        }
    }

    extern "C" fn retain_context(ctx_ptr: *const c_void) -> *const c_void {
        unsafe {
            Arc::increment_strong_count(ctx_ptr as *mut Self);
        }
        ctx_ptr
    }
}

/// Allocates a libc::sockaddr compatible struct and fills it with either a libc::sockaddr_in or a
/// libc::sockaddr_in6, depending on the passed in standard library SocketAddr.
fn to_c_sockaddr(addr: SocketAddr) -> Box<libc::sockaddr> {
    let ptr = match addr {
        // See reference conversion from socket2:
        // https://github.com/rust-lang/socket2/blob/3a938932829ea6ee3025d2d7a86c7b095c76e6c3/src/sockaddr.rs#L277-L287
        // https://github.com/rust-lang/socket2/blob/3a938932829ea6ee3025d2d7a86c7b095c76e6c3/src/sys/unix.rs#L1356-L1363
        SocketAddr::V4(addr) => Box::into_raw(Box::new(libc::sockaddr_in {
            sin_len: std::mem::size_of::<libc::sockaddr_in>() as u8,
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: addr.port().to_be(),
            sin_addr: {
                // `s_addr` is stored as BE on all machines, and the array is in BE order.
                // So the native endian conversion method is used so that it's never
                // swapped.
                libc::in_addr {
                    s_addr: u32::from_ne_bytes(addr.ip().octets()),
                }
            },
            sin_zero: Default::default(),
        })) as *mut c_void,
        // See reference conversion from socket2:
        // https://github.com/rust-lang/socket2/blob/3a938932829ea6ee3025d2d7a86c7b095c76e6c3/src/sockaddr.rs#L314-L331
        // https://github.com/rust-lang/socket2/blob/3a938932829ea6ee3025d2d7a86c7b095c76e6c3/src/sys/unix.rs#L1369-L1373
        SocketAddr::V6(addr) => Box::into_raw(Box::new(libc::sockaddr_in6 {
            sin6_len: std::mem::size_of::<libc::sockaddr_in6>() as u8,
            sin6_family: libc::AF_INET6 as libc::sa_family_t,
            sin6_port: addr.port().to_be(),
            sin6_flowinfo: addr.flowinfo(),
            sin6_addr: libc::in6_addr {
                s6_addr: addr.ip().octets(),
            },
            sin6_scope_id: addr.scope_id(),
        })) as *mut c_void,
    };

    unsafe { Box::from_raw(ptr as *mut _) }
}

#[cfg(test)]
mod test {
    use super::*;

    use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
    use std::{
        ffi::CString,
        net::{Ipv4Addr, Ipv6Addr},
    };

    #[test]
    fn test_network_reachability_from_addr() {
        let sockaddrs = vec![
            "0.0.0.0:0".parse::<SocketAddr>().unwrap(),
            "[::0]:0".parse::<SocketAddr>().unwrap(),
        ];

        for addr in sockaddrs {
            let mut reachability = SCNetworkReachability::from(addr);
            assert!(
                !reachability.0.is_null(),
                "Failed to construct a SCNetworkReachability struct with {}",
                addr
            );
            reachability.set_callback(|_| {}).unwrap();
            // SAFETY: We use the Apple provided run_loop_mode kCFRunLoopCommonModes
            unsafe {
                reachability
                    .schedule_with_runloop(&CFRunLoop::get_current(), kCFRunLoopCommonModes)
                    .unwrap();
                reachability
                    .unschedule_from_runloop(&CFRunLoop::get_current(), kCFRunLoopCommonModes)
                    .unwrap();
            }
        }
    }

    #[test]
    fn test_sockaddr_pair_reachability() {
        let pairs = vec![
            ("0.0.0.0:0", "[::0]:0"),
            ("[::0]:0", "0.0.0.0:0"),
            ("[::0]:0", "[::0]:0"),
            ("0.0.0.0:0", "0.0.0.0:0"),
        ]
        .into_iter()
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()));

        for (local, remote) in pairs {
            let mut reachability = SCNetworkReachability::from_addr_pair(local, remote);
            assert!(
                !reachability.0.is_null(),
                "Failed to construct a SCNetworkReachability struct with address pair {} - {}",
                local,
                remote
            );
            reachability.set_callback(|_| {}).unwrap();
            // SAFETY: We use the Apple provided run_loop_mode kCFRunLoopCommonModes
            unsafe {
                reachability
                    .schedule_with_runloop(&CFRunLoop::get_current(), kCFRunLoopCommonModes)
                    .unwrap();
                reachability
                    .unschedule_from_runloop(&CFRunLoop::get_current(), kCFRunLoopCommonModes)
                    .unwrap();
            }
        }
    }

    #[test]
    fn test_sockaddr_local_to_dns_google_pair_reachability() {
        let sockaddrs = [
            "[2001:4860:4860::8844]:443".parse::<SocketAddr>().unwrap(),
            "8.8.4.4:443".parse().unwrap(),
        ];
        for remote_addr in sockaddrs {
            match std::net::TcpStream::connect(remote_addr) {
                Err(_) => {
                    let local_addr = if remote_addr.is_ipv4() {
                        SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0)
                    } else {
                        SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 0)
                    };
                    let reachability =
                        SCNetworkReachability::from_addr_pair(local_addr, remote_addr);
                    let reachability_flags = reachability.reachability().unwrap();
                    // Verify that not established tcp connection path is reported as not reachable.
                    assert!(!reachability_flags.contains(ReachabilityFlags::REACHABLE));
                }
                Ok(tcp) => {
                    let local = tcp.local_addr().unwrap();
                    let remote = tcp.peer_addr().unwrap();
                    let reachability = SCNetworkReachability::from_addr_pair(local, remote);
                    let reachability_flags = reachability.reachability().unwrap();
                    // Verify established tcp connection path is reported as reachable.
                    assert!(reachability_flags.contains(ReachabilityFlags::REACHABLE));
                }
            }
        }
    }

    #[test]
    fn test_reachability_ref_from_host() {
        let valid_inputs = vec!["example.com", "host-in-local-network", "en0"];

        let get_cstring = |input: &str| CString::new(input).unwrap();

        for input in valid_inputs.into_iter().map(get_cstring) {
            match SCNetworkReachability::from_host(&input) {
                Some(mut reachability) => {
                    reachability.set_callback(|_| {}).unwrap();
                    // SAFETY: We use the Apple provided run_loop_mode kCFRunLoopCommonModes
                    unsafe {
                        reachability
                            .schedule_with_runloop(&CFRunLoop::get_current(), kCFRunLoopCommonModes)
                            .unwrap();
                        reachability
                            .unschedule_from_runloop(
                                &CFRunLoop::get_current(),
                                kCFRunLoopCommonModes,
                            )
                            .unwrap();
                    }
                }
                None => {
                    panic!(
                        "Failed to construct a SCNetworkReachability from {}",
                        input.to_string_lossy(),
                    );
                }
            }
        }

        // Can only testify that an empty string is invalid, everything else seems to work
        assert!(
            SCNetworkReachability::from_host(&get_cstring("")).is_none(),
            "Constructed valid SCNetworkReachability from empty string"
        );
    }

    unsafe impl Send for SCNetworkReachability {}

    #[test]
    fn assert_infallibility_of_setting_a_callback() {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut reachability =
                SCNetworkReachability::from("0.0.0.0:0".parse::<SocketAddr>().unwrap());
            reachability.set_callback(|_| {}).unwrap();
            // SAFETY: We use the Apple provided run_loop_mode kCFRunLoopCommonModes
            unsafe {
                reachability
                    .schedule_with_runloop(&CFRunLoop::get_current(), kCFRunLoopCommonModes)
                    .unwrap();
            }
            reachability.set_callback(|_| {}).unwrap();
            let _ = tx.send(reachability);
            CFRunLoop::run_current();
        });
        let mut reachability = rx.recv().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        reachability.set_callback(|_| {}).unwrap();
    }
}
