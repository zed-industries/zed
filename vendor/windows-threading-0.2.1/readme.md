## Windows threading

The [windows-threading](https://crates.io/crates/windows-threading) crate provides simple, safe, and efficient access to the Windows threading support.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-threading]
version = "0.2"
```

Use the Windows threading support as needed. Here is how you might submit a closure to run on the default thread pool:

```rust,no_run
windows_threading::submit(|| {
    println!("thread: {}", windows_threading::thread_id());

    loop {
        println!(".");
        windows_threading::sleep(1000);
    }
});
```

As you would expect, the closure will print the thread identifier of the pool thread it is occupying indefinitely and then print "." on one second intervals.

```text
thread: 27292
.
.
.
.
.
.
```

Here is how you might call a closure on each element of the iterator in parallel, waiting for all closures to finish:

```rust,no_run
let counter = std::sync::RwLock::<usize>::new(0);

windows_threading::for_each(0..10, |value| {
    println!("thread: {}, value: {value}", windows_threading::thread_id());
    let mut counter = counter.write().unwrap();
    *counter += value;
});

println!("\nshould be 45 = {}", counter.read().unwrap());
```

The resulting thread identifiers will be unpredictable and so will be the order of the values:

```text
thread: 44088, value: 0
thread: 36152, value: 1
thread: 36152, value: 3
thread: 36152, value: 4
thread: 36152, value: 5
thread: 36152, value: 7
thread: 36152, value: 8
thread: 44088, value: 2
thread: 41592, value: 6
thread: 34688, value: 9

should be 45 = 45
```

The `for_each` function uses a `Pool` object internally, which you can also use directly if you prefer:

```rust,no_run
let set = std::sync::RwLock::<std::collections::HashMap<u32, usize>>::default();
let pool = windows_threading::Pool::new();
pool.set_thread_limits(2, 10);
pool.scope(|pool| {
    for _ in 0..10 {
        pool.submit(|| {
            windows_threading::sleep(10);
            let mut writer = set.write().unwrap();
            *writer.entry(windows_threading::thread_id()).or_default() += 1;
        })
    }
});

println!("{:#?}", set.read().unwrap());
```

The `set_thread_limits(2, 10)` method is used to ensure that the pool includes at least two threads at all times and up to a maximum of 10. There is no reason to call `set_thread_limits` if you prefer the operating system to manage this dynamically. Calling `set_thread_limits(1, 1)` will for example ensure that all closures run on the same dedicated thread.

The `submit` method takes the closure and runs it on one of those threads.

The `join` method waits for all previously submitted closures to finish.

As you might expect, the resulting distribution of closures spans a number of threads.

```text
{
    25064: 3,
    13692: 2,
    40784: 2,
    29608: 3,
}
```

Removing the `sleep` call will likely produce very different results:

```text
{
    22720: 10,
}
```

This is because the thread pool is careful not to overschedule and will happily reuse a small number of threads when the closures finish quickly.
