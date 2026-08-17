#![cfg_attr(test, feature(test))]

extern crate random_choice;
extern crate rand;
extern crate test;


#[cfg(test)]
mod benches {
    use test::Bencher;
    use random_choice::random_choice;
    use random_choice::RandomChoice;
    use rand::SeedableRng;

    #[bench]
    fn bench_random_choice_1000_it_f64(b: &mut Bencher) {
        let capacity: usize = 1000;
        let mut samples: Vec<f64> = Vec::with_capacity(capacity);
        let mut weights: Vec<f64> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push((i + 1usize) as f64);
            weights.push((i + 1usize) as f64);
        }
        b.iter(|| {
            random_choice().random_choice_f64(&samples, &weights, capacity);
        });
    }

    #[bench]
    fn bench_random_choice_500_it_f64(b: &mut Bencher) {
        let capacity: usize = 500;
        let mut samples: Vec<f64> = Vec::with_capacity(capacity);
        let mut weights: Vec<f64> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push((i + 1usize) as f64);
            weights.push((i + 1usize) as f64);
        }
        b.iter(|| {
            random_choice().random_choice_f64(&samples, &weights, capacity);
        });
    }

    #[bench]
    fn bench_random_choice_1000_it_f32(b: &mut Bencher) {
        let capacity: usize = 1000;
        let mut samples: Vec<f32> = Vec::with_capacity(capacity);
        let mut weights: Vec<f32> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push((i + 1usize) as f32);
            weights.push((i + 1usize) as f32);
        }
        b.iter(|| {
            random_choice().random_choice_f32(&samples, &weights, capacity);
        });
    }

    #[bench]
    fn bench_random_choice_500_it_f32(b: &mut Bencher) {
        let capacity: usize = 500;
        let mut samples: Vec<f32> = Vec::with_capacity(capacity);
        let mut weights: Vec<f32> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push((i + 1usize) as f32);
            weights.push((i + 1usize) as f32);
        }
        b.iter(|| {
            random_choice().random_choice_f32(&samples, &weights, capacity);
        });
    }

    #[bench]
    fn test_random_choice_with_seed_f64(b: &mut Bencher) {
        let capacity: usize = 1000;
        let mut samples: Vec<usize> = Vec::with_capacity(capacity);
        let mut weights: Vec<f64> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push(i);
            weights.push(i as f64);
        }

        let rng = super::rand::StdRng::from_seed(&[5000, 44, 55, 199]);
        let mut random_choice = RandomChoice::new(rng);

        let number_choices = 1000;

        b.iter(|| {
            random_choice.random_choice_f64(&samples, &weights, number_choices);
        });
    }
}