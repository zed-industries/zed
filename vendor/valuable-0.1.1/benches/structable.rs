use valuable::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Default)]
struct HelloWorld {
    one: usize,
    two: usize,
    three: usize,
    four: usize,
    five: usize,
    six: usize,
}

static FIELDS: &[NamedField<'static>] = &[
    NamedField::new("one"),
    NamedField::new("two"),
    NamedField::new("three"),
    NamedField::new("four"),
    NamedField::new("five"),
    NamedField::new("six"),
];

impl Structable for HelloWorld {
    fn definition(&self) -> StructDef<'_> {
        StructDef::new_static("HelloWorld", Fields::Named(FIELDS))
    }
}

impl Valuable for HelloWorld {
    fn as_value(&self) -> Value<'_> {
        Value::Structable(self)
    }

    fn visit(&self, v: &mut dyn Visit) {
        v.visit_named_fields(&NamedValues::new(
            FIELDS,
            &[
                Value::Usize(self.one),
                Value::Usize(self.two),
                Value::Usize(self.three),
                Value::Usize(self.four),
                Value::Usize(self.five),
                Value::Usize(self.six),
            ],
        ));
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    const NUM: usize = 50;

    let hello_world = black_box(HelloWorld::default());
    let structable = &hello_world as &dyn Structable;
    let f = match structable.definition() {
        StructDef::Static {
            fields: Fields::Named(fields),
            ..
        } => &fields[5],
        _ => unreachable!(),
    };

    struct Sum(usize, &'static NamedField<'static>);

    impl Visit for Sum {
        fn visit_named_fields(&mut self, record: &NamedValues<'_>) {
            self.0 += match record.get(self.1) {
                Some(Value::Usize(v)) => v,
                _ => return,
            }
        }

        fn visit_value(&mut self, _: Value<'_>) {
            unimplemented!()
        }
    }

    c.bench_function("struct", |b| {
        b.iter(|| {
            let mut num = 0;
            for _ in 0..NUM {
                let hello_world = black_box(HelloWorld::default());
                num += hello_world.six;
            }

            black_box(num);
        })
    });

    c.bench_function("valuable", |b| {
        b.iter(|| {
            let mut v = Sum(black_box(0), f);

            for _ in 0..NUM {
                v.visit_named_fields(&NamedValues::new(
                    FIELDS,
                    &[
                        Value::Usize(0),
                        Value::Usize(0),
                        Value::Usize(0),
                        Value::Usize(0),
                        Value::Usize(0),
                        Value::Usize(0),
                    ],
                ));
                /*
                v.visit_struct(&Record::new(
                    &definition,
                    &[
                        Value::Usize(hello_world.one),
                        Value::Usize(hello_world.two),
                        Value::Usize(hello_world.three),
                        Value::Usize(hello_world.four),
                        Value::Usize(hello_world.five),
                        Value::Usize(hello_world.six),
                    ]
                ));
                */
                // hello_world.visit(&mut v);
            }

            black_box(v.0);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
