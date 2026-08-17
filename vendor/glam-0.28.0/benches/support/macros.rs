#[macro_export]
macro_rules! bench_func {
    ($name: ident, $desc: expr, op => $func: ident, from => $from: expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const SIZE: usize = 1 << 13;
            let mut rng = support::PCG32::default();
            let inputs =
                criterion::black_box((0..SIZE).map(|_| $from(&mut rng)).collect::<Vec<_>>());
            // pre-fill output vector with some random value
            let mut outputs = vec![$func($from(&mut rng)); SIZE];
            let mut i = 0;
            c.bench_function($desc, |b| {
                b.iter(|| {
                    i = (i + 1) & (SIZE - 1);
                    unsafe {
                        *outputs.get_unchecked_mut(i) = $func(*inputs.get_unchecked(i));
                    }
                })
            });
            criterion::black_box(outputs);
        }
    };
}

#[macro_export]
macro_rules! bench_unop {
    ($name: ident, $desc: expr, op => $unop: ident, from => $from: expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const SIZE: usize = 1 << 13;
            let mut rng = support::PCG32::default();
            let inputs =
                criterion::black_box((0..SIZE).map(|_| $from(&mut rng)).collect::<Vec<_>>());
            // pre-fill output vector with some random value
            let mut outputs = vec![$from(&mut rng).$unop(); SIZE];
            let mut i = 0;
            c.bench_function($desc, |b| {
                b.iter(|| {
                    i = (i + 1) & (SIZE - 1);
                    unsafe {
                        *outputs.get_unchecked_mut(i) = inputs.get_unchecked(i).$unop();
                    }
                })
            });
            criterion::black_box(outputs);
        }
    };
}

#[macro_export]
macro_rules! bench_binop {
    ($name: ident, $desc: expr, op => $binop: ident, from1 => $from1:expr, from2 => $from2:expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const SIZE: usize = 1 << 13;
            let mut rng = support::PCG32::default();
            let inputs1 =
                criterion::black_box((0..SIZE).map(|_| $from1(&mut rng)).collect::<Vec<_>>());
            let inputs2 =
                criterion::black_box((0..SIZE).map(|_| $from2(&mut rng)).collect::<Vec<_>>());
            // pre-fill output vector with some random value
            let mut outputs = vec![$from1(&mut rng).$binop($from2(&mut rng)); SIZE];
            let mut i = 0;
            c.bench_function($desc, |b| {
                b.iter(|| {
                    i = (i + 1) & (SIZE - 1);
                    unsafe {
                        *outputs.get_unchecked_mut(i) = inputs1.get_unchecked(i).$binop(*inputs2.get_unchecked(i));
                    }
                })
            });
            criterion::black_box(outputs);
        }
    };
    ($name: ident, $desc: expr, op => $binop: ident, from => $from: expr) => {
        bench_binop!($name, $desc, op => $binop, from1 => $from, from2 => $from);
    };
}

#[macro_export]
macro_rules! bench_trinop {
    ($name: ident, $desc: expr, op => $trinop: ident, from1 => $from1:expr, from2 => $from2:expr, from3 => $from3:expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const SIZE: usize = 1 << 13;
            let mut rng = support::PCG32::default();
            let inputs1 =
                criterion::black_box((0..SIZE).map(|_| $from1(&mut rng)).collect::<Vec<_>>());
            let inputs2 =
                criterion::black_box((0..SIZE).map(|_| $from2(&mut rng)).collect::<Vec<_>>());
            let inputs3 =
                criterion::black_box((0..SIZE).map(|_| $from3(&mut rng)).collect::<Vec<_>>());
            // pre-fill output vector with some random value
            let mut outputs =
                vec![$from1(&mut rng).$trinop($from2(&mut rng), $from3(&mut rng)); SIZE];
            let mut i = 0;
            c.bench_function($desc, |b| {
                b.iter(|| {
                    i = (i + 1) & (SIZE - 1);
                    unsafe {
                        *outputs.get_unchecked_mut(i) = inputs1
                            .get_unchecked(i)
                            .$trinop(*inputs2.get_unchecked(i), *inputs3.get_unchecked(i));
                    }
                })
            });
            criterion::black_box(outputs);
        }
    };
}

#[macro_export]
macro_rules! bench_select {
    ($name:ident, $desc:expr, ty => $ty: ident, op => $op: ident, from => $from:expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const SIZE: usize = 1 << 13;
            let mut rng = support::PCG32::default();
            let inputs1 =
                criterion::black_box((0..SIZE).map(|_| $from(&mut rng)).collect::<Vec<_>>());
            let inputs2 =
                criterion::black_box((0..SIZE).map(|_| $from(&mut rng)).collect::<Vec<_>>());
            let masks = vec![$from(&mut rng).$op($from(&mut rng)); SIZE];
            // pre-fill output vector with some random value
            let mut outputs = vec![$from(&mut rng); SIZE];
            let mut i = 0;
            c.bench_function($desc, |b| {
                b.iter(|| {
                    i = (i + 1) & (SIZE - 1);
                    unsafe {
                        *outputs.get_unchecked_mut(i) = $ty::select(
                            *masks.get_unchecked(i),
                            *inputs1.get_unchecked(i),
                            *inputs2.get_unchecked(i),
                        );
                    }
                })
            });
            criterion::black_box(outputs);
        }
    };
}
#[macro_export]
macro_rules! bench_from_ypr {
    ($name: ident, $desc: expr, ty => $ty:ty) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const SIZE: usize = 1 << 13;
            let mut rng = support::PCG32::default();
            let inputs = criterion::black_box(
                (0..SIZE)
                    .map(|_| {
                        (
                            random_radians(&mut rng),
                            random_radians(&mut rng),
                            random_radians(&mut rng),
                        )
                    })
                    .collect::<Vec<_>>(),
            );
            let mut outputs = vec![<$ty>::default(); SIZE];
            let mut i = 0;
            c.bench_function($desc, |b| {
                b.iter(|| {
                    i = (i + 1) & (SIZE - 1);
                    unsafe {
                        let data = inputs.get_unchecked(i);
                        *outputs.get_unchecked_mut(i) =
                            <$ty>::from_euler(glam::EulerRot::YXZ, data.0, data.1, data.2)
                    }
                })
            });
        }
    };
}

#[macro_export]
macro_rules! euler {
    ($name: ident, $desc: expr, ty => $t: ty, storage => $storage: ty, zero => $zero: expr, rand => $rand: ident) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const UPDATE_RATE: f32 = 1.0 / 60.0;
            const NUM_OBJECTS: usize = 10000;

            struct TestData {
                acc: Vec<$storage>,
                vel: Vec<$storage>,
                pos: Vec<$storage>,
            }

            let mut rng = support::PCG32::default();
            let mut data = TestData {
                acc: vec![$rand(&mut rng); NUM_OBJECTS],
                vel: vec![$zero; NUM_OBJECTS],
                pos: vec![$zero; NUM_OBJECTS],
            };
            let dt = <$t>::splat(UPDATE_RATE);

            c.bench_function($desc, |b| {
                b.iter(|| {
                    for ((position, acceleration), velocity) in
                        data.pos.iter_mut().zip(&data.acc).zip(&mut data.vel)
                    {
                        let local_acc: $t = (*acceleration).into();
                        let mut local_pos: $t = (*position).into();
                        let mut local_vel: $t = (*velocity).into();
                        local_vel += local_acc * dt;
                        local_pos += local_vel * dt;
                        *velocity = local_vel.into();
                        *position = local_pos.into();
                    }
                })
            });
        }
    };
}
