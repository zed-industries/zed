//! OSX specific functionality for items.
use crate::item::ItemSearchOptions;
use crate::os::macos::keychain::SecKeychain;
use crate::ItemSearchOptionsInternals;

// Moved to crate::Key
pub use crate::key::KeyType;

/// An extension trait adding OSX specific functionality to `ItemSearchOptions`.
pub trait ItemSearchOptionsExt {
    /// Search within the specified keychains.
    ///
    /// If this is not called, the default keychain will be searched.
    fn keychains(&mut self, keychains: &[SecKeychain]) -> &mut Self;
}

impl ItemSearchOptionsExt for ItemSearchOptions {
    #[inline(always)]
    fn keychains(&mut self, keychains: &[SecKeychain]) -> &mut Self {
        ItemSearchOptionsInternals::keychains(self, keychains)
    }
}

#[cfg(test)]
mod test {
    use crate::item::*;
    use crate::os::macos::certificate::SecCertificateExt;
    use crate::os::macos::item::ItemSearchOptionsExt;
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
        let certificate = match results[0] {
            SearchResult::Ref(Reference::Certificate(ref cert)) => cert,
            _ => panic!("expected certificate"),
        };
        assert_eq!("foobar.com", p!(certificate.common_name()));
    }
}
