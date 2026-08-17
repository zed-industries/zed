//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::io`.

use crate::std_facade::String;
#[cfg(test)]
use crate::std_facade::Vec;
use std::io::ErrorKind::*;
use std::io::*;

use crate::arbitrary::*;
use crate::strategy::statics::static_map;
use crate::strategy::*;

// TODO: IntoInnerError
// Consider: std::io::Initializer

macro_rules! buffer {
    ($type: ident, $bound: path) => {
        arbitrary!(
            [A: Arbitrary + $bound] $type<A>,
            SMapped<(A, Option<u16>), Self>, A::Parameters;
            args => static_map(
                arbitrary_with(product_pack![args, Default::default()]),
                |(inner, cap)| {
                    if let Some(cap) = cap {
                        $type::with_capacity(cap as usize, inner)
                    } else {
                        $type::new(inner)
                    }
                }
            )
        );

        lift1!([$bound] $type<A>; base =>
            (base, any::<Option<u16>>()).prop_map(|(inner, cap)| {
                if let Some(cap) = cap {
                    $type::with_capacity(cap as usize, inner)
                } else {
                    $type::new(inner)
                }
            })
        );
    };
}

buffer!(BufReader, Read);
buffer!(BufWriter, Write);
buffer!(LineWriter, Write);

arbitrary!(
    [A: Read + Arbitrary, B: Read + Arbitrary] Chain<A, B>,
    SMapped<(A, B), Self>, product_type![A::Parameters, B::Parameters];
    args => static_map(arbitrary_with(args), |(a, b)| a.chain(b))
);

wrap_ctor!(Cursor);

lazy_just!(
      Empty, empty
    ; Sink, sink
    ; Stderr, stderr
    ; Stdin, stdin
    ; Stdout, stdout
);

wrap_ctor!([BufRead] Lines, BufRead::lines);

arbitrary!(Repeat, SMapped<u8, Self>; static_map(any::<u8>(), repeat));

arbitrary!(
    [A: BufRead + Arbitrary] Split<A>, SMapped<(A, u8), Self>, A::Parameters;
    args => static_map(
        arbitrary_with(product_pack![args, Default::default()]),
        |(a, b)| a.split(b)
    )
);
lift1!(['static + BufRead] Split<A>;
    base => (base, any::<u8>()).prop_map(|(a, b)| a.split(b)));

arbitrary!(
    [A: Read + Arbitrary] Take<A>, SMapped<(A, u64), Self>, A::Parameters;
    args => static_map(
        arbitrary_with(product_pack![args, Default::default()]),
        |(a, b)| a.take(b)
    )
);
lift1!(['static + Read] Take<A>;
    base => (base, any::<u64>()).prop_map(|(a, b)| a.take(b)));

arbitrary!(ErrorKind, Union<Just<Self>>;
    Union::new(
    [ NotFound
    , PermissionDenied
    , ConnectionRefused
    , ConnectionReset
    , ConnectionAborted
    , NotConnected
    , AddrInUse
    , AddrNotAvailable
    , BrokenPipe
    , AlreadyExists
    , WouldBlock
    , InvalidInput
    , InvalidData
    , TimedOut
    , WriteZero
    , Interrupted
    , Other
    , UnexpectedEof
    // TODO: watch this type for variant-additions.
    ].iter().cloned().map(Just))
);

arbitrary!(
    SeekFrom,
    TupleUnion<(
        WA<SMapped<u64, SeekFrom>>,
        WA<SMapped<i64, SeekFrom>>,
        WA<SMapped<i64, SeekFrom>>,
    )>;
    prop_oneof![
        static_map(any::<u64>(), SeekFrom::Start),
        static_map(any::<i64>(), SeekFrom::End),
        static_map(any::<i64>(), SeekFrom::Current)
    ]
);

arbitrary!(Error, SMapped<(ErrorKind, Option<String>), Self>;
    static_map(arbitrary(), |(k, os)|
        if let Some(s) = os { Error::new(k, s) } else { k.into() }
    )
);

#[cfg(test)]
mod test {

    no_panic_test!(
        buf_reader  => BufReader<Repeat>,
        buf_writer  => BufWriter<Sink>,
        line_writer => LineWriter<Sink>,
        chain       => Chain<Empty, BufReader<Repeat>>,
        cursor      => Cursor<Empty>,
        empty       => Empty,
        sink        => Sink,
        stderr      => Stderr,
        stdin       => Stdin,
        stdout      => Stdout,
        lines       => Lines<Empty>,
        repeat      => Repeat,
        split       => Split<Cursor<Vec<u8>>>,
        take        => Take<Repeat>,
        error_kind  => ErrorKind,
        seek_from   => SeekFrom,
        error       => Error
    );
}
