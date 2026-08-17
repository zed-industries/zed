use leb128;
use quickcheck;

use std::io;

#[test]
fn can_write_any_unsigned_int() {
    fn f(x: u64) -> io::Result<()> {
        let mut v = vec![];
        leb128::write::unsigned(&mut v, x)?;
        Ok(())
    }
    quickcheck::quickcheck(f as fn(u64) -> io::Result<()>);
}

#[test]
fn can_round_trip_any_unsigned_int() {
    fn f(x: u64) -> io::Result<bool> {
        let mut v = vec![];
        leb128::write::unsigned(&mut v, x)?;
        let y = leb128::read::unsigned(&mut &v[..])
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(x == y)
    }
    quickcheck::quickcheck(f as fn(u64) -> io::Result<bool>);
}

#[test]
fn can_write_any_signed_int() {
    fn f(x: i64) -> io::Result<()> {
        let mut v = vec![];
        leb128::write::signed(&mut v, x)?;
        Ok(())
    }
    quickcheck::quickcheck(f as fn(i64) -> io::Result<()>);
}

#[test]
fn can_round_trip_any_signed_int() {
    fn f(x: i64) -> io::Result<bool> {
        let mut v = vec![];
        leb128::write::signed(&mut v, x)?;
        let y = leb128::read::signed(&mut &v[..])
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(x == y)
    }
    quickcheck::quickcheck(f as fn(i64) -> io::Result<bool>);
}
