use serde::{Deserialize, Serialize};
#[cfg(feature = "serde_bytes")]
use serde_bytes::ByteBuf;
use std::{collections::HashMap, hint::black_box, vec};

use criterion::{Criterion, criterion_group, criterion_main};

use zvariant::{LE, Type, Value, serialized::Context, to_bytes};

macro_rules! benchmark {
    ($c:ident, $data:ident, $data_type:ty, $func_prefix:literal) => {
        let ser_function_name = format!("{}_ser", $func_prefix);
        let de_function_name = format!("{}_de", $func_prefix);

        // Let's try with DBus format first
        let ctxt = Context::new_dbus(LE, 0);
        let mut group = $c.benchmark_group("dbus");
        group.measurement_time(std::time::Duration::from_secs(30));
        group.bench_function(&ser_function_name, |b| {
            b.iter(|| {
                let encoded = to_bytes(black_box(ctxt), black_box(&$data)).unwrap();
                black_box(encoded);
            })
        });

        let encoded = to_bytes(ctxt, &$data).unwrap();
        group.bench_function(&de_function_name, |b| {
            b.iter(|| {
                let (s, _): ($data_type, _) = encoded.deserialize().unwrap();
                black_box(s);
            })
        });
        group.finish();

        // Now GVariant.
        #[cfg(feature = "gvariant")]
        {
            let ctxt = Context::new_gvariant(LE, 0);
            let mut group = $c.benchmark_group("gvariant");
            group.measurement_time(std::time::Duration::from_secs(30));

            group.bench_function(&ser_function_name, |b| {
                b.iter(|| {
                    let encoded = to_bytes(black_box(ctxt), black_box(&$data)).unwrap();
                    black_box(encoded);
                })
            });

            let encoded = to_bytes(ctxt, &$data).unwrap();
            group.bench_function(&de_function_name, |b| {
                b.iter(|| {
                    let (s, _): ($data_type, _) = encoded.deserialize().unwrap();
                    black_box(s);
                })
            });
            group.finish();
        }
    };
}

#[cfg(feature = "serde_bytes")]
fn byte_array(c: &mut Criterion) {
    let ay = ByteBuf::from(vec![77u8; 100_000]);

    benchmark!(c, ay, ByteBuf, "byte_array");
}

fn fixed_size_array(c: &mut Criterion) {
    let ay = vec![77u8; 100_000];

    benchmark!(c, ay, Vec<u8>, "fixed_size_array");
}

fn big_array(c: &mut Criterion) {
    let mut asv_dict = HashMap::new();
    let mut ass_dict = HashMap::new();
    let int_array = vec![0u64; 1024 * 10];
    let mut strings = Vec::new();
    let mut string_array: Vec<&str> = Vec::new();
    for idx in 0..1024 * 10 {
        strings.push(format!(
            "{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}{idx}"
        ));
    }
    for s in &strings {
        string_array.push(s.as_str());
        ass_dict.insert(s.as_str(), s.as_str());
        asv_dict.insert(s.as_str(), Value::from(s.as_str()));
    }

    let structure = BigArrayStruct {
        string1: "Testtest",
        int1: 0xFFFFFFFFFFFFFFFFu64,
        field: BigArrayField {
            string2: "TesttestTestest",
            int2: 0xFFFFFFFFFFFFFFFFu64,
        },
        int_array,
        string_array,
    };

    benchmark!(c, structure, BigArrayStruct<'_>, "big_array");

    let data = BigArrayDictStruct {
        array_struct: structure.clone(),
        dict: ass_dict,
    };
    benchmark!(c, data, BigArrayDictStruct<'_>, "big_array_and_ass_dict");

    let data = BigArrayDictVariantStruct {
        array_struct: structure,
        dict: asv_dict,
    };
    benchmark!(
        c,
        data,
        BigArrayDictVariantStruct<'_>,
        "big_array_and_asv_dict"
    );
}

fn signature_parse(c: &mut Criterion) {
    #[derive(Type, PartialEq, Debug)]
    struct LongSignatureStruct {
        f1: BigArrayDictVariantStruct<'static>,
        f2: BigArrayDictVariantStruct<'static>,
        f3: BigArrayDictVariantStruct<'static>,
        f4: BigArrayDictVariantStruct<'static>,
        f5: BigArrayDictVariantStruct<'static>,
        f6: BigArrayDictVariantStruct<'static>,
        f7: BigArrayDictVariantStruct<'static>,
        f8: BigArrayDictVariantStruct<'static>,
        f9: BigArrayDictVariantStruct<'static>,
        f10: BigArrayDictVariantStruct<'static>,
        f11: BigArrayDictVariantStruct<'static>,
        f12: BigArrayDictVariantStruct<'static>,
        f13: BigArrayDictVariantStruct<'static>,
        f14: (u32, String, u64, i32),
    }
    let signature_str = LongSignatureStruct::SIGNATURE.to_string();
    // Ensure we have the maximum signature length allowed by the spec.
    assert_eq!(signature_str.len(), 255);

    c.bench_function("signature_parse", |b| {
        b.iter(|| {
            zvariant::Signature::try_from(black_box(signature_str.as_str())).unwrap();
        })
    });
}

fn object_path_parse(c: &mut Criterion) {
    const PATH: &str = "/a/very/very_very/veeeeeeeeeeeeeery/long/long_long/long/long/\
        _/long_path/to_test_parsing_of/paths/you/see";

    c.bench_function("object_path_parse", |b| {
        b.iter(|| {
            zvariant::ObjectPath::try_from(black_box(PATH)).unwrap();
        })
    });
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
struct BigArrayField<'f> {
    int2: u64,
    string2: &'f str,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
struct BigArrayStruct<'s> {
    string1: &'s str,
    int1: u64,
    field: BigArrayField<'s>,
    int_array: Vec<u64>,
    string_array: Vec<&'s str>,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct BigArrayDictStruct<'s> {
    #[serde(borrow)]
    array_struct: BigArrayStruct<'s>,
    dict: HashMap<&'s str, &'s str>,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct BigArrayDictVariantStruct<'s> {
    #[serde(borrow)]
    array_struct: BigArrayStruct<'s>,
    dict: HashMap<&'s str, Value<'s>>,
}

#[cfg(feature = "serde_bytes")]
criterion_group!(
    benches,
    big_array,
    byte_array,
    fixed_size_array,
    signature_parse,
    object_path_parse
);
#[cfg(not(feature = "serde_bytes"))]
criterion_group!(
    benches,
    big_array,
    fixed_size_array,
    signature_parse,
    object_path_parse
);
criterion_main!(benches);
