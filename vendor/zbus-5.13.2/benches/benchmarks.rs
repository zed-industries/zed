use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hint::black_box, vec};
use zbus::Message;

use criterion::{Criterion, criterion_group, criterion_main};

use zvariant::{Type, Value};

fn msg_ser(c: &mut Criterion) {
    let mut g = c.benchmark_group("message-ser");
    g.bench_function("small", |b| {
        b.iter(|| {
            let msg = empty_message();
            black_box(msg);
        })
    });

    let mut strings = Vec::new();
    let big_boy = BigBoy::new(&mut strings);
    g.measurement_time(std::time::Duration::from_secs(30));
    g.bench_function("big", |b| {
        b.iter(|| {
            let msg = big_boy_message(&big_boy);
            black_box(msg);
        })
    });
}

fn msg_de(c: &mut Criterion) {
    let mut g = c.benchmark_group("message-de");
    let msg = empty_message();
    g.bench_function("header", |b| {
        b.iter(|| {
            let header = msg.header();
            black_box(header);
            let body = msg.body();
            let signature = body.signature();
            black_box(signature);
        })
    });

    g.measurement_time(std::time::Duration::from_secs(30));
    let mut strings = Vec::new();
    let big_boy = BigBoy::new(&mut strings);
    let msg = big_boy_message(&big_boy);
    g.bench_function("body", |b| {
        b.iter(|| {
            let body = msg.body();
            let signature = body.signature();
            black_box(signature);
            let body: BigBoy<'_> = body.deserialize().unwrap();
            black_box(body);
        })
    });
}

fn empty_message() -> Message {
    Message::method_call("/org/freedesktop/DBus/Something", "Ping")
        .unwrap()
        .destination("org.freedesktop.DBus.Something")
        .unwrap()
        .interface("org.freedesktop.DBus.Something")
        .unwrap()
        .build(&())
        .unwrap()
}

fn big_boy_message(big_boy: &BigBoy<'_>) -> Message {
    Message::method_call("/org/freedesktop/DBus/Something", "Ping")
        .unwrap()
        .destination("org.freedesktop.DBus.Something")
        .unwrap()
        .interface("org.freedesktop.DBus.Something")
        .unwrap()
        .build(&big_boy)
        .unwrap()
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct BigBoy<'s> {
    string1: &'s str,
    int1: u64,
    field: (u64, &'s str),
    int_array: Vec<u64>,
    string_array: Vec<&'s str>,
    asv_dict: HashMap<&'s str, Value<'s>>,
}

impl<'s> BigBoy<'s> {
    fn new(strings: &'s mut Vec<String>) -> Self {
        let mut asv_dict = HashMap::new();
        let int_array = vec![0u64; 1024 * 10];
        let mut string_array: Vec<&str> = Vec::new();
        for idx in 0..1024 * 10 {
            strings.push(format!(
                "{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}"
            ));
        }
        for s in strings {
            string_array.push(s.as_str());
            asv_dict.insert(s.as_str(), Value::from(s.as_str()));
        }

        BigBoy {
            string1: "Testtest",
            int1: 0xFFFFFFFFFFFFFFFFu64,
            field: (0xFFFFFFFFFFFFFFFFu64, "TesttestTestest"),
            int_array,
            string_array,
            asv_dict,
        }
    }
}

fn address_parse(c: &mut Criterion) {
    const UNIX_ADDRESS: &str = "unix:path=/tmp/dbus/long/loooooooong/path/to/socket\
        /so/we/need/to/keeeeeeeeeep/going/where/no/man/has/gone/before,runtime=yes,\
        guid=0123456789ABCDEF0123456789ABCDEF";
    const TCP_ADDRESS: &str = "tcp:host=some.looong.name.so.we.must.keep.going.on.and.on.\
        on,port=1234,family=ipv4,guid=0123456789ABCDEF0123456789ABCDEF";

    let mut group = c.benchmark_group("parse_dbus_address");
    group.sample_size(1000);

    group.bench_function("unix", |b| {
        b.iter(|| {
            zbus::Address::try_from(black_box(UNIX_ADDRESS)).unwrap();
        })
    });

    group.bench_function("tcp", |b| {
        b.iter(|| {
            zbus::Address::try_from(black_box(TCP_ADDRESS)).unwrap();
        })
    });
}

criterion_group!(benches, msg_ser, msg_de, address_parse);
criterion_main!(benches);
