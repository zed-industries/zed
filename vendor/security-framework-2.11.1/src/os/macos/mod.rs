//! OSX specific extensions.

pub mod access;
pub mod certificate;
pub mod certificate_oids;
pub mod code_signing;
pub mod digest_transform;
pub mod encrypt_transform;
pub mod identity;
pub mod import_export;
pub mod item;
pub mod key;
pub mod keychain;
pub mod keychain_item;
pub mod passwords;
pub mod secure_transport;
pub mod transform;

#[cfg(test)]
pub mod test {
    use crate::identity::SecIdentity;
    use crate::item::{ItemClass, ItemSearchOptions, Reference, SearchResult};
    use crate::os::macos::item::ItemSearchOptionsExt;
    use crate::os::macos::keychain::SecKeychain;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    #[must_use] pub fn identity(dir: &Path) -> SecIdentity {
        // FIXME https://github.com/rust-lang/rust/issues/30018
        let keychain = keychain(dir);
        let mut items = p!(ItemSearchOptions::new()
            .class(ItemClass::identity())
            .keychains(&[keychain])
            .search());
        match items.pop().unwrap() {
            SearchResult::Ref(Reference::Identity(identity)) => identity,
            _ => panic!("expected identity"),
        }
    }

    #[must_use] pub fn keychain(dir: &Path) -> SecKeychain {
        let path = dir.join("server.keychain");
        let mut file = p!(File::create(&path));
        p!(file.write_all(include_bytes!("../../../test/server.keychain")));
        drop(file);

        let mut keychain = p!(SecKeychain::open(&path));
        p!(keychain.unlock(Some("password123")));
        keychain
    }
}
