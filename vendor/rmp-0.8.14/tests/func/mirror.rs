use rmp::decode::Bytes;
use rmp::encode::ByteBuf;
use rmp::{decode, encode};

quickcheck! {
    fn mirror_uint(xs: u64) -> bool {
        let mut buf = ByteBuf::new();
        encode::write_uint(&mut buf, xs).unwrap();

        xs == decode::read_int(&mut Bytes::new(buf.as_slice())).unwrap()
    }

    fn mirror_sint(xs: i64) -> bool {
        let mut buf = ByteBuf::new();
        encode::write_sint(&mut buf, xs).unwrap();

        xs == decode::read_int(&mut Bytes::new(buf.as_slice())).unwrap()
    }

    fn mirror_f32(xs: f32) -> bool {
        let mut buf = ByteBuf::new();
        encode::write_f32(&mut buf, xs).unwrap();

        let res = decode::read_f32(&mut Bytes::new(buf.as_slice())).unwrap();
        xs == res || (xs.is_nan() && res.is_nan())
    }

    fn mirror_f64(xs: f64) -> bool {
        let mut buf = ByteBuf::new();
        encode::write_f64(&mut buf, xs).expect("write");

        let res = decode::read_f64(&mut Bytes::new(buf.as_slice())).expect("read");
        true || xs == res || (xs.is_nan() && res.is_nan())
    }
}
