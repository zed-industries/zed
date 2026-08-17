[![crates.io](https://img.shields.io/crates/v/any_vec.svg)](https://crates.io/crates/any_vec)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![Docs](https://docs.rs/any_vec/badge.svg)](https://docs.rs/any_vec)
[![CI](https://github.com/tower120/any_vec/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/tower120/any_vec/actions/workflows/ci.yml)

Type erased vector. All elements have the same type.

Designed to be type-erased as far as possible - most operations do not know a concrete type.
For example, you can move or copy/clone items from one type-erased vector to another, without ever knowing
their type. Or you can erase, swap, move, copy elements inside type-erased vector, etc...

Only type-erased destruct and clone operations have additional overhead of indirect call.

# Usage

```rust
    let mut vec: AnyVec = AnyVec::new::<String>();
    {
        // Typed operations.
        let mut vec = vec.downcast_mut::<String>().unwrap();
        vec.push(String::from("0"));
        vec.push(String::from("1"));
        vec.push(String::from("2"));
    }
 
    let mut other_vec: AnyVec = AnyVec::new::<String>();
    // Fully type erased element move from one vec to another
    // without intermediate mem-copies.
    let element = vec.swap_remove(0);
    other_vec.push(element);

    // Output 2 1
    for s in vec.downcast_ref::<String>().unwrap(){
        println!("{}", s);
    } 
```

See [documentation](https://docs.rs/any_vec) for more.

## Send, Sync, Clone

You can make `AnyVec` `Send`able, `Sync`able, `Clone`able:

```rust
use any_vec::AnyVec;
use any_vec::traits::*;
let v1: AnyVec<dyn Cloneable + Sync + Send> = AnyVec::new::<String>();
let v2 = v1.clone();
 ```
 This constraints will be applied compiletime to element type:
```rust
// This will fail to compile. 
let v1: AnyVec<dyn Sync + Send> = AnyVec::new::<Rc<usize>>();
```

Non-Clonable `AnyVec` has a size 1 pointer smaller. 

## LazyClone

 Whenever possible, `any_vec` type erased elements can be lazily cloned:

```rust
 let mut v1: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
 v1.push(AnyValueWrapper::new(String::from("0")));

 let mut v2: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
 let e = v1.swap_remove(0);
 v2.push(e.lazy_clone());
 v2.push(e.lazy_clone());
```

## MemBuilder

 `MemBuilder` + `Mem` works like `Allocator` for `AnyVec`. But unlike allocator,
 `Mem` container-specialized design allows to perform more optimizations. For example,
 it is possible to make stack-allocated `FixedAnyVec` and small-buffer-optimized(SBO) `SmallAnyVec`
 from `AnyVec` by just changing `MemBuilder`:

```rust
type FixedAnyVec<Traits = dyn None> = AnyVec<Traits, Stack<512>>;
let mut any_vec: FixedAnyVec = AnyVec::new::<String>();

// This will be on stack, without any allocations.
any_vec.push(AnyValueWrapper::new(String::from("0")))
```

 With help of `clone_empty_in` you can use stack allocated, or SBO `AnyVec`
 as fast intermediate storage for values of unknown type:

```rust
fn self_push_first_element<T: Trait + Cloneable>(any_vec: &mut AnyVec<T>){
    let mut tmp = any_vec.clone_empty_in(StackN::<1, 256>);
    tmp.push(any_vec.at(0).lazy_clone());
    any_vec.push(tmp.pop().unwrap());
}
```

 `MemBuilder` interface, being stateful, allow to make `Mem`, which can work with complex custom allocators.

## no_std + no_alloc

This is `no_std` library, which can work without `alloc` too.  

### Changelog

See [CHANGELOG.md](CHANGELOG.md) for version differences.

# Known alternatives

* [type_erased_vec](https://crates.io/crates/type_erased_vec). Allow to store `Vec<T>` in type erased way, 
but you need to perform operations, you need to "cast" to concrete type first.
* [untyped_vec](https://crates.io/crates/untyped_vec). Some operations like `len`, `capacity` performed without type
knowledge; but the rest - require concrete type.
