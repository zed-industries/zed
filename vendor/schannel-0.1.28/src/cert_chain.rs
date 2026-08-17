//! Bindings to winapi's certificate-chain related APIs.

use std::mem;
use std::slice;

use windows_sys::Win32::Security::Cryptography;

use crate::cert_context::CertContext;
use crate::Inner;

/// A certificate chain context (consisting of multiple chains)
pub struct CertChainContext(pub(crate) *mut Cryptography::CERT_CHAIN_CONTEXT);
inner!(CertChainContext, *mut Cryptography::CERT_CHAIN_CONTEXT);

unsafe impl Sync for CertChainContext {}
unsafe impl Send for CertChainContext {}

impl Clone for CertChainContext {
    fn clone(&self) -> Self {
        let rced = unsafe { Cryptography::CertDuplicateCertificateChain(self.0) };
        CertChainContext(rced)
    }
}

impl Drop for CertChainContext {
    fn drop(&mut self) {
        unsafe {
            Cryptography::CertFreeCertificateChain(self.0);
        }
    }
}

impl CertChainContext {
    /// Get the final (for a successful verification this means successful) certificate chain
    ///
    /// https://msdn.microsoft.com/de-de/library/windows/desktop/aa377182(v=vs.85).aspx
    /// rgpChain[cChain - 1] is the final chain
    pub fn final_chain(&self) -> Option<CertChain> {
        if let Some(chain) = self.chains().last() {
            return Some(CertChain(chain.0, self.clone()));
        }
        None
    }

    /// Retrieves the specified chain from the context.
    pub fn get_chain(&self, index: usize) -> Option<CertChain> {
        let cert_chain = unsafe {
            let cert_chain = *self.0;
            if index >= cert_chain.cChain as usize {
                None
            } else {
                let chain_slice =
                    slice::from_raw_parts(cert_chain.rgpChain, cert_chain.cChain as usize);
                Some(CertChain(chain_slice[index], self.clone()))
            }
        };
        cert_chain
    }

    /// Return an iterator over all certificate chains in this context
    pub fn chains(&self) -> CertificateChains {
        CertificateChains {
            context: self,
            idx: 0,
        }
    }
}

/// A (simple) certificate chain
pub struct CertChain(*mut Cryptography::CERT_SIMPLE_CHAIN, CertChainContext);

impl CertChain {
    /// Returns the number of certificates in the chain
    pub fn len(&self) -> usize {
        unsafe { (*self.0).cElement as usize }
    }

    /// Returns true if there are no certificates in the chain
    pub fn is_empty(&self) -> bool {
        unsafe { (*self.0).cElement == 0 }
    }

    /// Get the n-th certificate from the current chain
    pub fn get(&self, idx: usize) -> Option<CertContext> {
        let elements = unsafe {
            let cert_chain = *self.0;
            slice::from_raw_parts(
                cert_chain.rgpElement as *mut &mut Cryptography::CERT_CHAIN_ELEMENT,
                cert_chain.cElement as usize,
            )
        };
        elements.get(idx).map(|el| {
            let cert = unsafe { CertContext::from_inner(el.pCertContext) };
            let rc_cert = cert.clone();
            mem::forget(cert);
            rc_cert
        })
    }

    /// Return an iterator over all certificates in this chain
    pub fn certificates(&self) -> Certificates {
        Certificates {
            chain: self,
            idx: 0,
        }
    }
}

/// An iterator that iterates over all chains in a context
pub struct CertificateChains<'a> {
    context: &'a CertChainContext,
    idx: usize,
}

impl<'a> Iterator for CertificateChains<'a> {
    type Item = CertChain;

    fn next(&mut self) -> Option<CertChain> {
        let idx = self.idx;
        self.idx += 1;
        self.context.get_chain(idx)
    }
}

/// An iterator that iterates over all certificates in a chain
pub struct Certificates<'a> {
    chain: &'a CertChain,
    idx: usize,
}

impl<'a> Iterator for Certificates<'a> {
    type Item = CertContext;

    fn next(&mut self) -> Option<CertContext> {
        let idx = self.idx;
        self.idx += 1;
        self.chain.get(idx)
    }
}
