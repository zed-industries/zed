use hmac::Hmac;
use sha1::Sha1;

#[derive(Debug)]
pub struct Test {
    pub name: &'static str,
    pub password: &'static [u8],
    pub salt: &'static [u8],
    pub c: &'static str,
    pub output: &'static [u8],
}

#[macro_export]
macro_rules! new_tests {
    ( $( $name:expr ),*  ) => {
        [$(
            Test {
                name: $name,
                password: include_bytes!(concat!("data/", $name, ".password.bin")),
                salt: include_bytes!(concat!("data/", $name, ".salt.bin")),
                c: include_str!(concat!("data/", $name, ".c.bin")),
                output: include_bytes!(concat!("data/", $name, ".output.bin")),
            },
        )*]
    };
}

#[test]
fn rfc6070() {
    let tests = new_tests!("1", "2", "3", "4", "5");
    let mut buf = [0u8; 25];
    for test in &tests {
        let c = test.c.parse().unwrap();
        println!("c: {:?}", test);
        let n = test.output.len();
        pbkdf2::pbkdf2::<Hmac<Sha1>>(test.password, test.salt, c, &mut buf[..n]);
        assert_eq!(&buf[..n], test.output);
    }
}
