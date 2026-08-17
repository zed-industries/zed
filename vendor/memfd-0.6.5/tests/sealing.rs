extern crate memfd;
use std::iter::FromIterator;

#[test]
fn test_sealing_default() {
    let opts = memfd::MemfdOptions::default();
    let m0 = opts.create("default").unwrap();
    let sset = m0.seals().unwrap();
    let default = memfd::SealsHashSet::from_iter(vec![memfd::FileSeal::SealSeal]);
    assert_eq!(sset, default);
}

#[test]
fn test_sealing_unsealed() {
    let opts = memfd::MemfdOptions::default();
    let m0 = opts.allow_sealing(true).create("default").unwrap();
    let sset = m0.seals().unwrap();
    assert_eq!(sset.len(), 0);
}

#[test]
fn test_sealing_add() {
    let opts = memfd::MemfdOptions::default();
    let m0 = opts.allow_sealing(true).create("default").unwrap();
    let sset = m0.seals().unwrap();
    assert_eq!(sset.len(), 0);

    let write_seal = memfd::SealsHashSet::from_iter(vec![memfd::FileSeal::SealWrite]);
    m0.add_seal(memfd::FileSeal::SealWrite).unwrap();
    let a0 = write_seal;
    let r0 = m0.seals().unwrap();
    assert_eq!(r0, a0);

    let grow_seal = memfd::SealsHashSet::from_iter(vec![memfd::FileSeal::SealGrow]);
    m0.add_seals(&grow_seal).unwrap();
    let a1 = a0.union(&grow_seal).cloned().collect();
    let r1 = m0.seals().unwrap();
    assert_eq!(r1, a1);

    let shrink_seal = memfd::SealsHashSet::from_iter(vec![memfd::FileSeal::SealShrink]);
    m0.add_seals(&shrink_seal).unwrap();
    let a2 = a1.union(&shrink_seal).cloned().collect();
    let r2 = m0.seals().unwrap();
    assert_eq!(r2, a2);

    // `SealFutureWrite` is new as of Linux 5.1, so be prepared for it to fail
    // if we don't have it.
    let mut a3 = a2;
    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        let future_write_seal =
            memfd::SealsHashSet::from_iter(vec![memfd::FileSeal::SealFutureWrite]);
        if let Ok(()) = m0.add_seal(memfd::FileSeal::SealFutureWrite) {
            a3 = a3.union(&future_write_seal).cloned().collect();
            let r3 = m0.seals().unwrap();
            assert_eq!(r3, a3);
        }
    }

    let seal_seal = memfd::SealsHashSet::from_iter(vec![memfd::FileSeal::SealSeal]);
    m0.add_seals(&seal_seal).unwrap();
    let a4 = a3.union(&seal_seal).cloned().collect();
    let r4 = m0.seals().unwrap();
    assert_eq!(r4, a4);

    // memfd is "seal" sealed, adding further sealing should fail.
    m0.add_seals(&shrink_seal).unwrap_err();
}

#[test]
fn test_sealing_resize() {
    let opts = memfd::MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("sized-1K").unwrap();
    mfd.as_file().set_len(1024).unwrap();

    mfd.add_seal(memfd::FileSeal::SealGrow).unwrap();
    mfd.as_file().set_len(2048).unwrap_err();
    mfd.as_file().set_len(512).unwrap();

    mfd.add_seal(memfd::FileSeal::SealShrink).unwrap();
    mfd.as_file().set_len(1000).unwrap_err();
    mfd.as_file().set_len(1024).unwrap_err();
    mfd.as_file().set_len(256).unwrap_err();
    mfd.as_file().set_len(512).unwrap();
}
