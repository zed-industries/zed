use na::{DMatrix, Eigen};

fn eigen_100x100(bh: &mut criterion::Criterion) {
    let m = DMatrix::<f64>::new_random(100, 100);

    bh.bench_function("eigen_100x100", move |bh| bh.iter(|| Eigen::new(m.clone(), 1.0e-7, 0)));
}

fn eigen_500x500(bh: &mut criterion::Criterion) {
    let m = DMatrix::<f64>::new_random(500, 500);

    bh.bench_function("eigen_500x500", move |bh| bh.iter(|| Eigen::new(m.clone(), 1.0e-7, 0)));
}

fn eigenvalues_100x100(bh: &mut criterion::Criterion) {
    let m = DMatrix::<f64>::new_random(100, 100);

    bh.bench_function("eigenvalues_100x100", move |bh| bh.iter(|| m.clone().eigenvalues(1.0e-7, 0)));
}

fn eigenvalues_500x500(bh: &mut criterion::Criterion) {
    let m = DMatrix::<f64>::new_random(500, 500);

    bh.bench_function("eigenvalues_500x500", move |bh| bh.iter(|| m.clone().eigenvalues(1.0e-7, 0)));
}

criterion_group!(eigen,
    eigen_100x100,
//    eigen_500x500,
    eigenvalues_100x100,
//    eigenvalues_500x500
);
