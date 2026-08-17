# Rayon FAQ

This file is for general questions that don't fit into the README or crate docs.

## How many threads will Rayon spawn?

By default, Rayon uses the same number of threads as the number of CPUs
available. Note that on systems with hyperthreading enabled this equals the
number of logical cores and not the physical ones.

If you want to alter the number of threads spawned, you can set the
environmental variable `RAYON_NUM_THREADS` to the desired number of threads or
use the
[`ThreadPoolBuilder::build_global` function](https://docs.rs/rayon/*/rayon/struct.ThreadPoolBuilder.html#method.build_global)
method.

## How does Rayon balance work between threads?

Behind the scenes, Rayon uses a technique called **work stealing** to try and
dynamically ascertain how much parallelism is available and exploit it. The idea
is very simple: we always have a pool of worker threads available, waiting for
some work to do. When you call `join` the first time, we shift over into that
pool of threads. But if you call `join(a, b)` from a worker thread W, then W
will place `b` into its work queue, advertising that this is work that other
worker threads might help out with. W will then start executing `a`.

While W is busy with `a`, other threads might come along and take `b` from its
queue. That is called *stealing* `b`. Once `a` is done, W checks whether `b` was
stolen by another thread and, if not, executes `b` itself. If W runs out of jobs
in its own queue, it will look through the other threads' queues and try to
steal work from them.

This technique is not new. It was first introduced by the [Cilk project][cilk],
done at MIT in the late nineties. The name Rayon is an homage to that work.

[cilk]: http://supertech.csail.mit.edu/cilk/

## What should I do if I use `Rc`, `Cell`, `RefCell` or other non-Send-and-Sync types?

There are a number of non-threadsafe types in the Rust standard library, and if
your code is using them, you will not be able to combine it with Rayon.
Similarly, even if you don't have such types, but you try to have multiple
closures mutating the same state, you will get compilation errors; for example,
this function won't work, because both closures access `slice`:

```rust
/// Increment all values in slice.
fn increment_all(slice: &mut [i32]) {
    rayon::join(|| process(slice), || process(slice));
}
```

The correct way to resolve such errors will depend on the case. Some cases are
easy: for example, uses of [`Rc`] can typically be replaced with [`Arc`], which
is basically equivalent, but thread-safe.

Code that uses `Cell` or `RefCell`, however, can be somewhat more complicated.
If you can refactor your code to avoid those types, that is often the best way
forward, but otherwise, you can try to replace those types with their threadsafe
equivalents:

- `Cell` -- replacement: `AtomicUsize`, `AtomicBool`, etc
- `RefCell` -- replacement: `RwLock`, or perhaps `Mutex`

However, you have to be wary! The parallel versions of these types have
different atomicity guarantees. For example, with a `Cell`, you can increment a
counter like so:

```rust
let value = counter.get();
counter.set(value + 1);
```

But when you use the equivalent `AtomicUsize` methods, you are actually
introducing a potential race condition (not a data race, technically, but it can
be an awfully fine distinction):

```rust
let value = tscounter.load(Ordering::SeqCst);
tscounter.store(value + 1, Ordering::SeqCst);
```

You can already see that the `AtomicUsize` API is a bit more complex, as it
requires you to specify an
[ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html). (I
won't go into the details on ordering here, but suffice to say that if you don't
know what an ordering is, and probably even if you do, you should use
`Ordering::SeqCst`.) The danger in this parallel version of the counter is that
other threads might be running at the same time and they could cause our counter
to get out of sync. For example, if we have two threads, then they might both
execute the "load" before either has a chance to execute the "store":

```
Thread 1                                          Thread 2
let value = tscounter.load(Ordering::SeqCst);
// value = X                                      let value = tscounter.load(Ordering::SeqCst);
                                                  // value = X
tscounter.store(value+1);                         tscounter.store(value+1);
// tscounter = X+1                                // tscounter = X+1
```

Now even though we've had two increments, we'll only increase the counter by
one! Even though we've got no data race, this is still probably not the result
we wanted. The problem here is that the `Cell` API doesn't make clear the scope
of a "transaction" -- that is, the set of reads/writes that should occur
atomically. In this case, we probably wanted the get/set to occur together.

In fact, when using the `Atomic` types, you very rarely want a plain `load` or
plain `store`. You probably want the more complex operations. A counter, for
example, would use `fetch_add` to atomically load and increment the value in one
step. Compare-and-swap is another popular building block.

A similar problem can arise when converting `RefCell` to `RwLock`, but it is
somewhat less likely, because the `RefCell` API does in fact have a notion of a
transaction: the scope of the handle returned by `borrow` or `borrow_mut`. So if
you convert each call to `borrow` to `read` (and `borrow_mut` to `write`),
things will mostly work fine in a parallel setting, but there can still be
changes in behavior. Consider using a `handle: RefCell<Vec<i32>>` like:

```rust
let len = handle.borrow().len();
for i in 0 .. len {
    let data = handle.borrow()[i];
    println!("{}", data);
}
```

In sequential code, we know that this loop is safe. But if we convert this to
parallel code with an `RwLock`, we do not: this is because another thread could
come along and do `handle.write().unwrap().pop()`, and thus change the length of
the vector. In fact, even in *sequential* code, using very small borrow sections
like this is an anti-pattern: you ought to be enclosing the entire transaction
together, like so:

```rust
let vec = handle.borrow();
let len = vec.len();
for i in 0 .. len {
    let data = vec[i];
    println!("{}", data);
}
```

Or, even better, using an iterator instead of indexing:

```rust
let vec = handle.borrow();
for data in vec {
    println!("{}", data);
}
```

There are several reasons to prefer one borrow over many. The most obvious is
that it is more efficient, since each borrow has to perform some safety checks.
But it's also more reliable: suppose we modified the loop above to not just
print things out, but also call into a helper function:

```rust
let vec = handle.borrow();
for data in vec {
    helper(...);
}
```

And now suppose, independently, this helper fn evolved and had to pop something
off of the vector:

```rust
fn helper(...) {
    handle.borrow_mut().pop();
}
```

Under the old model, where we did lots of small borrows, this would yield
precisely the same error that we saw in parallel land using an `RwLock`: the
length would be out of sync and our indexing would fail (note that in neither
case would there be an actual *data race* and hence there would never be
undefined behavior). But now that we use a single borrow, we'll see a borrow
error instead, which is much easier to diagnose, since it occurs at the point of
the `borrow_mut`, rather than downstream. Similarly, if we move to an `RwLock`,
we'll find that the code either deadlocks (if the write is on the same thread as
the read) or, if the write is on another thread, works just fine. Both of these
are preferable to random failures in my experience.

## But wait, isn't Rust supposed to free me from this kind of thinking?

You might think that Rust is supposed to mean that you don't have to think about
atomicity at all. In fact, if you avoid interior mutability (`Cell` and
`RefCell` in a sequential setting, or `AtomicUsize`, `RwLock`, `Mutex`, et al.
in parallel code), then this is true: the type system will basically guarantee
that you don't have to think about atomicity at all. But often there are times
when you WANT threads to interleave in the ways I showed above.

Consider for example when you are conducting a search in parallel, say to find
the shortest route. To avoid fruitless search, you might want to keep a cell
with the shortest route you've found thus far. This way, when you are searching
down some path that's already longer than this shortest route, you can just stop
and avoid wasted effort. In sequential land, you might model this "best result"
as a shared value like `Rc<Cell<usize>>` (here the `usize` represents the length
of best path found so far); in parallel land, you'd use a `Arc<AtomicUsize>`.

```rust
fn search(path: &Path, cost_so_far: usize, best_cost: &AtomicUsize) {
    if cost_so_far >= best_cost.load(Ordering::SeqCst) {
        return;
    }
    // Using `fetch_min` to avoid a race condition, in case it changed since `load`.
    best_cost.fetch_min(..., Ordering::SeqCst);
}
```

Now in this case, we really WANT to see results from other threads interjected
into our execution!
