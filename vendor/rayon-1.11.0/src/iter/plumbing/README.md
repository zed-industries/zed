# Parallel Iterators

These are some notes on the design of the parallel iterator traits.
This file does not describe how to **use** parallel iterators.

## The challenge

Parallel iterators are more complicated than sequential iterators.
The reason is that they have to be able to split themselves up and
operate in parallel across the two halves.

The current design for parallel iterators has two distinct modes in
which they can be used; as we will see, not all iterators support both
modes (which is why there are two):

- **Pull mode** (the `Producer` and `UnindexedProducer` traits): in this mode,
  the iterator is asked to produce the next item using a call to `next`. This
  is basically like a normal iterator, but with a twist: you can split the
  iterator in half to produce disjoint items in separate threads.
  - in the `Producer` trait, splitting is done with `split_at`, which accepts
    an index where the split should be performed. Only indexed iterators can
    work in this mode, as they know exactly how much data they will produce,
    and how to locate the requested index.
  - in the `UnindexedProducer` trait, splitting is done with `split`, which
    simply requests that the producer divide itself *approximately* in half.
    This is useful when the exact length and/or layout is unknown, as with
    `String` characters, or when the length might exceed `usize`, as with
    `Range<u64>` on 32-bit platforms.
    - In theory, any `Producer` could act unindexed, but we don't currently
      use that possibility.  When you know the exact length, a `split` can
      simply be implemented as `split_at(length/2)`.
- **Push mode** (the `Consumer` and `UnindexedConsumer` traits): in
  this mode, the iterator instead is *given* each item in turn, which
  is then processed. This is the opposite of a normal iterator. It's
  more like a `for_each` call: each time a new item is produced, the
  `consume` method is called with that item. (The traits themselves are
  a bit more complex, as they support state that can be threaded
  through and ultimately reduced.) Like producers, there are two
  variants of consumers which differ in how the split is performed:
  - in the `Consumer` trait, splitting is done with `split_at`, which
    accepts an index where the split should be performed. All
    iterators can work in this mode. The resulting halves thus have an
    idea about how much data they expect to consume.
  - in the `UnindexedConsumer` trait, splitting is done with
    `split_off_left`.  There is no index: the resulting halves must be
    prepared to process any amount of data, and they don't know where that
    data falls in the overall stream.
    - Not all consumers can operate in this mode. It works for
      `for_each` and `reduce`, for example, but it does not work for
      `collect_into_vec`, since in that case the position of each item is
      important for knowing where it ends up in the target collection.

## How iterator execution proceeds

We'll walk through this example iterator chain to start. This chain
demonstrates more-or-less the full complexity of what can happen.

```rust
vec1.par_iter()
    .zip(vec2.par_iter())
    .flat_map(some_function)
    .for_each(some_other_function)
```

To handle an iterator chain, we start by creating consumers. This
works from the end. So in this case, the call to `for_each` is the
final step, so it will create a `ForEachConsumer` that, given an item,
just calls `some_other_function` with that item. (`ForEachConsumer` is
a very simple consumer because it doesn't need to thread any state
between items at all.)

Now, the `for_each` call will pass this consumer to the base iterator,
which is the `flat_map`. It will do this by calling the `drive_unindexed`
method on the `ParallelIterator` trait. `drive_unindexed` basically
says "produce items for this iterator and feed them to this consumer";
it only works for unindexed consumers.

(As an aside, it is interesting that only some consumers can work in
unindexed mode, but all producers can *drive* an unindexed consumer.
In contrast, only some producers can drive an *indexed* consumer, but
all consumers can be supplied indexes. Isn't variance neat.)

As it happens, `FlatMap` only works with unindexed consumers anyway.
This is because flat-map basically has no idea how many items it will
produce. If you ask flat-map to produce the 22nd item, it can't do it,
at least not without some intermediate state. It doesn't know whether
processing the first item will create 1 item, 3 items, or 100;
therefore, to produce an arbitrary item, it would basically just have
to start at the beginning and execute sequentially, which is not what
we want. But for unindexed consumers, this doesn't matter, since they
don't need to know how much data they will get.

Therefore, `FlatMap` can wrap the `ForEachConsumer` with a
`FlatMapConsumer` that feeds to it. This `FlatMapConsumer` will be
given one item. It will then invoke `some_function` to get a parallel
iterator out. It will then ask this new parallel iterator to drive the
`ForEachConsumer`. The `drive_unindexed` method on `flat_map` can then
pass the `FlatMapConsumer` up the chain to the previous item, which is
`zip`. At this point, something interesting happens.

## Switching from push to pull mode

If you think about `zip`, it can't really be implemented as a
consumer, at least not without an intermediate thread and some
channels or something (or maybe coroutines). The problem is that it
has to walk two iterators *in lockstep*. Basically, it can't call two
`drive` methods simultaneously, it can only call one at a time. So at
this point, the `zip` iterator needs to switch from *push mode* into
*pull mode*.

You'll note that `Zip` is only usable if its inputs implement
`IndexedParallelIterator`, meaning that they can produce data starting
at random points in the stream. This need to switch to push mode is
exactly why. If we want to split a zip iterator at position 22, we
need to be able to start zipping items from index 22 right away,
without having to start from index 0.

Anyway, so at this point, the `drive_unindexed` method for `Zip` stops
creating consumers. Instead, it creates a *producer*, a `ZipProducer`,
to be exact, and calls the `bridge` function in the `internals`
module. Creating a `ZipProducer` will in turn create producers for
the two iterators being zipped. This is possible because they both
implement `IndexedParallelIterator`.

The `bridge` function will then connect the consumer, which is
handling the `flat_map` and `for_each`, with the producer, which is
handling the `zip` and its predecessors. It will split down until the
chunks seem reasonably small, then pull items from the producer and
feed them to the consumer.

## The base case

The other time that `bridge` gets used is when we bottom out in an
indexed producer, such as a slice or range.  There is also a
`bridge_unindexed` equivalent for - you guessed it - unindexed producers,
such as string characters.

<a name="producer-callback">

## What on earth is `ProducerCallback`?

We saw that when you call a parallel action method like
`par_iter.reduce()`, that will create a "reducing" consumer and then
invoke `par_iter.drive_unindexed()` (or `par_iter.drive()`) as
appropriate. This may create yet more consumers as we proceed up the
parallel iterator chain. But at some point we're going to get to the
start of the chain, or to a parallel iterator (like `zip()`) that has
to coordinate multiple inputs. At that point, we need to start
converting parallel iterators into producers.

The way we do this is by invoking the method `with_producer()`, defined on
`IndexedParallelIterator`. This is a callback scheme. In an ideal world,
it would work like this:

```rust
base_iter.with_producer(|base_producer| {
    // here, `base_producer` is the producer for `base_iter`
});
```

In that case, we could implement a combinator like `map()` by getting
the producer for the base iterator, wrapping it to make our own
`MapProducer`, and then passing that to the callback. Something like
this:

```rust
struct MapProducer<'f, P, F: 'f> {
    base: P,
    map_op: &'f F,
}

impl<I, F> IndexedParallelIterator for Map<I, F>
    where I: IndexedParallelIterator,
          F: MapOp<I::Item>,
{
    fn with_producer<CB>(self, callback: CB) -> CB::Output {
        let map_op = &self.map_op;
        self.base_iter.with_producer(|base_producer| {
            // Here `producer` is the producer for `self.base_iter`.
            // Wrap that to make a `MapProducer`
            let map_producer = MapProducer {
                base: base_producer,
                map_op: map_op
            };

            // invoke the callback with the wrapped version
            callback(map_producer)
        });
    }
});
```

This example demonstrates some of the power of the callback scheme.
It winds up being a very flexible setup. For one thing, it means we
can take ownership of `par_iter`; we can then in turn give ownership
away of its bits and pieces into the producer (this is very useful if
the iterator owns an `&mut` slice, for example), or create shared
references and put *those* in the producer. In the case of map, for
example, the parallel iterator owns the `map_op`, and we borrow
references to it which we then put into the `MapProducer` (this means
the `MapProducer` can easily split itself and share those references).
The `with_producer` method can also create resources that are needed
during the parallel execution, since the producer does not have to be
returned.

Unfortunately there is a catch. We can't actually use closures the way
I showed you. To see why, think about the type that `map_producer`
would have to have. If we were going to write the `with_producer`
method using a closure, it would have to look something like this:

```rust
pub trait IndexedParallelIterator: ParallelIterator {
    type Producer;
    fn with_producer<CB, R>(self, callback: CB) -> R
        where CB: FnOnce(Self::Producer) -> R;
    ...
}
```

Note that we had to add this associated type `Producer` so that
we could specify the argument of the callback to be `Self::Producer`.
Now, imagine trying to write that `MapProducer` impl using this style:

```rust
impl<I, F> IndexedParallelIterator for Map<I, F>
    where I: IndexedParallelIterator,
          F: MapOp<I::Item>,
{
    type MapProducer = MapProducer<'f, P::Producer, F>;
    //                             ^^ wait, what is this `'f`?

    fn with_producer<CB, R>(self, callback: CB) -> R
        where CB: FnOnce(Self::Producer) -> R
    {
        let map_op = &self.map_op;
        //  ^^^^^^ `'f` is (conceptually) the lifetime of this reference,
        //         so it will be different for each call to `with_producer`!
    }
}
```

This may look familiar to you: it's the same problem that we have
trying to define an `Iterable` trait. Basically, the producer type
needs to include a lifetime (here, `'f`) that refers to the body of
`with_producer` and hence is not in scope at the impl level.

If we had [associated type constructors][1598], we could solve this
problem that way. But there is another solution. We can use a
dedicated callback trait like `ProducerCallback`, instead of `FnOnce`:

[1598]: https://github.com/rust-lang/rfcs/pull/1598

```rust
pub trait ProducerCallback<T> {
    type Output;
    fn callback<P>(self, producer: P) -> Self::Output
        where P: Producer<Item=T>;
}
```

Using this trait, the signature of `with_producer()` looks like this:

```rust
fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output;
```

Notice that this signature **never has to name the producer type** --
there is no associated type `Producer` anymore. This is because the
`callback()` method is generically over **all** producers `P`.

The problem is that now the `||` sugar doesn't work anymore. So we
have to manually create the callback struct, which is a mite tedious.
So our `MapProducer` code looks like this:

```rust
impl<I, F> IndexedParallelIterator for Map<I, F>
    where I: IndexedParallelIterator,
          F: MapOp<I::Item>,
{
    fn with_producer<CB>(self, callback: CB) -> CB::Output
        where CB: ProducerCallback<Self::Item>
    {
        return self.base.with_producer(Callback { callback: callback, map_op: self.map_op });
        //                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                             Manual version of the closure sugar: create an instance
        //                             of a struct that implements `ProducerCallback`.

        // The struct declaration. Each field is something that need to capture from the
        // creating scope.
        struct Callback<CB, F> {
            callback: CB,
            map_op: F,
        }

        // Implement the `ProducerCallback` trait. This is pure boilerplate.
        impl<T, F, CB> ProducerCallback<T> for Callback<CB, F>
            where F: MapOp<T>,
                  CB: ProducerCallback<F::Output>
        {
            type Output = CB::Output;

            fn callback<P>(self, base: P) -> CB::Output
                where P: Producer<Item=T>
            {
                // The body of the closure is here:
                let producer = MapProducer { base: base,
                                             map_op: &self.map_op };
                self.callback.callback(producer)
            }
        }
    }
}
```

OK, a bit tedious, but it works!
