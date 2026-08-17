mod all;
mod all_named;
mod bitflags_match;
mod bits;
mod clear;
mod complement;
mod contains;
mod difference;
mod empty;
mod eq;
mod extend;
mod flag_name;
mod flags;
mod fmt;
mod from_bits;
mod from_bits_retain;
mod from_bits_truncate;
mod from_name;
mod insert;
mod intersection;
mod intersects;
mod is_all;
mod is_empty;
mod iter;
mod known_bits;
mod parser;
mod remove;
mod symmetric_difference;
mod truncate;
mod union;
mod unknown;
mod unknown_bits;
mod iter_equal_names;

mod custom {
    pub const NAME: &'static str = "custom";
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestFlags: u8 {
        /// 1
        const A = 1;

        /// 1 << 1
        const B = 1 << 1;

        /// 1 << 2
        const C = 1 << 2;

        /// 1 | (1 << 1) | (1 << 2)
        const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestFlagsInvert: u8 {
        /// 1 | (1 << 1) | (1 << 2)
        const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();

        /// 1
        const A = 1;

        /// 1 << 1
        const B = 1 << 1;

        /// 1 << 2
        const C = 1 << 2;
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestZero: u8 {
        /// 0
        const ZERO = 0;
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestZeroOne: u8 {
        /// 0
        const ZERO = 0;

        /// 1
        const ONE = 1;
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestUnicode: u8 {
        /// 1
        const 一 = 1;

        /// 2
        const 二 = 1 << 1;
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestEmpty: u8 {}

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestOverlapping: u8 {
        /// 1 | (1 << 1)
        const AB = 1 | (1 << 1);

        /// (1 << 1) | (1 << 2)
        const BC = (1 << 1) | (1 << 2);
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestOverlappingFull: u8 {
        /// 1
        const A = 1;

        /// 1
        const B = 1;

        /// 1
        const C = 1;

        /// 2
        const D = 1 << 1;
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestExternal: u8 {
        /// 1
        const A = 1;

        /// 1 << 1
        const B = 1 << 1;

        /// 1 << 2
        const C = 1 << 2;

        /// 1 | (1 << 1) | (1 << 2)
        const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();

        /// External
        const _ = !0;
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestExternalFull: u8 {
        /// External
        const _ = !0;
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestRenamed: u8 {
        /// 1
        #[bitflags(flag_name = "a")]
        #[bitflags(flag_name = custom::NAME)]
        const A = 1;
        /// 1 << 1
        #[bitflags(flag_name = "custom")]
        const B = 1 << 1;
        /// 1 << 2
        #[bitflags(flag_name = "c")]
        const C = 1 << 2;
        /// 1 << 3
        #[bitflags(flag_name = "custom | e")]
        const D = 1 << 3;
    }
}
