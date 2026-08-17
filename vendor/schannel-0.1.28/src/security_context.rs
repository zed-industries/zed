use std::io;
use std::mem;
use std::ptr;

use windows_sys::Win32::Foundation;
use windows_sys::Win32::Security::Authentication::Identity;
use windows_sys::Win32::Security::Credentials;

use crate::alpn_list::AlpnList;
use crate::cert_context::CertContext;
use crate::context_buffer::ContextBuffer;
use crate::schannel_cred::SchannelCred;
use crate::{secbuf, secbuf_desc, Inner, INIT_REQUESTS};

pub struct SecurityContext(Credentials::SecHandle);

impl Drop for SecurityContext {
    fn drop(&mut self) {
        unsafe {
            Identity::DeleteSecurityContext(&self.0);
        }
    }
}

impl Inner<Credentials::SecHandle> for SecurityContext {
    unsafe fn from_inner(inner: Credentials::SecHandle) -> SecurityContext {
        SecurityContext(inner)
    }

    fn as_inner(&self) -> Credentials::SecHandle {
        self.0
    }

    fn get_mut(&mut self) -> &mut Credentials::SecHandle {
        &mut self.0
    }
}

impl SecurityContext {
    pub fn initialize(
        cred: &mut SchannelCred,
        accept: bool,
        domain: Option<&[u16]>,
        requested_application_protocols: &Option<Vec<Vec<u8>>>,
    ) -> io::Result<(SecurityContext, Option<ContextBuffer>)> {
        unsafe {
            let mut ctxt = mem::zeroed();

            if accept {
                // If we're performing an accept then we need to wait to call
                // `AcceptSecurityContext` until we've actually read some data.
                return Ok((SecurityContext(ctxt), None));
            }

            let domain = domain.map(|b| b.as_ptr()).unwrap_or(ptr::null_mut());

            let mut inbufs = vec![];

            // Make sure `AlpnList` is kept alive for the duration of this function.
            let mut alpns = requested_application_protocols
                .as_ref()
                .map(|alpn| AlpnList::new(alpn));
            if let Some(ref mut alpns) = alpns {
                inbufs.push(secbuf(
                    Identity::SECBUFFER_APPLICATION_PROTOCOLS,
                    Some(&mut alpns[..]),
                ));
            };

            let inbuf_desc = secbuf_desc(&mut inbufs[..]);

            let mut outbuf = [secbuf(Identity::SECBUFFER_EMPTY, None)];
            let mut outbuf_desc = secbuf_desc(&mut outbuf);

            let mut attributes = 0;

            match Identity::InitializeSecurityContextW(
                &cred.as_inner(),
                ptr::null_mut(),
                domain,
                INIT_REQUESTS,
                0,
                0,
                &inbuf_desc,
                0,
                &mut ctxt,
                &mut outbuf_desc,
                &mut attributes,
                ptr::null_mut(),
            ) {
                Foundation::SEC_I_CONTINUE_NEEDED => {
                    Ok((SecurityContext(ctxt), Some(ContextBuffer(outbuf[0]))))
                }
                err => Err(io::Error::from_raw_os_error(err)),
            }
        }
    }

    unsafe fn attribute<T>(&self, attr: Identity::SECPKG_ATTR) -> io::Result<T> {
        let mut value = mem::zeroed();
        let status =
            Identity::QueryContextAttributesW(&self.0, attr, &mut value as *mut _ as *mut _);
        match status {
            Foundation::SEC_E_OK => Ok(value),
            err => Err(io::Error::from_raw_os_error(err)),
        }
    }

    pub fn application_protocol(&self) -> io::Result<Identity::SecPkgContext_ApplicationProtocol> {
        unsafe { self.attribute(Identity::SECPKG_ATTR_APPLICATION_PROTOCOL) }
    }

    pub fn session_info(&self) -> io::Result<Identity::SecPkgContext_SessionInfo> {
        unsafe { self.attribute(Identity::SECPKG_ATTR_SESSION_INFO) }
    }

    pub fn stream_sizes(&self) -> io::Result<Identity::SecPkgContext_StreamSizes> {
        unsafe { self.attribute(Identity::SECPKG_ATTR_STREAM_SIZES) }
    }

    pub fn remote_cert(&self) -> io::Result<CertContext> {
        unsafe {
            self.attribute(Identity::SECPKG_ATTR_REMOTE_CERT_CONTEXT)
                .map(|p| CertContext::from_inner(p))
        }
    }

    pub fn local_cert(&self) -> io::Result<CertContext> {
        unsafe {
            self.attribute(Identity::SECPKG_ATTR_LOCAL_CERT_CONTEXT)
                .map(|p| CertContext::from_inner(p))
        }
    }
}
