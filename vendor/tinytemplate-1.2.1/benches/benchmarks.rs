#[macro_use]
extern crate criterion;
extern crate tinytemplate;
#[macro_use]
extern crate serde_derive;

use criterion::Criterion;
use tinytemplate::TinyTemplate;

static TABLE_SOURCE: &'static str = "<html>
    {{ for row in table }}
        <tr>{{ for value in row }}<td>{value}</td>{{ endfor }}</tr>
    {{ endfor }}
</html>";

#[derive(Serialize)]
struct TableContext {
    table: Vec<Vec<usize>>,
}

fn make_table_context(size: usize) -> TableContext {
    let mut table = Vec::with_capacity(size);
    for _ in 0..size {
        let mut inner = Vec::with_capacity(size);
        for i in 0..size {
            inner.push(i);
        }
        table.push(inner);
    }
    TableContext { table }
}

fn parse(criterion: &mut Criterion) {
    criterion.bench_function("parse-table", |b| {
        b.iter(|| {
            let mut tt = TinyTemplate::new();
            tt.add_template("table", TABLE_SOURCE).unwrap()
        });
    });
}

fn render(criterion: &mut Criterion) {
    let mut tt = TinyTemplate::new();
    tt.add_template("table", TABLE_SOURCE).unwrap();

    criterion.bench_function_over_inputs(
        "render-table",
        move |b, size| {
            let data = make_table_context(*size);

            b.iter(|| tt.render("table", &data).unwrap());
        },
        vec![1usize, 5, 10, 50, 100, 200],
    );
}

criterion_group!(benchmarks, parse, render);
criterion_main!(benchmarks);
