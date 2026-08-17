# Tasks

Now that we know what Futures are, we want to run them!

In `async-std`, the [`task`][tasks] module is responsible for this. The simplest way is using the `block_on` function:

```rust,edition2018
# extern crate async_std;
use async_std::{fs::File, io, prelude::*, task};

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

fn main() {
    let reader_task = task::spawn(async {
        let result = read_file("data.csv").await;
        match result {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error reading file: {:?}", e)
        }
    });
    println!("Started task!");
    task::block_on(reader_task);
    println!("Stopped task!");
}
```

This asks the runtime baked into `async_std` to execute the code that reads a file. Let's go one by one, though, inside to outside.

```rust,edition2018
# extern crate async_std;
# use async_std::{fs::File, io, prelude::*, task};
#
# async fn read_file(path: &str) -> io::Result<String> {
#     let mut file = File::open(path).await?;
#     let mut contents = String::new();
#     file.read_to_string(&mut contents).await?;
#     Ok(contents)
# }
#
async {
    let result = read_file("data.csv").await;
    match result {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error reading file: {:?}", e)
    }
};
```

This is an `async` *block*. Async blocks are necessary to call `async` functions, and will instruct the compiler to include all the relevant instructions to do so. In Rust, all blocks return a value and `async` blocks happen to return a value of the kind `Future`.

But let's get to the interesting part:

```rust,edition2018
# extern crate async_std;
# use async_std::task;
task::spawn(async { });
```

`spawn` takes a `Future` and starts running it on a `Task`. It returns a `JoinHandle`. Futures in Rust are sometimes called *cold* Futures. You need something that starts running them. To run a Future, there may be some additional bookkeeping required, e.g. whether it's running or finished, where it is being placed in memory and what the current state is. This bookkeeping part is abstracted away in a `Task`.

A `Task` is similar to a `Thread`, with some minor differences: it will be scheduled by the program instead of the operating system kernel, and if it encounters a point where it needs to wait, the program itself is responsible for waking it up again. We'll talk a little bit about that later. An `async_std` task can also have a name and an ID, just like a thread.

For now, it is enough to know that once you have `spawn`ed a task, it will continue running in the background. The `JoinHandle` is itself a future that will finish once the `Task` has run to conclusion. Much like with `threads` and the `join` function, we can now call `block_on` on the handle to *block* the program (or the calling thread, to be specific) and wait for it to finish.

## Tasks in `async_std`

Tasks in `async_std` are one of the core abstractions. Much like Rust's `thread`s, they provide some practical functionality over the raw concept. `Tasks` have a relationship to the runtime, but they are in themselves separate. `async_std` tasks have a number of desirable properties:

- They are allocated in one single allocation
- All tasks have a *backchannel*, which allows them to propagate results and errors to the spawning task through the `JoinHandle`
- They carry useful metadata for debugging
- They support task local storage

`async_std`s task API handles setup and teardown of a backing runtime for you and doesn't rely on a runtime being explicitly started.

## Blocking

`Task`s are assumed to run _concurrently_, potentially by sharing a thread of execution. This means that operations blocking an _operating system thread_, such as `std::thread::sleep` or io function from Rust's `std` library will _stop execution of all tasks sharing this thread_. Other libraries (such as database drivers) have similar behaviour. Note that _blocking the current thread_ is not in and of itself bad behaviour, just something that does not mix well with the concurrent execution model of `async-std`. Essentially, never do this:

```rust,edition2018
# extern crate async_std;
# use async_std::task;
fn main() {
    task::block_on(async {
        // this is std::fs, which blocks
        std::fs::read_to_string("test_file");
    })
}
```

If you want to mix operation kinds, consider putting such blocking operations on a separate `thread`.

## Errors and panics

Tasks report errors through normal patterns: If they are fallible, their `Output` should be of kind `Result<T,E>`.

In case of `panic`, behaviour differs depending on whether there's a reasonable part that addresses the `panic`. If not, the program _aborts_.

In practice, that means that `block_on` propagates panics to the blocking component:

```rust,edition2018,should_panic
# extern crate async_std;
# use async_std::task;
fn main() {
    task::block_on(async {
        panic!("test");
    });
}
```

```text
thread 'async-task-driver' panicked at 'test', examples/panic.rs:8:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

While panicking a spawned task will abort:

```rust,edition2018,should_panic
# extern crate async_std;
# use async_std::task;
# use std::time::Duration;
task::spawn(async {
    panic!("test");
});

task::block_on(async {
    task::sleep(Duration::from_millis(10000)).await;
})
```

```text
thread 'async-task-driver' panicked at 'test', examples/panic.rs:8:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
Aborted (core dumped)
```

That might seem odd at first, but the other option would be to silently ignore panics in spawned tasks. The current behaviour can be changed by catching panics in the spawned task and reacting with custom behaviour. This gives users the choice of panic handling strategy.

## Conclusion

`async_std` comes with a useful `Task` type that works with an API similar to `std::thread`. It covers error and panic behaviour in a structured and defined way.

Tasks are separate concurrent units and sometimes they need to communicate. That's where `Stream`s come in.

[tasks]: https://docs.rs/async-std/latest/async_std/task/index.html
