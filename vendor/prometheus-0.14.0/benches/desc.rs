// Copyright 2020 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prometheus::core::Desc;

fn description_validation(c: &mut Criterion) {
    c.bench_function("description_validation", |b| {
        b.iter(|| {
            black_box(Desc::new(
                "api_http_requests_total".to_string(),
                "not empty help".to_string(),
                vec!["method".to_string(), "handler".to_string()],
                Default::default(),
            ))
        });
    });
}

criterion_group!(benches, description_validation);
criterion_main!(benches);
