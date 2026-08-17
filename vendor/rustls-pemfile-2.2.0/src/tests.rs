#[cfg(test)]
mod unit {
    use alloc::{format, vec};
    use std::prelude::v1::*;

    use crate::{Error, Item};

    #[test]
    fn skips_leading_junk() {
        assert_eq!(
            check_both(
                b"junk\n\
                    -----BEGIN RSA PRIVATE KEY-----\n\
                    qw\n\
                    -----END RSA PRIVATE KEY-----\n"
            ),
            vec![Item::Pkcs1Key(vec![0xab].into())]
        );
    }

    #[test]
    fn skips_trailing_junk() {
        assert_eq!(
            check_both(
                b"-----BEGIN RSA PRIVATE KEY-----\n\
                    qw\n\
                    -----END RSA PRIVATE KEY-----\n\
                    junk"
            ),
            vec![Item::Pkcs1Key(vec![0xab].into())]
        );
    }

    #[test]
    fn skips_non_utf8_junk() {
        assert_eq!(
            check_both(
                b"\x00\x00\n\
                    -----BEGIN RSA PRIVATE KEY-----\n\
                    qw\n\
                    -----END RSA PRIVATE KEY-----\n
                    \x00\x00"
            ),
            vec![Item::Pkcs1Key(vec![0xab].into())]
        );
    }

    #[test]
    fn rejects_invalid_base64() {
        let input = b"-----BEGIN RSA PRIVATE KEY-----\n\
                            q=w\n\
                            -----END RSA PRIVATE KEY-----\n";
        assert_eq!(
            format!("{:?}", check_io(input)),
            "Err(Custom { kind: InvalidData, error: \"InvalidTrailingPadding\" })"
        );
        assert!(matches!(check_slice(input), Err(Error::Base64Decode(_))));
    }

    #[test]
    fn rejects_unclosed_start_section() {
        let input = b"-----BEGIN RSA PRIVATE KEY-----\n\
                            qw\n";
        assert_eq!(
            format!("{:?}",
                    check_io(input)),
            "Err(Custom { kind: InvalidData, error: \"section end \\\"-----END RSA PRIVATE KEY-----\\\" missing\" })"
        );
        assert_eq!(
            check_slice(input),
            Err(Error::MissingSectionEnd {
                end_marker: b"-----END RSA PRIVATE KEY-----".to_vec()
            })
        )
    }

    #[test]
    fn rejects_bad_start() {
        let input = b"-----BEGIN RSA PRIVATE KEY----\n\
                            qw\n\
                            -----END RSA PRIVATE KEY-----\n";
        assert_eq!(
            format!("{:?}",
                    check_io(input)),
            "Err(Custom { kind: InvalidData, error: \"illegal section start: \\\"-----BEGIN RSA PRIVATE KEY----\\\\n\\\"\" })"
        );
        assert_eq!(
            check_slice(input),
            Err(Error::IllegalSectionStart {
                line: b"-----BEGIN RSA PRIVATE KEY----".to_vec()
            })
        )
    }

    #[test]
    fn skips_unrecognised_section() {
        assert_eq!(
            check_both(
                b"junk\n\
                    -----BEGIN BREAKFAST CLUB-----\n\
                    qw\n\
                    -----END BREAKFAST CLUB-----\n"
            ),
            vec![]
        );
    }

    fn check_both(data: &[u8]) -> Vec<Item> {
        let mut reader = std::io::BufReader::new(data);
        let io_outcome = crate::read_all(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let slice_outcome = check_slice(data).unwrap();

        assert_eq!(io_outcome, slice_outcome);

        io_outcome
    }

    fn check_slice(mut data: &[u8]) -> Result<Vec<Item>, Error> {
        let mut items = vec![];
        while let Some((item, rest)) = crate::read_one_from_slice(data)? {
            items.push(item);
            data = rest;
        }

        Ok(items)
    }

    fn check_io(data: &[u8]) -> Result<Vec<Item>, std::io::Error> {
        let mut reader = std::io::BufReader::new(data);
        crate::read_all(&mut reader).collect()
    }
}
