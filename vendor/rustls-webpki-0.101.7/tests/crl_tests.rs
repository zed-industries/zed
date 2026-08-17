use webpki::{BorrowedCertRevocationList, CertRevocationList, Error};

const REVOKED_SERIAL: &[u8] = &[0x03, 0xAE, 0x51, 0xDB, 0x51, 0x15, 0x5A, 0x3C];

const REVOKED_SERIAL_NEGATIVE: &[u8] = &[0xfd, 0x78, 0xa8, 0x4e];

// because cert::Cert::serial is the raw DER integer encoding, this includes a leading
// zero if one is required to make the twos complement representation positive.  in this case
// one is required.
const REVOKED_SERIAL_WITH_TOP_BIT_SET: &[u8] = &[0x00, 0x80, 0xfe, 0xed, 0xf0, 0x0d];

#[test]
fn parse_valid_crl() {
    // We should be able to parse a valid CRL without error, and find the revoked serial.
    let crl = include_bytes!("crls/crl.valid.der");
    let crl = BorrowedCertRevocationList::from_der(&crl[..]).expect("failed to parse valid crl");
    assert!(crl.find_serial(REVOKED_SERIAL).unwrap().is_some());

    #[cfg(feature = "alloc")]
    {
        let crl = crl.to_owned().unwrap();
        assert!(crl.find_serial(REVOKED_SERIAL).unwrap().is_some());
    }
}

#[test]
fn parse_empty_crl() {
    // We should be able to parse an empty CRL without error, and find no revoked certs.
    let crl = include_bytes!("crls/crl.empty.der");
    let crl = BorrowedCertRevocationList::from_der(&crl[..]).expect("failed to parse empty crl");
    assert!(crl.into_iter().next().is_none());

    #[cfg(feature = "alloc")]
    {
        // We should also be able to create an owned empty CRL without error.
        let res = crl.to_owned();
        assert!(res.is_ok());
    }
}

#[test]
fn parse_mismatched_sigalg_crl() {
    // Parsing a CRL with a mismatched outer/inner signature algorithm should fail.
    let crl = include_bytes!("crls/crl.mismatched.sigalg.der");
    let res = BorrowedCertRevocationList::from_der(&crl[..]);
    assert!(matches!(res, Err(Error::SignatureAlgorithmMismatch)));
}

#[test]
fn parse_bad_this_update_crl() {
    // Parsing a CRL with an invalid this update time should error.
    let crl = include_bytes!("crls/crl.invalid.this.update.time.der");
    let res = BorrowedCertRevocationList::from_der(&crl[..]);
    assert!(matches!(res, Err(Error::BadDerTime)));
}

#[test]
fn parse_missing_next_update_crl() {
    // Parsing a CRL with a missing next update time should error.
    let crl = include_bytes!("crls/crl.missing.next.update.der");
    let res = BorrowedCertRevocationList::from_der(&crl[..]);
    assert!(matches!(res, Err(Error::BadDer)));
}

#[test]
fn parse_wrong_version_crl() {
    // Parsing a CRL with an unsupported version should error.
    let crl = include_bytes!("crls/crl.wrong.version.der");
    let res = BorrowedCertRevocationList::from_der(&crl[..]);
    assert!(matches!(res, Err(Error::UnsupportedCrlVersion)));
}

#[test]
fn parse_missing_exts_crl() {
    // Parsing a CRL with no list extensions should error.
    let crl = include_bytes!("crls/crl.missing.exts.der");
    let res = BorrowedCertRevocationList::from_der(&crl[..]);
    assert!(matches!(res, Err(Error::MalformedExtensions)));
}

#[test]
fn parse_delta_crl() {
    // Parsing a CRL with an extension indicating its a delta CRL should error.
    let crl = include_bytes!("crls/crl.delta.der");
    let res = BorrowedCertRevocationList::from_der(&crl[..]);
    assert!(matches!(res, Err(Error::UnsupportedDeltaCrl)));
}

#[test]
fn parse_unknown_crit_ext_crl() {
    // Parsing a CRL with an unknown critical list extension should error.
    let crl = include_bytes!("crls/crl.unknown.crit.ext.der");
    let res = BorrowedCertRevocationList::from_der(&crl[..]);
    assert!(matches!(res, Err(Error::UnsupportedCriticalExtension)));
}

#[test]
fn parse_negative_crl_number_crl() {
    // Parsing a CRL with a negative CRL number should error.
    let crl = include_bytes!("crls/crl.negative.crl.number.der");
    let res = BorrowedCertRevocationList::from_der(&crl[..]);
    assert!(matches!(res, Err(Error::InvalidCrlNumber)));
}

#[test]
fn parse_too_long_crl_number_crl() {
    // Parsing a CRL with a CRL number > 20 octets should error.
    let crl = include_bytes!("crls/crl.too.long.crl.number.der");
    let res = BorrowedCertRevocationList::from_der(&crl[..]);
    assert!(matches!(res, Err(Error::InvalidCrlNumber)));
}

#[test]
fn parse_entry_negative_serial_crl() {
    let crl = include_bytes!("crls/crl.negative.serial.der");
    let crl = BorrowedCertRevocationList::from_der(&crl[..]).unwrap();

    assert!(crl
        .find_serial(REVOKED_SERIAL)
        .expect("looking for REVOKED_SERIAL failed")
        .is_none());
    assert!(crl
        .find_serial(REVOKED_SERIAL_NEGATIVE)
        .expect("looking for REVOKED_SERIAL_NEGATIVE failed")
        .is_some());

    #[cfg(feature = "alloc")]
    {
        let crl = crl.to_owned().unwrap();
        assert!(crl
            .find_serial(REVOKED_SERIAL)
            .expect("looking for REVOKED_SERIAL failed")
            .is_none());
        assert!(crl
            .find_serial(REVOKED_SERIAL_NEGATIVE)
            .expect("looking for REVOKED_SERIAL_NEGATIVE failed")
            .is_some());
    }
}

#[test]
fn parse_entry_topbit_serial_crl() {
    let crl = include_bytes!("crls/crl.topbit.serial.der");
    let crl = BorrowedCertRevocationList::from_der(&crl[..]).unwrap();

    assert!(crl
        .find_serial(REVOKED_SERIAL_WITH_TOP_BIT_SET)
        .expect("failed to look for REVOKED_SERIAL_WITH_TOP_BIT_SET")
        .is_some());

    #[cfg(feature = "alloc")]
    {
        let crl = crl.to_owned().unwrap();
        assert!(crl
            .find_serial(REVOKED_SERIAL_WITH_TOP_BIT_SET)
            .expect("failed to look for REVOKED_SERIAL_WITH_TOP_BIT_SET")
            .is_some());
    }
}

#[test]
fn parse_entry_without_exts_crl() {
    // Parsing a CRL that includes a revoked entry that has no extensions shouldn't error, and we
    // should find the expected revoked certificate.
    let crl = include_bytes!("crls/crl.no.entry.exts.der");
    let crl = BorrowedCertRevocationList::from_der(&crl[..]).expect("unexpected error parsing crl");
    assert!(crl.find_serial(REVOKED_SERIAL).unwrap().is_some());

    #[cfg(feature = "alloc")]
    {
        let crl = crl.to_owned().unwrap();
        assert!(crl.find_serial(REVOKED_SERIAL).unwrap().is_some());
    }
}

#[test]
fn parse_entry_with_empty_exts_seq() {
    // Parsing a CRL that has a revoked cert entry with an empty extensions sequence shouldn't error.
    let crl = include_bytes!("crls/crl.entry.empty.ext.seq.der");
    let res = BorrowedCertRevocationList::from_der(&crl[..]);
    assert!(res.is_ok());

    #[cfg(feature = "alloc")]
    {
        let res = res.unwrap().to_owned();
        assert!(res.is_ok());
    }
}

#[test]
fn parse_entry_unknown_crit_ext_crl() {
    // Parsing a CRL that includes a revoked entry that has an unknown critical extension shouldn't
    // error up-front because the problem is with a revoked cert entry.
    let crl = include_bytes!("crls/crl.entry.unknown.crit.ext.der");
    let crl = BorrowedCertRevocationList::from_der(&crl[..]).unwrap();

    // but should error when we try to find a revoked serial due to the entry with the unsupported
    // critical ext.
    let res = crl.find_serial(REVOKED_SERIAL);
    assert!(matches!(res, Err(Error::UnsupportedCriticalExtension)));

    #[cfg(feature = "alloc")]
    {
        // Parsing the CRL as an owned CRL should error since it will process the revoked certs.
        let res = crl.to_owned();
        assert!(matches!(res, Err(Error::UnsupportedCriticalExtension)));
    }
}

#[test]
fn parse_entry_invalid_reason_crl() {
    // Parsing a CRL that includes a revoked entry that has an unknown revocation reason shouldn't
    // error up-front since the problem is with a revoked entry.
    let crl = include_bytes!("crls/crl.entry.invalid.reason.der");
    let crl = BorrowedCertRevocationList::from_der(&crl[..]).unwrap();

    // But searching for a serial should error due to the revoked cert with the unknown reason.
    let res = crl.find_serial(REVOKED_SERIAL);
    assert!(matches!(res, Err(Error::UnsupportedRevocationReason)));

    #[cfg(feature = "alloc")]
    {
        // Parsing the CRL as an owned CRL should error since it will process the revoked certs.
        let res = crl.to_owned();
        assert!(matches!(res, Err(Error::UnsupportedRevocationReason)));
    }
}

#[test]
fn parse_entry_invalidity_date_crl() {
    // Parsing a CRL that includes a revoked entry that has an invalidity date ext shouldn't error
    // and we should find the expected revoked cert with an invalidity date.
    let crl = include_bytes!("crls/crl.entry.invalidity.date.der");
    let crl = BorrowedCertRevocationList::from_der(&crl[..]).expect("unexpected err parsing CRL");
    assert!(crl
        .find_serial(REVOKED_SERIAL)
        .unwrap()
        .unwrap()
        .invalidity_date
        .is_some());

    #[cfg(feature = "alloc")]
    {
        let crl = crl.to_owned().unwrap();
        assert!(crl
            .find_serial(REVOKED_SERIAL)
            .unwrap()
            .unwrap()
            .invalidity_date
            .is_some());
    }
}

#[test]
fn parse_entry_indirect_issuer_crl() {
    // Parsing a CRL that includes a revoked entry that has a issuer certificate extension
    // shouldn't error up-front - we expect the error to be surfaced when we iterate the revoked
    // certs.
    let crl = include_bytes!("crls/crl.entry.issuer.ext.der");
    let crl = BorrowedCertRevocationList::from_der(&crl[..]).unwrap();

    let res = crl.find_serial(REVOKED_SERIAL);
    assert!(matches!(res, Err(Error::UnsupportedIndirectCrl)));

    #[cfg(feature = "alloc")]
    {
        // Building an owned CRL should error up front since it will process the revoked certs.
        let res = crl.to_owned();
        assert!(matches!(res, Err(Error::UnsupportedIndirectCrl)));
    }
}
