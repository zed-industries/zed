# Random constants
This crate provides compile time random number generation.
This allows you to insert random constants into your code that will be auto-generated at compile time.

A new value will be generated every time the file is rebuilt.
This obviously makes the resulting binary or lib non-deterministic. (See below)

# Example 

```rust
use const_random::const_random  ;
const MY_RANDOM_NUMBER: u32 = const_random!(u32);
```
This works exactly as through you have called: `OsRng.gen::<u32>()` at compile time.
So for details of the random number generation, see the `rand` crates documentation.

The following types are supported: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize and [u8; N].

# Deterministic builds

Sometimes it is an advantage for build systems to be deterministic. To support this `const-random` reads the environmental
variable `CONST_RANDOM_SEED`. If this variable is set, it will be used as the seed for the random number generation.
Setting the same seed on a build of the same code should result in identical output.

