use std::env;

/// Returns the seeds a randomized `#[gpui::test]` or `#[gpui::bench]` runs with, in
/// order, and whether there is more than one.
///
/// With `$SEED` set, runs `$SEED..$SEED + iterations` and ignores `explicit_seeds`.
/// Otherwise runs `0..iterations` followed by `explicit_seeds`, except that the
/// default single iteration is dropped when explicit seeds are given.
/// `$ITERATIONS` overrides `iterations`.
/// Tests and benchmarks share this so that one `SEED` reproduces either.
///
/// This is intended for use by the `gpui::test` and `gpui::bench` macros and generally
/// should not be used directly.
#[doc(hidden)]
pub fn calculate_seeds(
    iterations: u64,
    explicit_seeds: &[u64],
) -> (impl Iterator<Item = u64> + '_, bool) {
    let iterations = env::var("ITERATIONS")
        .ok()
        .map(|var| var.parse().expect("invalid ITERATIONS variable"))
        .unwrap_or(iterations);

    let env_seed = env::var("SEED")
        .map(|seed| seed.parse().expect("invalid SEED variable as integer"))
        .ok();

    let iter = seeds(iterations, explicit_seeds, env_seed);
    let is_multiple_runs = iter.clone().nth(1).is_some();
    (iter, is_multiple_runs)
}

fn seeds(
    iterations: u64,
    explicit_seeds: &[u64],
    env_seed: Option<u64>,
) -> impl Iterator<Item = u64> + Clone + '_ {
    let (iterations_range, explicit_seeds) = match env_seed {
        Some(env_seed) => (env_seed..env_seed + iterations, &[][..]),
        None if iterations == 1 && !explicit_seeds.is_empty() => (0..0, explicit_seeds),
        None => (0..iterations, explicit_seeds),
    };
    iterations_range.chain(explicit_seeds.iter().copied())
}

#[cfg(test)]
mod tests {
    use super::seeds;

    fn collect(iterations: u64, explicit_seeds: &[u64], env_seed: Option<u64>) -> Vec<u64> {
        seeds(iterations, explicit_seeds, env_seed).collect()
    }

    #[test]
    fn seeds_follow_iterations_explicit_seeds_and_seed_variable() {
        assert_eq!(collect(1, &[], None), [0]);
        assert_eq!(collect(3, &[], None), [0, 1, 2]);
        assert_eq!(collect(1, &[10, 20], None), [10, 20]);
        assert_eq!(collect(2, &[10], None), [0, 1, 10]);
        assert_eq!(collect(1, &[10], Some(7)), [7]);
        assert_eq!(
            collect(2, &[10], Some(7)),
            [7, 8],
            "`SEED` starts the iterations once, without repeating itself"
        );
    }
}
