# State machines generation using ragel

HarfBuzz uses [ragel](https://github.com/adrian-thurston/ragel) to generate
state machines for some shapers.
The produced C code is rather low-level and relies on `goto` a lot,
therefore converting it to Rust by hand is rather problematic.
And this is the code that updates relatively often.

Can we use `ragel` to generate Rust code directly? Not using the stable `ragel`.
But the latest development branch does have a native Rust support.
Therefore to use `ragel` directly, we have to use the latest devel version and modify
ragel scripts a bit.

## Build ragel

`ragel` is a C++ project that uses the dreadful `autotools`.
Good luck building it on Windows. But on Unix-like OSes it should be relatively easy.

It also requires a 3rdparty dependency of
[colm](https://github.com/adrian-thurston/colm), which we have to build first.

In case of macOS, we would need:

```sh
brew install automake autoconf libtool
```

And now we can build `ragel`:

```sh
# build `colm` first
git clone https://github.com/adrian-thurston/colm
cd colm
./autogen.sh
./configure --prefix=/path/to/colm/install # prefer a custom path to /usr/local
make
make install

cd ..
git clone https://github.com/adrian-thurston/ragel
./autogen.sh
# --with-colm takes the same path we used above
./configure --prefix=/path/to/ragel/install --with-colm=/path/to/colm/install
make
make install
```

## Running ragel

Now we can convert our ragel scripts using:

```sh
/path/to/ragel/install/bin/ragel-rust -e -F1 src/hb/ot_shape_complex_indic_machine.rl # or any other .rl file
```

That's it!

PS: `ragel` will create temporary `*.ri` files. They can be safely removed.

## Code format

`ragel` uses strange code formatting for the Rust output, therefore running `cargo fmt`
is required.

## The Universal state machine

The `universal_machine.rl` is special since we have to modify its output manually
because we cannot express our needs via `ragel` directly.

The change is simple. After the `universal_machine.rs` is generated, Rust will complain about
about some variables set to 0. Like `ts = 0;`.
In all those cases `0` should simply be replaced with `p0`.
There are no better solution for now...
