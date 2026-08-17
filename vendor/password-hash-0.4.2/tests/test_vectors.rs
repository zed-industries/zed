//! Test vectors for commonly used password hashing algorithms.

use password_hash::{Ident, PasswordHash};

const ARGON2D_HASH: &str =
    "$argon2d$v=19$m=512,t=3,p=2$5VtWOO3cGWYQHEMaYGbsfQ$AcmqasQgW/wI6wAHAMk4aQ";
const BCRYPT_HASH: &str = "$2b$MTIzNA$i5btSOiulHhaPHPbgNUGdObga/GCAVG/y5HHY1ra7L0C9dpCaw8u";
const SCRYPT_HASH: &str =
    "$scrypt$epIxT/h6HbbwHaehFnh/bw$7H0vsXlY8UxxyW/BWx/9GuY7jEvGjT71GFd6O4SZND0";

#[test]
fn argon2id() {
    let ph = PasswordHash::new(ARGON2D_HASH).unwrap();
    assert_eq!(ph.algorithm, Ident::new("argon2d").unwrap());
    assert_eq!(ph.version, Some(19));
    assert_eq!(ph.params.iter().count(), 3);
    assert_eq!(ph.params.get_decimal("m").unwrap(), 512);
    assert_eq!(ph.params.get_decimal("t").unwrap(), 3);
    assert_eq!(ph.params.get_decimal("p").unwrap(), 2);
    assert_eq!(ph.salt.unwrap().as_ref(), "5VtWOO3cGWYQHEMaYGbsfQ");
    assert_eq!(ph.hash.unwrap().to_string(), "AcmqasQgW/wI6wAHAMk4aQ");
    assert_eq!(ph.to_string(), ARGON2D_HASH);
}

#[test]
fn bcrypt() {
    let ph = PasswordHash::new(BCRYPT_HASH).unwrap();
    assert_eq!(ph.algorithm, Ident::new("2b").unwrap());
    assert_eq!(ph.version, None);
    assert_eq!(ph.params.len(), 0);
    assert_eq!(ph.salt.unwrap().to_string(), "MTIzNA");
    assert_eq!(
        ph.hash.unwrap().to_string(),
        "i5btSOiulHhaPHPbgNUGdObga/GCAVG/y5HHY1ra7L0C9dpCaw8u"
    );
    assert_eq!(ph.to_string(), BCRYPT_HASH);
}

#[test]
fn scrypt() {
    let ph = PasswordHash::new(SCRYPT_HASH).unwrap();
    assert_eq!(ph.algorithm, Ident::new("scrypt").unwrap());
    assert_eq!(ph.version, None);
    assert_eq!(ph.params.len(), 0);
    assert_eq!(ph.salt.unwrap().to_string(), "epIxT/h6HbbwHaehFnh/bw");
    assert_eq!(
        ph.hash.unwrap().to_string(),
        "7H0vsXlY8UxxyW/BWx/9GuY7jEvGjT71GFd6O4SZND0"
    );
    assert_eq!(ph.to_string(), SCRYPT_HASH);
}
