# Procedural macro attribute to do not repeat yourself while testing

## Usage

``` rust
#[macro_use]
extern crate interpolate_name;

use interpolate_name::interpolate_test;

#[interpolate_test(foo, "foo")]
#[interpolate_test(bar, "bar")]
#[interpolate_test(baz, "baz")]
fn testme(f: &str) {
    println!("testing {}", f);
}
```

Produces

```
running 3 tests
test testme_baz ... ok
test testme_bar ... ok
test testme_foo ... ok
```


