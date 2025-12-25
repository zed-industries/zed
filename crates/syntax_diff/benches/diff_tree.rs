use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use syntax_diff::{DiffTree, generate_diff, match_trees};

fn parse_rust(code: &str) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("failed to set language");
    parser.parse(code, None).expect("failed to parse")
}

fn generate_rust_code(num_functions: usize) -> String {
    let mut code = String::new();
    for i in 0..num_functions {
        code.push_str(&format!(
            r#"
fn function_{i}(a: i32, b: i32) -> i32 {{
    let result = a + b;
    if result > 0 {{
        result * 2
    }} else {{
        result - 1
    }}
}}
"#
        ));
    }
    code
}

fn generate_modified_rust_code(num_functions: usize, modification_ratio: f64) -> String {
    let mut code = String::new();
    let modifications = (num_functions as f64 * modification_ratio) as usize;

    for i in 0..num_functions {
        if i < modifications {
            // Modified function
            code.push_str(&format!(
                r#"
fn function_{i}_renamed(a: i32, b: i32, c: i32) -> i32 {{
    let result = a + b + c;
    if result > 0 {{
        result * 3
    }} else {{
        result - 2
    }}
}}
"#
            ));
        } else {
            // Unchanged function
            code.push_str(&format!(
                r#"
fn function_{i}(a: i32, b: i32) -> i32 {{
    let result = a + b;
    if result > 0 {{
        result * 2
    }} else {{
        result - 1
    }}
}}
"#
            ));
        }
    }
    code
}

fn bench_tree_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_construction");

    for num_functions in [10, 50, 100, 200, 1000, 10000] {
        let code = generate_rust_code(num_functions);
        let tree = parse_rust(&code);

        group.throughput(Throughput::Bytes(code.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{num_functions}_functions")),
            &(tree, code),
            |b, (tree, code)| {
                b.iter(|| {
                    let diff_tree = DiffTree::new(black_box(tree.walk()), black_box(code));
                    black_box(diff_tree)
                });
            },
        );
    }

    group.finish();
}

fn bench_identical_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("identical_matching");

    for num_functions in [10, 50, 100, 200, 1000, 10000] {
        let code = generate_rust_code(num_functions);
        let tree = parse_rust(&code);
        let diff_tree = DiffTree::new(tree.walk(), &code);

        group.throughput(Throughput::Elements(diff_tree.node_count() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{num_functions}_functions")),
            &diff_tree,
            |b, diff_tree| {
                b.iter(|| {
                    let matching = match_trees(black_box(diff_tree), black_box(diff_tree));
                    black_box(matching)
                });
            },
        );
    }

    group.finish();
}

fn bench_modified_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("modified_matching");

    for num_functions in [10, 50, 100, 500, 1000] {
        for modification_ratio in [0.1, 0.25, 0.5] {
            let old_code = generate_rust_code(num_functions);
            let new_code = generate_modified_rust_code(num_functions, modification_ratio);

            let old_tree = parse_rust(&old_code);
            let new_tree = parse_rust(&new_code);

            let old_diff = DiffTree::new(old_tree.walk(), &old_code);
            let new_diff = DiffTree::new(new_tree.walk(), &new_code);

            let label = format!(
                "{num_functions}_funcs_{:.0}pct_modified",
                modification_ratio * 100.0
            );

            group.throughput(Throughput::Elements(
                (old_diff.node_count() + new_diff.node_count()) as u64,
            ));
            group.bench_with_input(
                BenchmarkId::from_parameter(&label),
                &(old_diff, new_diff),
                |b, (old, new)| {
                    b.iter(|| {
                        let matching = match_trees(black_box(old), black_box(new));
                        black_box(matching)
                    });
                },
            );
        }
    }

    group.finish();
}

fn bench_diff_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("diff_generation");

    for num_functions in [10, 50, 100, 200, 500, 1000] {
        let old_code = generate_rust_code(num_functions);
        let new_code = generate_modified_rust_code(num_functions, 0.25);

        let old_tree = parse_rust(&old_code);
        let new_tree = parse_rust(&new_code);

        let old_diff = DiffTree::new(old_tree.walk(), &old_code);
        let new_diff = DiffTree::new(new_tree.walk(), &new_code);
        let matching = match_trees(&old_diff, &new_diff);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{num_functions}_functions")),
            &(old_diff, new_diff, matching),
            |b, (old, new, matching)| {
                b.iter(|| {
                    let diff = generate_diff(black_box(old), black_box(new), black_box(matching));
                    black_box(diff)
                });
            },
        );
    }

    group.finish();
}

fn bench_full_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_pipeline");

    for num_functions in [10, 50, 100, 1000] {
        let old_code = generate_rust_code(num_functions);
        let new_code = generate_modified_rust_code(num_functions, 0.25);

        let old_tree = parse_rust(&old_code);
        let new_tree = parse_rust(&new_code);

        group.throughput(Throughput::Bytes((old_code.len() + new_code.len()) as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{num_functions}_functions")),
            &(old_tree, old_code, new_tree, new_code),
            |b, (old_tree, old_code, new_tree, new_code)| {
                b.iter(|| {
                    let old_diff = DiffTree::new(black_box(old_tree.walk()), black_box(old_code));
                    let new_diff = DiffTree::new(black_box(new_tree.walk()), black_box(new_code));
                    let matching = match_trees(&old_diff, &new_diff);
                    let diff = generate_diff(&old_diff, &new_diff, &matching);
                    black_box(diff)
                });
            },
        );
    }

    group.finish();
}

fn bench_real_world_scenario(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world");

    // Simulate a typical git diff scenario: small change in a medium-sized file
    let old_code = r#"
use std::collections::HashMap;

pub struct Cache {
    data: HashMap<String, String>,
    max_size: usize,
}

impl Cache {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_size,
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn insert(&mut self, key: String, value: String) {
        if self.data.len() >= self.max_size {
            // Simple eviction: remove first key
            if let Some(first_key) = self.data.keys().next().cloned() {
                self.data.remove(&first_key);
            }
        }
        self.data.insert(key, value);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut cache = Cache::new(10);
        cache.insert("key".to_string(), "value".to_string());
        assert_eq!(cache.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_eviction() {
        let mut cache = Cache::new(2);
        cache.insert("a".to_string(), "1".to_string());
        cache.insert("b".to_string(), "2".to_string());
        cache.insert("c".to_string(), "3".to_string());
        assert_eq!(cache.len(), 2);
    }
}
"#;

    // Small modification: add a new method and modify an existing one
    let new_code = r#"
use std::collections::HashMap;

pub struct Cache {
    data: HashMap<String, String>,
    max_size: usize,
}

impl Cache {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_size,
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        if self.data.len() >= self.max_size && !self.data.contains_key(&key) {
            // Simple eviction: remove first key
            if let Some(first_key) = self.data.keys().next().cloned() {
                self.data.remove(&first_key);
            }
        }
        self.data.insert(key, value)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut cache = Cache::new(10);
        cache.insert("key".to_string(), "value".to_string());
        assert_eq!(cache.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_contains() {
        let mut cache = Cache::new(10);
        cache.insert("key".to_string(), "value".to_string());
        assert!(cache.contains("key"));
        assert!(!cache.contains("missing"));
    }

    #[test]
    fn test_eviction() {
        let mut cache = Cache::new(2);
        cache.insert("a".to_string(), "1".to_string());
        cache.insert("b".to_string(), "2".to_string());
        cache.insert("c".to_string(), "3".to_string());
        assert_eq!(cache.len(), 2);
    }
}
"#;

    let old_tree = parse_rust(old_code);
    let new_tree = parse_rust(new_code);

    group.throughput(Throughput::Bytes((old_code.len() + new_code.len()) as u64));
    group.bench_function("typical_git_diff", |b| {
        b.iter(|| {
            let old_diff = DiffTree::new(black_box(old_tree.walk()), black_box(old_code));
            let new_diff = DiffTree::new(black_box(new_tree.walk()), black_box(new_code));
            let matching = match_trees(&old_diff, &new_diff);
            let diff = generate_diff(&old_diff, &new_diff, &matching);
            black_box(diff)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_tree_construction,
    bench_identical_matching,
    bench_modified_matching,
    bench_diff_generation,
    bench_full_pipeline,
    bench_real_world_scenario,
);

criterion_main!(benches);
