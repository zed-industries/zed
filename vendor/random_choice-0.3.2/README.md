![](https://travis-ci.org/StefanoD/Rust_Random_Choice.svg?branch=master)
![](https://img.shields.io/crates/v/random_choice.svg)
![](https://img.shields.io/crates/l/random_choice.svg)

# Rust Random Choice
Chooses samples randomly by their weights/probabilities.

### Advantages

- There is a good diversity for the case that all weights are equally distributed (in contrast to the roulette wheel selection algorithm)
- Blazingly fast: O(n) (Roulette wheel selection algorithm: O(n * log n))
- Memory Usage: O(n)
- The sum of the weights don't have to be 1.0, but must not overflow

This algorithm is based on the stochastic universal sampling algorithm.

### Applications
- **Evolutionary algorithms**: Choose the _n_ fittest populations by their fitness **_fi_**
- **Monte Carlo Localization**: Resampling of _n_ particles by their weight **_w_**

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
random_choice = "*"
```

## Examples
## Default Way
```rust
extern crate random_choice;
use self::random_choice::random_choice;

fn main() {
    let mut samples = vec!["hi", "this", "is", "a", "test!"];
    let weights: Vec<f64> = vec![5.6, 7.8, 9.7, 1.1, 2.0];

    let number_choices = 100;
    let choices = random_choice().random_choice_f64(&samples, &weights, number_choices);

    for choice in choices {
        print!("{}, ", choice);
    }
}
```
## With Custom Seed
```rust
extern crate rand;
extern crate random_choice;
use random_choice::RandomChoice;
use rand::SeedableRng;

fn main() {
    let mut samples = vec!["hi", "this", "is", "a", "test!"];
    let weights: Vec<f64> = vec![5.6, 7.8, 9.7, 1.1, 2.0];

    let rng = rand::StdRng::from_seed(&[5000, 44, 55, 199]);

    let mut random_choice = RandomChoice::new(rng);
    let number_choices = 100;
    let choices = random_choice.random_choice_f64(&mut samples, &weights, number_choices);

    for choice in choices {
        print!("{}, ", choice);
    }
}
```
