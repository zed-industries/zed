//! OSX specific functionality for items.
use crate::item::ItemSearchOptions;
use crate::os::macos::keychain::SecKeychain;

// Moved to crate::Key
pub use crate::key::KeyType;

// TODO: mark as deprecated
#[doc(hidden)]
/// An obsolete trait for `ItemSearchOptions`. Use methods on `ItemSearchOptions` directly.
pub trait ItemSearchOptionsExt {
    /// Search within the specified keychains.
    ///
    /// If this is not called, the default keychain will be searched.
    fn keychains(&mut self, keychains: &[SecKeychain]) -> &mut Self;

    // Do not extend this trait; use `impl ItemSearchOptions` directly
}

impl ItemSearchOptionsExt for ItemSearchOptions {
    fn keychains(&mut self, keychains: &[SecKeychain]) -> &mut Self {
        Self::keychains(self, keychains)
    }

    // Do not extend this trait; use `impl ItemSearchOptions` directly
}

#[cfg(test)]
mod test {
    use crate::item::*;
    use crate::os::macos::certificate::SecCertificateExt;
    use crate::os::macos::test::keychain;
    use tempfile::tempdir;

    #[test]
    fn find_certificate() {
        let dir = p!(tempdir());
        let keychain = keychain(dir.path());
        let results = p!(ItemSearchOptions::new()
            .keychains(&[keychain])
            .class(ItemClass::certificate())
            .search());
        assert_eq!(1, results.len());
        let SearchResult::Ref(Reference::Certificate(certificate)) = &results[0] else {
            panic!("expected certificate")
        };
        assert_eq!("foobar.com", p!(certificate.common_name()));
    }
}
