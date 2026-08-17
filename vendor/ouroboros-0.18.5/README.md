# Ouroboros

[![Ouroboros on Crates.IO](https://img.shields.io/crates/v/ouroboros)](https://crates.io/crates/ouroboros)
[![Documentation](https://img.shields.io/badge/documentation-link-success)](https://docs.rs/ouroboros)

Easy self-referential struct generation for Rust. 
Dual licensed under MIT / Apache 2.0.

```rust
use ouroboros::self_referencing;

#[self_referencing]
struct MyStruct {
    int_data: i32,
    float_data: f32,
    #[borrows(int_data)]
    // the 'this lifetime is created by the #[self_referencing] macro
    // and should be used on all references marked by the #[borrows] macro
    int_reference: &'this i32,
    #[borrows(mut float_data)]
    float_reference: &'this mut f32,
}

fn main() {
    // The builder is created by the #[self_referencing] macro 
    // and is used to create the struct
    let mut my_value = MyStructBuilder {
        int_data: 42,
        float_data: 3.14,

        // Note that the name of the field in the builder 
        // is the name of the field in the struct + `_builder` 
        // ie: {field_name}_builder
        // the closure that assigns the value for the field will be passed 
        // a reference to the field(s) defined in the #[borrows] macro
	
        int_reference_builder: |int_data: &i32| int_data,
        float_reference_builder: |float_data: &mut f32| float_data,
    }.build();

    // The fields in the original struct can not be accessed directly
    // The builder creates accessor methods which are called borrow_{field_name}()

    // Prints 42
    println!("{:?}", my_value.borrow_int_data());
    // Prints 3.14
    println!("{:?}", my_value.borrow_float_reference());
    // Sets the value of float_data to 84.0
    my_value.with_mut(|fields| {
        **fields.float_reference = (**fields.int_reference as f32) * 2.0;
    });

    // We can hold on to this reference...
    let int_ref = *my_value.borrow_int_reference();
    println!("{:?}", *int_ref);
    // As long as the struct is still alive.
    drop(my_value);
    // This will cause an error!
    // println!("{:?}", *int_ref);
}
```

Since the macro this crate provides adds lots of public functions to structs it is used on (each wrapping a particular unsafe operation in a safe way), it is not recommended to use this macro on types exposed to users of a library. Instead, it is expected that this macro will be used on an internal struct, then wrapped with a friendly struct that the library exports:

```rust
// The extra wrapper methods are only added to this struct.
#[self_referencing]
struct Internal {
    // ...
}

// This struct is free to provide a nicer interface that is not polluted by the
// extra functions #[self_referencing] adds.
pub struct Friendly {
    internal: Internal,
}

impl Friendly {
    pub fn new(/* ... */) -> Self {
        // Complicated code here...
    }
    
    pub fn do_the_thing(&self) -> T {
        // More complicated code here....
    }
}
```

While this crate is `no_std` compatible, it still requires the `alloc` crate.

Version notes:
- Version `0.18.0` now correctly refuses to compile unsound usages of `with_mut`, but requires Rust 1.63 or later.
- Version `0.17.0` reintroduces type parameter support, but requires at least
  version 1.60 of the Rust toolchain.
- Version `0.16.0` fixes a potential soundness issue but removes template
  parameter support.
- Version `0.13.0` and later contain checks for additional situations which
  cause undefined behavior if not caught.
- Version `0.11.0` and later place restrictions on derive macros, earlier
  versions allowed using them in ways which could lead to undefined behavior if
  not used properly.
- Version `0.10.0` and later automatically box every field. This is done
  to prevent undefined behavior, but has the side effect of making the library
  easier to work with.

Tests are located in the examples/ folder because they need to be in a crate
outside of `ouroboros` for the `self_referencing` macro to work properly.
