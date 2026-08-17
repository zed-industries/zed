use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, thiserror::Error)]
#[error("Cycle detected")]
pub struct TopoSortCycle;

pub fn toposort<K, I>(
    input: impl IntoIterator<Item = K>,
    deps: impl Fn(&K) -> I,
) -> Result<Vec<K>, TopoSortCycle>
where
    K: Eq + Hash + Clone,
    I: Iterator<Item = K>,
{
    struct Ts<K, D, I>
    where
        K: Eq + Hash + Clone,
        I: Iterator<Item = K>,
        D: Fn(&K) -> I,
    {
        result_set: HashSet<K>,
        result: Vec<K>,
        deps: D,
        stack: HashSet<K>,
    }

    impl<K, D, I> Ts<K, D, I>
    where
        K: Eq + Hash + Clone,
        I: Iterator<Item = K>,
        D: Fn(&K) -> I,
    {
        fn visit(&mut self, i: &K) -> Result<(), TopoSortCycle> {
            if self.result_set.contains(i) {
                return Ok(());
            }

            if !self.stack.insert(i.clone()) {
                return Err(TopoSortCycle);
            }
            for dep in (self.deps)(i) {
                self.visit(&dep)?;
            }

            let removed = self.stack.remove(i);
            assert!(removed);

            self.result.push(i.clone());
            self.result_set.insert(i.clone());

            Ok(())
        }
    }

    let mut ts = Ts {
        result: Vec::new(),
        result_set: HashSet::new(),
        deps,
        stack: HashSet::new(),
    };

    for i in input {
        ts.visit(&i)?;
    }

    Ok(ts.result)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::toposort::toposort;
    use crate::toposort::TopoSortCycle;

    fn test_toposort(input: &str) -> Result<Vec<&str>, TopoSortCycle> {
        let mut keys: Vec<&str> = Vec::new();
        let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
        for part in input.split(" ") {
            match part.split_once("->") {
                Some((k, vs)) => {
                    keys.push(k);
                    edges.insert(k, vs.split(",").collect());
                }
                None => keys.push(part),
            };
        }

        toposort(keys, |k| {
            edges
                .get(k)
                .map(|v| v.as_slice())
                .unwrap_or_default()
                .into_iter()
                .copied()
        })
    }

    fn test_toposort_check(input: &str, expected: &str) {
        let sorted = test_toposort(input).unwrap();
        let expected = expected.split(" ").collect::<Vec<_>>();
        assert_eq!(expected, sorted);
    }

    #[test]
    fn test() {
        test_toposort_check("1 2 3", "1 2 3");
        test_toposort_check("1->2 2->3 3", "3 2 1");
        test_toposort_check("1 2->1 3->2", "1 2 3");
        test_toposort_check("1->2,3 2->3 3", "3 2 1");
    }

    #[test]
    fn cycle() {
        assert!(test_toposort("1->1").is_err());
        assert!(test_toposort("1->2 2->1").is_err());
    }
}
