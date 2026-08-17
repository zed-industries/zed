use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sqlformat::*;

fn simple_query(c: &mut Criterion) {
    let input = "SELECT * FROM my_table WHERE id = 1";
    c.bench_function("simple query", |b| {
        b.iter(|| {
            format(
                black_box(input),
                black_box(&QueryParams::None),
                black_box(FormatOptions::default()),
            )
        })
    });
}

fn complex_query(c: &mut Criterion) {
    let input = "SELECT t1.id, t1.name, t1.title, t1.description, t2.mothers_maiden_name, t2.first_girlfriend\nFROM my_table t1 LEFT JOIN other_table t2 ON t1.id = t2.other_id WHERE t2.order BETWEEN  17 AND 30";
    c.bench_function("complex query", |b| {
        b.iter(|| {
            format(
                black_box(input),
                black_box(&QueryParams::None),
                black_box(FormatOptions::default()),
            )
        })
    });
}

fn query_with_named_params(c: &mut Criterion) {
    let input = "SELECT * FROM my_table WHERE id = :first OR id = :second OR id = :third";
    let params = vec![
        ("first".to_string(), "1".to_string()),
        ("second".to_string(), "2".to_string()),
        ("third".to_string(), "3".to_string()),
    ];
    c.bench_function("named params", |b| {
        b.iter(|| {
            format(
                black_box(input),
                black_box(&QueryParams::Named(params.clone())),
                black_box(FormatOptions::default()),
            )
        })
    });
}

fn query_with_explicit_indexed_params(c: &mut Criterion) {
    let input = "SELECT * FROM my_table WHERE id = ?1 OR id = ?2 OR id = ?0";
    let params = vec!["0".to_string(), "1".to_string(), "2".to_string()];
    c.bench_function("explicit indexed params", |b| {
        b.iter(|| {
            format(
                black_box(input),
                black_box(&QueryParams::Indexed(params.clone())),
                black_box(FormatOptions::default()),
            )
        })
    });
}

fn query_with_implicit_indexed_params(c: &mut Criterion) {
    let input = "SELECT * FROM my_table WHERE id = ? OR id = ? OR id = ?";
    let params = vec!["0".to_string(), "1".to_string(), "2".to_string()];
    c.bench_function("implicit indexed params", |b| {
        b.iter(|| {
            format(
                black_box(input),
                black_box(&QueryParams::Indexed(params.clone())),
                black_box(FormatOptions::default()),
            )
        })
    });
}

fn issue_633(c: &mut Criterion) {
    const SIZE: usize = 1000;

    pub struct UserData {
        pub first_name: String,
        pub last_name: String,
        pub address: String,
        pub email: String,
        pub phone: String,
    }

    fn sample() -> UserData {
        UserData {
            first_name: "FIRST_NAME".to_string(),
            last_name: "LAST_NAME".to_string(),
            address: "SOME_ADDRESS".to_string(),
            email: "email@example.com".to_string(),
            phone: "9999999999".to_string(),
        }
    }

    fn to_insert_params(user_data: &UserData) -> String {
        format!(
            r#"('{}', '{}', '{}', '{}', '{}')"#,
            user_data.first_name,
            user_data.last_name,
            user_data.address,
            user_data.email,
            user_data.phone,
        )
    }

    static INSERT_QUERY: &str = "
INSERT INTO user_data
(first_name, last_name, address, phone, email)
VALUES
";

    fn generate_insert_query() -> String {
        let mut query_str = String::with_capacity(1_000_000);
        query_str.push_str(INSERT_QUERY);
        let mut is_first = true;
        let sample_data = sample();
        for _ in 0..SIZE {
            if is_first {
                is_first = false;
            } else {
                query_str.push(',');
            }
            let params = to_insert_params(&sample_data);
            query_str.push_str(&params);
        }
        query_str.push(';');
        query_str
    }

    let input = generate_insert_query();
    c.bench_function("issue 633", |b| {
        b.iter(|| {
            format(
                black_box(&input),
                black_box(&QueryParams::None),
                black_box(FormatOptions::default()),
            )
        })
    });
}

fn issue_633_2(c: &mut Criterion) {
    let input = "SELECT\n  d.uuid AS uuid,\n\td.name_of_document AS name,\n\td.slug_name AS slug,\n\td.default_contract_uuid AS default_contract_uuid,\n\ta.uuid AS parent_uuid,\n\ta.name_of_agreement AS agreement_name,\n\td.icon_name AS icon\nFROM `documents` d\nLEFT JOIN agreements a ON a.uuid = d.parent_uuid\n WHERE d.uuid = ? LIMIT 1";
    let params = vec!["0".to_string()];
    c.bench_function("issue 633 query 2", |b| {
        b.iter(|| {
            format(
                black_box(input),
                black_box(&QueryParams::Indexed(params.clone())),
                black_box(FormatOptions::default()),
            )
        })
    });
}

fn issue_633_3(c: &mut Criterion) {
    const SIZE: usize = 1000;

    let mut input = String::with_capacity(100_000);
    input.push_str("INSERT INTO test_table(a) values ");
    let mut is_first = true;
    for _ in 0..SIZE {
        if is_first {
            is_first = false;
        } else {
            input.push_str(", ");
        }
        input.push_str("(?)");
    }

    c.bench_function("issue 633 query 3", |b| {
        b.iter(|| {
            format(
                black_box(&input),
                black_box(&QueryParams::None),
                black_box(FormatOptions::default()),
            )
        })
    });
}

criterion_group!(
    benches,
    simple_query,
    complex_query,
    query_with_named_params,
    query_with_explicit_indexed_params,
    query_with_implicit_indexed_params,
    issue_633,
    issue_633_2,
    issue_633_3
);
criterion_main!(benches);
