# Futures

A notable point about Rust is [*fearless concurrency*](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html). That is the notion that you should be empowered to do concurrent things, without giving up safety. Also, Rust being a low-level language, it's about fearless concurrency *without picking a specific implementation strategy*. This means we *must* abstract over the strategy, to allow choice *later*, if we want to have any way to share code between users of different strategies.

Futures abstract over *computation*. They describe the "what", independent of the "where" and the "when". For that, they aim to break code into small, composable actions that can then be executed by a part of our system. Let's take a tour through what it means to compute things to find where we can abstract.

## Send and Sync

Luckily, concurrent Rust already has two well-known and effective concepts abstracting over sharing between concurrent parts of a program: `Send` and `Sync`. Notably, both the `Send` and `Sync` traits abstract over *strategies* of concurrent work, compose neatly, and don't prescribe an implementation.

As a quick summary:

- `Send` abstracts over *passing data* in a computation to another concurrent computation (let's call it the receiver), losing access to it on the sender side. In many programming languages, this strategy is commonly implemented, but missing support from the language side, and expects you to enforce the "losing access" behaviour yourself. This is a regular source of bugs: senders keeping handles to sent things around and maybe even working with them after sending. Rust mitigates this problem by making this behaviour known. Types can be `Send` or not (by implementing the appropriate marker trait), allowing or disallowing sending them around, and the ownership and borrowing rules prevent subsequent access.

- `Sync` is about *sharing data* between two concurrent parts of a program. This is another common pattern: as writing to a memory location or reading while another party is writing is inherently unsafe, this access needs to be moderated through synchronisation.[^1] There are many common ways for two parties to agree on not using the same part in memory at the same time, for example mutexes and spinlocks. Again, Rust gives you the option of (safely!) not caring. Rust gives you the ability to express that something *needs* synchronisation while not being specific about the *how*.

Note how we avoided any word like *"thread"*, but instead opted for "computation". The full power of `Send` and `Sync` is that they relieve you of the burden of knowing *what* shares. At the point of implementation, you only need to know which method of sharing is appropriate for the type at hand. This keeps reasoning local and is not influenced by whatever implementation the user of that type later uses.

`Send` and `Sync` can be composed in interesting fashions, but that's beyond the scope here. You can find examples in the [Rust Book][rust-book-sync].

[rust-book-sync]: https://doc.rust-lang.org/stable/book/ch16-04-extensible-concurrency-sync-and-send.html

To sum up: Rust gives us the ability to safely abstract over important properties of concurrent programs, their data sharing. It does so in a very lightweight fashion; the language itself only knows about the two markers `Send` and `Sync` and helps us a little by deriving them itself, when possible. The rest is a library concern.

## An easy view of computation

While computation is a subject to write a whole [book](https://computationbook.com/) about, a very simplified view suffices for us: A sequence of composable operations which can branch based on a decision, run to succession and yield a result or yield an error

## Deferring computation

As mentioned above, `Send` and `Sync` are about data. But programs are not only about data, they also talk about *computing* the data. And that's what [`Futures`][futures] do. We are going to have a close look at how that works in the next chapter. Let's look at what Futures allow us to express, in English. Futures go from this plan:

- Do X
- If X succeeded, do Y

towards:

- Start doing X
- Once X succeeds, start doing Y

Remember the talk about "deferred computation" in the intro? That's all it is. Instead of telling the computer what to execute and decide upon *now*, you tell it what to start doing and how to react on potential events in the... well... `Future`.

[futures]: https://doc.rust-lang.org/std/future/trait.Future.html

## Orienting towards the beginning

Let's have a look at a simple function, specifically the return value:

```rust,edition2018
# use std::{fs::File, io, io::prelude::*};
#
fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

You can call that at any time, so you are in full control on when you call it. But here's the problem: the moment you call it, you transfer control to the called function until it returns a value - eventually.
Note that this return value talks about the past. The past has a drawback: all decisions have been made. It has an advantage: the outcome is visible. We can unwrap the results of the program's past computation, and then decide what to do with it.

But we wanted to abstract over *computation* and let someone else choose how to run it. That's fundamentally incompatible with looking at the results of previous computation all the time. So, let's find a type that *describes* a computation without running it. Let's look at the function again:

```rust,edition2018
# use std::{fs::File, io, io::prelude::*};
#
fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

Speaking in terms of time, we can only take action *before* calling the function or *after* the function returned. This is not desirable, as it takes from us the ability to do something *while* it runs. When working with parallel code, this would take from us the ability to start a parallel task while the first runs (because we gave away control).

This is the moment where we could reach for [threads](https://en.wikipedia.org/wiki/Thread_). But threads are a very specific concurrency primitive and we said that we are searching for an abstraction.

What we are searching for is something that represents ongoing work towards a result in the future. Whenever we say "something" in Rust, we almost always mean a trait. Let's start with an incomplete definition of the `Future` trait:

```rust,edition2018
# use std::{pin::Pin, task::{Context, Poll}};
#
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
```

Looking at it closely, we see the following:

- It is generic over the `Output`.
- It provides a function called `poll`, which allows us to check on the state of the current computation.
- (Ignore `Pin` and `Context` for now, you don't need them for high-level understanding.)

Every call to `poll()` can result in one of these two cases:

1. The computation is done, `poll` will return [`Poll::Ready`](https://doc.rust-lang.org/std/task/enum.Poll.html#variant.Ready)
2. The computation has not finished executing, it will return [`Poll::Pending`](https://doc.rust-lang.org/std/task/enum.Poll.html#variant.Pending)

This allows us to externally check if a `Future` still has unfinished work, or is finally done and can give us the value. The most simple (but not efficient) way would be to just constantly poll futures in a loop. There are optimisations possible, and this is what a good runtime does for you.
Note that calling `poll` again after case 1 happened may result in confusing behaviour. See the [futures-docs](https://doc.rust-lang.org/std/future/trait.Future.html) for details.

## Async

While the `Future` trait has existed in Rust for a while, it was inconvenient to build and describe them. For this, Rust now has a special syntax: `async`. The example from above, implemented with `async-std`, would look like this:

```rust,edition2018
# extern crate async_std;
# use async_std::{fs::File, io, io::prelude::*};
#
async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
```

Amazingly little difference, right? All we did is label the function `async` and insert 2 special commands: `.await`.

This `async` function sets up a deferred computation. When this function is called, it will produce a `Future<Output = io::Result<String>>` instead of immediately returning a `io::Result<String>`. (Or, more precisely, generate a type for you that implements `Future<Output = io::Result<String>>`.)

## What does `.await` do?

The `.await` postfix does exactly what it says on the tin: the moment you use it, the code will wait until the requested action (e.g. opening a file or reading all data in it) is finished. The `.await?` is not special, it's just the application of the `?` operator to the result of `.await`. So, what is gained over the initial code example? We're getting futures and then immediately waiting for them?

The `.await` points act as a marker. Here, the code will wait for a `Future` to produce its value. How will a future finish? You don't need to care! The marker allows the component (usually called the “runtime”) in charge of *executing* this piece of code to take care of all the other things it has to do while the computation finishes. It will come back to this point when the operation you are doing in the background is done. This is why this style of programming is also called *evented programming*. We are waiting for *things to happen* (e.g. a file to be opened) and then react (by starting to read).

When executing 2 or more of these functions at the same time, our runtime system is then able to fill the wait time with handling *all the other events* currently going on.

## Conclusion

Working from values, we searched for something that expresses *working towards a value available later*. From there, we talked about the concept of polling.

A `Future` is any data type that does not represent a value, but the ability to *produce a value at some point in the future*. Implementations of this are very varied and detailed depending on use-case, but the interface is simple.

Next, we will introduce you to `tasks`, which we will use to actually *run* Futures.

[^1]: Two parties reading while it is guaranteed that no one is writing is always safe.

[futures]: https://rust-lang.github.io/async-book/02_execution/02_future.html
