# oorandom

[![Crates.io](https://img.shields.io/crates/v/oorandom.svg)](https://crates.io/crates/oorandom)
[![Docs](https://docs.rs/oorandom/badge.svg)](https://docs.rs/oorandom)
[![builds.sr.ht status](https://builds.sr.ht/~icefox/oorandom.svg)](https://builds.sr.ht/~icefox/oorandom?)

# What is this?

`oorandom` is a minimalistic pseudorandom number generator in Rust.  For
those times when the `rand` crate is just too big and you want something
a bit dumber.

More specifically, it implements ONE prng, which is currently a permuted
congruential generator (PCG).  It may change if something better comes
along, but that seems unlikely and will probably be a major version
bump.  It will give you `u32` or `u64`, and signed or floating-point
equivalents.  It is also `#[no_std]`.  Anything else is gravy.

Thanks to Lokathor for making
[`randomize`](https://github.com/Lokathor/randomize), which inspired me
to do my own equivalent.

The name comes from my attempts to find a good way to pronounce
`/dev/urandom`.

Please direct questions, discussions and bugs to the [issue
tracker](https://todo.sr.ht/~icefox/oorandom).

# Why use `oorandom` instead of...

 * `rand` -- `oorandom` is simpler and has zero choices you need to
   make.  It also compiles in 1/10th the time and has a stable API.
 * `getrandom` -- They solve different problems; `getrandom` gives you
   whatever secure randomness the OS decides to give you, not a
   deterministic and seedable PRNG.  It's generally a good idea to use
   `getrandom` to seed this RNG though.
 * `randomize` -- `randomize` used to be more complicated, but
   `randomize` 3.x is quite similar to `oorandom` in functionality and
   design.  Go for it.
 * `rand_pcg` and `rand_core` -- Yes you can take `rand` apart into its
   pieces and use those individually, if you want to abandon having an
   all-in-one solution, still deal with the lack of stability in
   `rand_core` and actually figure out which pieces you need.  It works
   just fine.  Seems more complicated than it needs to be though.
 * `nanorand` -- `nanorand` uses the
   [WyRand](https://github.com/wangyi-fudan/wyhash) PRNG algorithm,
   which is supposedly faster than PCG and at least as good quality.  I
   haven't verified these claims, and I don't know of any *really*
   thorough 3rd party investigation into them, though it apparently
   passes [Dr. Lemire's tests](https://github.com/lemire/testingRNG).
   So for now I personally consider WyRand to be in the "trust but
   verify" level of quality.  It's probably fine.  Check back in 2027.
 * `fastrand` -- Looks fine, uses the same algorithm as `oorandom`.
   Made by the same people as the `smol` async runtime, which may be
   good or bad for you.  Does slightly more than `oorandom` does, which
   may be good or bad for you.  Use it if you like it.

# This is not...

This is not cryptographically secure, and if you use it for crypto you
will get what you deserve.  You are also in charge of choosing a useful
seed; the `getrandom` crate might be useful for that.

This is also not optimized to be stupidly fast, but is basically just
as fast as `rustc` feels like making it.  This means it is safe and robust
and portable and involves absolutely zero clever tricks.

# Usage

```rust
use oorandom;
fn main() {
    let some_seed = 4;
    let mut rng = oorandom::Rand32::new(some_seed);
    println!("Your random number is: {}", rng.rand_float());
}
```

If you want a nondeterministic seed, I recommend using the `getrandom` crate to produce one.

# License

MIT

# A brief history of random numbers

The usefulness of random numbers has been known for a long, long time
to people who also knew how to use slide rules.  If you wanted to do
some math without the bother of coming up with all that pesky input
data from the real world, you might as well just use any ol' random numbers,
as long as there weren't any patterns in them to heck up the patterns you were
trying to look at.  So in the first half
of the 20th century you had little old ladies named Edith spinning
roulette wheels or pulling bingo balls out of baskets and writing the
results down, which got assembled into giant tomes and published so
that engineering schools could buy them and have giant tomes sitting
on their shelves.  Anyone who wanted some meaningless numbers could
pull the tome down, flip it open to a presumably-random page, and
there were all the random numbers anyone could want.  The problem was
solved, and life was good.

In late 1940's computers were invented, but they were far too big and
expensive to be put on the task of *intentionally* generating
nonsense, and things carried on as before.  If you needed random
numbers in a computer program, you just got a pretty young lady named
Mary to transcribe part of the book to punch cards for you.

Around the early 1960's computers got fast enough that Edith and Mary
couldn't keep up with them, so they got downsized and replaced with
more computers.  To do this people came up with Linear Congruential
Generators (LCG's), which could generate lots of numbers numbers that
weren't really random, but sure looked random.  LCG's worked well on
computers that even a second-rate university could afford, and so the
problem was solved, and life was good.

At some unknown point in here, presumably sometime in the 60's or 70's,
someone seems to have invented Linear Feedback Shift Registers (LFSR's)
as well.  These made random-looking numbers and were really easy to
implement in hardware compared to the LCG, which needed to do
complicated things like multiply numbers.  The random-looking numbers
made by LFSR's were good enough for hardware people, so they started
using LFSR's whenever they needed to and never looked back.

Anyway, by the late 60's people who knew how to use slide rules had
realized that using numbers that only *looked* random could really heck
up their math pretty bad, and one of the more common LCG implmentations,
RANDU, was actually about as bad as possible.  So, just using any old
LCG wasn't good enough, you had to use one made by someone with a PhD in
mathematics.  Donald Knuth shook his fist at the world and shouted "Hah!
I told you so!", published a book on how to do it Right that most people
didn't read, and then went back into his Fortress of Solitude to write
TeX.  Because it was created by IBM, RANDU's awfulness is now enshrined
forever in history documents like this one, and because the people
writing OS's and programming languages at the time weren't actually
doing much slide-rule stuff anymore and didn't actually *need* very good
random-looking numbers, everyone went back to using whatever old crap
RNG they were using anyway.  The problem was solved, or at least not
terribly problematic, and life was good.

Also, sometime in the 70's or 80's the arts of cryptography started
leaking from classified government works into the real world.  People
started thinking about how much money they could make from scrambling
satellite TV so that plebs with HAM radio licenses couldn't watch it,
and these people started giving more money to people who had PhD's in
mathematics to figure out how to make this work.  It was quickly
determined that neither LCG's nor LFSR's made numbers that were
random-looking enough to really get in the way of someone who knew how
to use a slide rule, and since Edith had long ago retired to a small
beach house in New Jersey, they needed to figure out how to get
computers to make better random-looking numbers.  But making numbers
look random enough that someone couldn't undo the process and get free
pay-per-view was slow and involved lots of details that nobody else
really cared about, so that topic went off on its own adventures and
will not be further mentioned.

Things more or less trundled along this way until the late 90's, when
suddenly computers were everywhere and there was a new generation of
people who had grown up too late to know how to use slide rules, so they
did all their math with computers.  They were doing a LOT of math by
now, and they looked around and realized that their random-looking
numbers really weren't very random-looking at all, and this was actually
something of a problem by now.  So the Mersenne Twister got invented.
It was pretty slow and used a lot of memory and made kinda mediocre
random numbers, but it was way better than a bad LCG, and most
importantly, it had a cool name. Most people didn't want to read Knuth's
book and figure out how to make a non-bad LCG, so everyone started using
the Mersenne Twister whenever possible.  The problem was solved, and
life was good.

This is where things stood until the early 2010's, when I finished my MS
and started paying attention again.  People suddenly realized it was
possible to make random-looking numbers better than the Mersenne Twister
using an algorithm called xorshift.  Xorshift was fast, it made good
pretty random-looking numbers, and it didn't need a whole 3 kilobytes of
state just sitting around taking up space and causing comment among the
neighbors at church.  It did sometimes have problems with some of its
numbers not looking random enough in a few select circumstances, but
people were gun-shy about their randomness by now so a few people with
PhD's in mathematics slowly and patiently spent years figuring out ways
to work around these problems, leading to a whole confusing family of
related things such as xoshiro, xoroshiro, xoroshiro+, xoroshiro*, and
so on.  Nobody else could really tell the difference between them, but
everyone agreed they were better than Mersenne Twister, easier to
implement, and the name was nearly as cool.  Many papers were published,
the problem was solved, and life was good.

However, at about the same time some bright young spark figured out that
it actually wasn't too hard, if you read Knuth's book and thought real
hard about what you were doing, to take the old LCG and hop it up on
cocaine and moon juice.  The result got called the Permuted Congruential
Generator, or PCG.  This quite miffed the people working on xorshift
generators by being almost as small and fast, and producing
random-looking numbers that satisfied even the people who had learned to
use slide rules for fun in this latter age.  It also used  xor's and bit
shifts, and that's xorshift's turf, dammit, it's right in the name!
Since nobody had figured out any downsides to PCG's yet, everyone
shrugged and said "might as well just go with that then", and that is
where, as of 2019, the art currently stands.  The problem is solved, and
life is good.

