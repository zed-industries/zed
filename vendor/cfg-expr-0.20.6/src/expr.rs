pub mod lexer;
mod parser;

use smallvec::SmallVec;
use std::ops::Range;

/// A predicate function, used to combine 1 or more predicates
/// into a single value
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Func {
    /// `not()` with a configuration predicate. It is true if its predicate
    /// is false and false if its predicate is true.
    Not,
    /// `all()` with a comma separated list of configuration predicates. It
    /// is false if at least one predicate is false. If there are no predicates,
    /// it is true.
    ///
    /// The associated `usize` is the number of predicates inside the `all()`.
    All(usize),
    /// `any()` with a comma separated list of configuration predicates. It
    /// is true if at least one predicate is true. If there are no predicates,
    /// it is false.
    ///
    /// The associated `usize` is the number of predicates inside the `any()`.
    Any(usize),
}

use crate::targets as targ;

/// All predicates that pertains to a target, except for `target_feature`
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TargetPredicate {
    /// [target_abi](https://github.com/rust-lang/rust/issues/80970)
    Abi(targ::Abi),
    /// [target_arch](https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch)
    Arch(targ::Arch),
    /// [target_endian](https://doc.rust-lang.org/reference/conditional-compilation.html#target_endian)
    Endian(targ::Endian),
    /// [target_env](https://doc.rust-lang.org/reference/conditional-compilation.html#target_env)
    Env(targ::Env),
    /// [target_family](https://doc.rust-lang.org/reference/conditional-compilation.html#target_family)
    /// This also applies to the bare [`unix` and `windows`](https://doc.rust-lang.org/reference/conditional-compilation.html#unix-and-windows)
    /// predicates.
    Family(targ::Family),
    /// [target_has_atomic](https://doc.rust-lang.org/reference/conditional-compilation.html#target_has_atomic).
    HasAtomic(targ::HasAtomic),
    /// [target_os](https://doc.rust-lang.org/reference/conditional-compilation.html#target_os)
    Os(targ::Os),
    /// [panic](https://doc.rust-lang.org/reference/conditional-compilation.html#panic)
    Panic(targ::Panic),
    /// [target_pointer_width](https://doc.rust-lang.org/reference/conditional-compilation.html#target_pointer_width)
    PointerWidth(u8),
    /// [target_vendor](https://doc.rust-lang.org/reference/conditional-compilation.html#target_vendor)
    Vendor(targ::Vendor),
}

pub trait TargetMatcher {
    fn matches(&self, tp: &TargetPredicate) -> bool;
}

impl TargetMatcher for targ::TargetInfo {
    fn matches(&self, tp: &TargetPredicate) -> bool {
        use TargetPredicate::{
            Abi, Arch, Endian, Env, Family, HasAtomic, Os, Panic, PointerWidth, Vendor,
        };

        match tp {
            // The ABI is allowed to be an empty string
            Abi(abi) => match &self.abi {
                Some(a) => abi == a,
                None => abi.0.is_empty(),
            },
            Arch(a) => a == &self.arch,
            Endian(end) => *end == self.endian,
            // The environment is allowed to be an empty string
            Env(env) => match &self.env {
                Some(e) => env == e,
                None => env.0.is_empty(),
            },
            Family(fam) => self.families.contains(fam),
            HasAtomic(has_atomic) => self.has_atomics.contains(*has_atomic),
            Os(os) => match &self.os {
                Some(self_os) => os == self_os,
                // os = "none" means it should be matched against None. Note that this is different
                // from "env" above.
                None => os.as_str() == "none",
            },
            PointerWidth(w) => *w == self.pointer_width,
            Vendor(ven) => match &self.vendor {
                Some(v) => ven == v,
                None => ven == &targ::Vendor::unknown,
            },
            Panic(panic) => &self.panic == panic,
        }
    }
}

#[cfg(feature = "targets")]
impl TargetMatcher for target_lexicon::Triple {
    #[allow(clippy::cognitive_complexity)]
    #[allow(clippy::match_same_arms)]
    fn matches(&self, tp: &TargetPredicate) -> bool {
        use TargetPredicate::{
            Abi, Arch, Endian, Env, Family, HasAtomic, Os, Panic, PointerWidth, Vendor,
        };
        use target_lexicon::{
            self as tl, Architecture as arch, ArmArchitecture, Endianness as endian,
            Environment as env, Mips32Architecture as mips32, Mips64Architecture as mips64,
            OperatingSystem as os,
        };

        const NUTTX: tl::Vendor = tl::Vendor::Custom(tl::CustomVendor::Static("nuttx"));
        const RTEMS: tl::Vendor = tl::Vendor::Custom(tl::CustomVendor::Static("rtems"));
        const WALI: tl::Vendor = tl::Vendor::Custom(tl::CustomVendor::Static("wali"));
        const WASIP3: tl::Vendor = tl::Vendor::Custom(tl::CustomVendor::Static("wasip3"));

        match tp {
            Abi(_) => {
                // `target_abi` is unstable. Assume false for this.
                false
            }
            Arch(arch) => {
                if arch == &targ::Arch::x86 {
                    matches!(self.architecture, arch::X86_32(_))
                } else if arch == &targ::Arch::wasm32 {
                    self.architecture == arch::Wasm32 || self.architecture == arch::Asmjs
                } else if arch == &targ::Arch::arm {
                    matches!(self.architecture, arch::Arm(_))
                } else if arch == &targ::Arch::bpf {
                    self.architecture == arch::Bpfeb || self.architecture == arch::Bpfel
                } else if arch == &targ::Arch::x86_64 {
                    self.architecture == arch::X86_64 || self.architecture == arch::X86_64h
                } else if arch == &targ::Arch::mips32r6 {
                    matches!(
                        self.architecture,
                        arch::Mips32(mips32::Mipsisa32r6 | mips32::Mipsisa32r6el)
                    )
                } else if arch == &targ::Arch::mips64r6 {
                    matches!(
                        self.architecture,
                        arch::Mips64(mips64::Mipsisa64r6 | mips64::Mipsisa64r6el)
                    )
                } else if arch == &targ::Arch::amdgpu {
                    self.architecture == arch::AmdGcn
                } else {
                    match arch.0.parse::<arch>() {
                        Ok(a) => match (self.architecture, a) {
                            (arch::Aarch64(_), arch::Aarch64(_))
                            | (arch::Mips32(_), arch::Mips32(_))
                            | (arch::Mips64(_), arch::Mips64(_))
                            | (arch::Powerpc64le, arch::Powerpc64)
                            | (arch::Riscv32(_), arch::Riscv32(_))
                            | (arch::Riscv64(_), arch::Riscv64(_))
                            | (arch::Sparcv9, arch::Sparc64) => true,
                            (a, b) => a == b,
                        },
                        Err(_) => false,
                    }
                }
            }
            Endian(end) => match self.architecture.endianness() {
                Ok(endian) => matches!(
                    (end, endian),
                    (crate::targets::Endian::little, endian::Little)
                        | (crate::targets::Endian::big, endian::Big)
                ),

                Err(_) => false,
            },
            Env(env) => {
                // The environment is implied by some operating systems
                match self.operating_system {
                    os::Redox => env == &targ::Env::relibc,
                    os::VxWorks => env == &targ::Env::gnu,
                    os::Freebsd => env.0.is_empty(),
                    os::Netbsd => match self.architecture {
                        arch::Arm(ArmArchitecture::Armv6 | ArmArchitecture::Armv7) => {
                            env.0.is_empty()
                        }
                        _ => env.0.is_empty(),
                    },
                    os::None_ | os::Cloudabi | os::Hermit => match self.environment {
                        env::LinuxKernel => env == &targ::Env::gnu,
                        _ => env.0.is_empty(),
                    },
                    os::IOS(_) | os::TvOS(_) => match self.environment {
                        env::LinuxKernel => env == &targ::Env::gnu,
                        env::Macabi => env == &targ::Env::macabi,
                        env::Sim => env == &targ::Env::sim,
                        env::Unknown => env.0.is_empty() || env == &targ::Env::sim,
                        _ => env.0.is_empty(),
                    },
                    os::WasiP1 => env == &targ::Env::p1,
                    os::WasiP2 => env == &targ::Env::p2,
                    os::Wasi => env.0.is_empty() || env == &targ::Env::p1,
                    _ => {
                        if env.0.is_empty() {
                            matches!(
                                self.environment,
                                env::Unknown
                                    | env::Android
                                    | env::Softfloat
                                    | env::Androideabi
                                    | env::Eabi
                                    | env::Eabihf
                                    | env::Sim
                                    | env::None
                            )
                        } else if env == &targ::Env::p3 {
                            self.vendor == WASIP3
                        } else {
                            match env.0.parse::<env>() {
                                Ok(e) => {
                                    // Rustc shortens multiple "gnu*" environments to just "gnu"
                                    if env == &targ::Env::gnu {
                                        match self.environment {
                                            env::Gnu
                                            | env::Gnuabi64
                                            | env::Gnueabi
                                            | env::Gnuspe
                                            | env::Gnux32
                                            | env::GnuIlp32
                                            | env::Gnueabihf
                                            | env::GnuLlvm => true,
                                            // Rust 1.49.0 changed all android targets to have the
                                            // gnu environment
                                            env::Android | env::Androideabi
                                                if self.operating_system == os::Linux =>
                                            {
                                                true
                                            }
                                            env::Kernel => self.operating_system == os::Linux,
                                            _ => self.architecture == arch::Avr,
                                        }
                                    } else if env == &targ::Env::musl {
                                        matches!(
                                            self.environment,
                                            env::Musl
                                                | env::Musleabi
                                                | env::Musleabihf
                                                | env::Muslabi64
                                        )
                                    } else if env == &targ::Env::uclibc {
                                        matches!(
                                            self.environment,
                                            env::Uclibc | env::Uclibceabi | env::Uclibceabihf
                                        )
                                    } else if env == &targ::Env::newlib {
                                        matches!(self.operating_system, os::Horizon | os::Espidf)
                                            || self.vendor == RTEMS
                                    } else {
                                        self.environment == e
                                    }
                                }
                                Err(_) => false,
                            }
                        }
                    }
                }
            }
            Family(fam) => {
                match self.operating_system {
                    os::AmdHsa
                    | os::Bitrig
                    | os::Cloudabi
                    | os::Cuda
                    | os::Hermit
                    | os::Nebulet
                    | os::None_
                    | os::Uefi => false,
                    os::Aix
                    | os::Darwin(_)
                    | os::Dragonfly
                    | os::Espidf
                    | os::Freebsd
                    | os::Fuchsia
                    | os::Haiku
                    | os::Hurd
                    | os::Illumos
                    | os::IOS(_)
                    | os::L4re
                    | os::MacOSX { .. }
                    | os::Horizon
                    | os::Netbsd
                    | os::Openbsd
                    | os::Redox
                    | os::Solaris
                    | os::TvOS(_)
                    | os::VisionOS(_)
                    | os::VxWorks
                    | os::WatchOS(_) => fam == &crate::targets::Family::unix,
                    os::Emscripten => {
                        match self.architecture {
                            // asmjs, wasm32 and wasm64 are part of both the wasm and unix families
                            arch::Asmjs | arch::Wasm32 => {
                                fam == &crate::targets::Family::wasm
                                    || fam == &crate::targets::Family::unix
                            }
                            _ => false,
                        }
                    }
                    os::Unknown if self.vendor == NUTTX || self.vendor == RTEMS => {
                        fam == &crate::targets::Family::unix
                    }
                    os::Unknown => {
                        // asmjs, wasm32 and wasm64 are part of the wasm family.
                        match self.architecture {
                            arch::Asmjs | arch::Wasm32 | arch::Wasm64 => {
                                fam == &crate::targets::Family::wasm
                            }
                            _ => false,
                        }
                    }
                    os::Linux if self.vendor == WALI => {
                        fam == &crate::targets::Family::wasm || fam == &crate::targets::Family::unix
                    }
                    os::Linux => {
                        // The 'kernel' environment is treated specially as not-unix
                        if self.environment != env::Kernel {
                            fam == &crate::targets::Family::unix
                        } else {
                            false
                        }
                    }
                    os::Wasi | os::WasiP1 | os::WasiP2 => fam == &crate::targets::Family::wasm,
                    os::Windows => fam == &crate::targets::Family::windows,
                    os::Cygwin => fam == &crate::targets::Family::unix,
                    // I really dislike non-exhaustive :(
                    _ => false,
                }
            }
            HasAtomic(_) => {
                // atomic support depends on both the architecture and the OS. Assume false for
                // this.
                false
            }
            Os(os) => {
                if os == &targ::Os::wasi
                    && (matches!(self.operating_system, os::WasiP1 | os::WasiP2)
                        || self.vendor == WASIP3)
                    || (os == &targ::Os::nuttx && self.vendor == NUTTX)
                    || (os == &targ::Os::rtems && self.vendor == RTEMS)
                {
                    return true;
                }

                match os.0.parse::<os>() {
                    Ok(o) => match self.environment {
                        env::HermitKernel => os == &targ::Os::hermit,
                        _ => self.operating_system == o,
                    },
                    Err(_) => {
                        // Handle special case for darwin/macos, where the triple is
                        // "darwin", but rustc identifies the OS as "macos"
                        if os == &targ::Os::macos && matches!(self.operating_system, os::Darwin(_))
                        {
                            true
                        } else {
                            // For android, the os is still linux, but the environment is android
                            os == &targ::Os::android
                                && self.operating_system == os::Linux
                                && (self.environment == env::Android
                                    || self.environment == env::Androideabi)
                        }
                    }
                }
            }
            Panic(_) => {
                // panic support depends on the OS. Assume false for this.
                false
            }
            Vendor(ven) => match ven.0.parse::<target_lexicon::Vendor>() {
                Ok(v) => {
                    if self.vendor == v
                        || ((self.vendor == NUTTX
                            || self.vendor == RTEMS
                            || self.vendor == WALI
                            || self.vendor == WASIP3)
                            && ven == &targ::Vendor::unknown)
                    {
                        true
                    } else if let tl::Vendor::Custom(custom) = &self.vendor {
                        matches!(custom.as_str(), "esp" | "esp32" | "esp32s2" | "esp32s3")
                            && (v == tl::Vendor::Espressif || v == tl::Vendor::Unknown)
                    } else {
                        false
                    }
                }
                Err(_) => false,
            },
            PointerWidth(pw) => {
                // The gnux32 environment is a special case, where it has an
                // x86_64 architecture, but a 32-bit pointer width
                if !matches!(self.environment, env::Gnux32 | env::GnuIlp32) {
                    *pw == match self.pointer_width() {
                        Ok(pw) => pw.bits(),
                        Err(_) => return false,
                    }
                } else {
                    *pw == 32
                }
            }
        }
    }
}

impl TargetPredicate {
    /// Returns true of the predicate matches the specified target
    ///
    /// Note that when matching against a [`target_lexicon::Triple`], the
    /// `has_target_atomic` and `panic` predicates will _always_ return `false`.
    ///
    /// ```
    /// use cfg_expr::{targets::*, expr::TargetPredicate as tp};
    /// let win = get_builtin_target_by_triple("x86_64-pc-windows-msvc").unwrap();
    ///
    /// assert!(
    ///     tp::Arch(Arch::x86_64).matches(win) &&
    ///     tp::Endian(Endian::little).matches(win) &&
    ///     tp::Env(Env::msvc).matches(win) &&
    ///     tp::Family(Family::windows).matches(win) &&
    ///     tp::Os(Os::windows).matches(win) &&
    ///     tp::PointerWidth(64).matches(win) &&
    ///     tp::Vendor(Vendor::pc).matches(win)
    /// );
    /// ```
    pub fn matches<T>(&self, target: &T) -> bool
    where
        T: TargetMatcher,
    {
        target.matches(self)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Which {
    Abi,
    Arch,
    Endian(targ::Endian),
    Env,
    Family,
    Os,
    HasAtomic(targ::HasAtomic),
    Panic,
    PointerWidth(u8),
    Vendor,
}

#[derive(Clone, Debug)]
pub(crate) struct InnerTarget {
    which: Which,
    span: Option<Range<usize>>,
}

/// A single predicate in a `cfg()` expression
#[derive(Debug, PartialEq, Eq)]
pub enum Predicate<'a> {
    /// A target predicate, with the `target_` prefix
    Target(TargetPredicate),
    /// Whether rustc's test harness is [enabled](https://doc.rust-lang.org/reference/conditional-compilation.html#test)
    Test,
    /// [Enabled](https://doc.rust-lang.org/reference/conditional-compilation.html#debug_assertions)
    /// when compiling without optimizations.
    DebugAssertions,
    /// [Enabled](https://doc.rust-lang.org/reference/conditional-compilation.html#proc_macro) for
    /// crates of the `proc_macro` type.
    ProcMacro,
    /// A [`feature = "<name>"`](https://doc.rust-lang.org/nightly/cargo/reference/features.html)
    Feature(&'a str),
    /// [target_feature](https://doc.rust-lang.org/reference/conditional-compilation.html#target_feature)
    TargetFeature(&'a str),
    /// A generic bare predicate key that doesn't match one of the known options, eg `cfg(bare)`
    Flag(&'a str),
    /// A generic key = "value" predicate that doesn't match one of the known options, eg `cfg(foo = "bar")`
    KeyValue { key: &'a str, val: &'a str },
}

#[derive(Clone, Debug)]
pub(crate) enum InnerPredicate {
    Target(InnerTarget),
    Test,
    DebugAssertions,
    ProcMacro,
    Feature(Range<usize>),
    TargetFeature(Range<usize>),
    Other {
        identifier: Range<usize>,
        value: Option<Range<usize>>,
    },
}

impl InnerPredicate {
    fn to_pred<'a>(&self, s: &'a str) -> Predicate<'a> {
        use InnerPredicate as IP;
        use Predicate::{
            DebugAssertions, Feature, Flag, KeyValue, ProcMacro, Target, TargetFeature, Test,
        };

        match self {
            IP::Target(it) => match &it.which {
                Which::Abi => Target(TargetPredicate::Abi(targ::Abi::new(
                    s[it.span.clone().unwrap()].to_owned(),
                ))),
                Which::Arch => Target(TargetPredicate::Arch(targ::Arch::new(
                    s[it.span.clone().unwrap()].to_owned(),
                ))),
                Which::Os => Target(TargetPredicate::Os(targ::Os::new(
                    s[it.span.clone().unwrap()].to_owned(),
                ))),
                Which::Vendor => Target(TargetPredicate::Vendor(targ::Vendor::new(
                    s[it.span.clone().unwrap()].to_owned(),
                ))),
                Which::Env => Target(TargetPredicate::Env(targ::Env::new(
                    s[it.span.clone().unwrap()].to_owned(),
                ))),
                Which::Family => Target(TargetPredicate::Family(targ::Family::new(
                    s[it.span.clone().unwrap()].to_owned(),
                ))),
                Which::Endian(end) => Target(TargetPredicate::Endian(*end)),
                Which::HasAtomic(has_atomic) => Target(TargetPredicate::HasAtomic(*has_atomic)),
                Which::Panic => Target(TargetPredicate::Panic(targ::Panic::new(
                    s[it.span.clone().unwrap()].to_owned(),
                ))),
                Which::PointerWidth(pw) => Target(TargetPredicate::PointerWidth(*pw)),
            },
            IP::Test => Test,
            IP::DebugAssertions => DebugAssertions,
            IP::ProcMacro => ProcMacro,
            IP::Feature(rng) => Feature(&s[rng.clone()]),
            IP::TargetFeature(rng) => TargetFeature(&s[rng.clone()]),
            IP::Other { identifier, value } => match value {
                Some(vs) => KeyValue {
                    key: &s[identifier.clone()],
                    val: &s[vs.clone()],
                },
                None => Flag(&s[identifier.clone()]),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum ExprNode {
    Fn(Func),
    Predicate(InnerPredicate),
}

/// A parsed `cfg()` expression that can evaluated
#[derive(Clone, Debug)]
pub struct Expression {
    pub(crate) expr: SmallVec<[ExprNode; 5]>,
    // We keep the original string around for providing the arbitrary
    // strings that can make up an expression
    pub(crate) original: String,
}

impl Expression {
    /// An iterator over each predicate in the expression
    pub fn predicates(&self) -> impl Iterator<Item = Predicate<'_>> {
        self.expr.iter().filter_map(move |item| match item {
            ExprNode::Predicate(pred) => {
                let pred = pred.clone().to_pred(&self.original);
                Some(pred)
            }
            ExprNode::Fn(_) => None,
        })
    }

    /// Evaluates the expression, using the provided closure to determine the value of
    /// each predicate, which are then combined into a final result depending on the
    /// functions `not()`, `all()`, or `any()` in the expression.
    ///
    /// `eval_predicate` typically returns `bool`, but may return any type that implements
    /// the `Logic` trait.
    ///
    /// ## Examples
    ///
    /// ```
    /// use cfg_expr::{targets::*, Expression, Predicate};
    ///
    /// let linux_musl = get_builtin_target_by_triple("x86_64-unknown-linux-musl").unwrap();
    ///
    /// let expr = Expression::parse(r#"all(not(windows), target_env = "musl", any(target_arch = "x86", target_arch = "x86_64"))"#).unwrap();
    ///
    /// assert!(expr.eval(|pred| {
    ///     match pred {
    ///         Predicate::Target(tp) => tp.matches(linux_musl),
    ///         _ => false,
    ///     }
    /// }));
    /// ```
    ///
    /// Returning `Option<bool>`, where `None` indicates the result is unknown:
    ///
    /// ```
    /// use cfg_expr::{targets::*, Expression, Predicate};
    ///
    /// let expr = Expression::parse(r#"any(target_feature = "sse2", target_env = "musl")"#).unwrap();
    ///
    /// let linux_gnu = get_builtin_target_by_triple("x86_64-unknown-linux-gnu").unwrap();
    /// let linux_musl = get_builtin_target_by_triple("x86_64-unknown-linux-musl").unwrap();
    ///
    /// fn eval(expr: &Expression, target: &TargetInfo) -> Option<bool> {
    ///     expr.eval(|pred| {
    ///         match pred {
    ///             Predicate::Target(tp) => Some(tp.matches(target)),
    ///             Predicate::TargetFeature(_) => None,
    ///             _ => panic!("unexpected predicate"),
    ///         }
    ///     })
    /// }
    ///
    /// // Whether the target feature is present is unknown, so the whole expression evaluates to
    /// // None (unknown).
    /// assert_eq!(eval(&expr, linux_gnu), None);
    ///
    /// // Whether the target feature is present is irrelevant for musl, since the any() always
    /// // evaluates to true.
    /// assert_eq!(eval(&expr, linux_musl), Some(true));
    /// ```
    pub fn eval<EP, T>(&self, mut eval_predicate: EP) -> T
    where
        EP: FnMut(&Predicate<'_>) -> T,
        T: Logic + std::fmt::Debug,
    {
        let mut result_stack = SmallVec::<[T; 8]>::new();

        // We store the expression as postfix, so just evaluate each component
        // requirement in the order it comes, and then combining the previous
        // results according to each operator as it comes
        for node in self.expr.iter() {
            match node {
                ExprNode::Predicate(pred) => {
                    let pred = pred.to_pred(&self.original);

                    result_stack.push(eval_predicate(&pred));
                }
                ExprNode::Fn(Func::All(count)) => {
                    // all() with a comma separated list of configuration predicates.
                    let mut result = T::top();

                    for _ in 0..*count {
                        let r = result_stack.pop().unwrap();
                        result = result.and(r);
                    }

                    result_stack.push(result);
                }
                ExprNode::Fn(Func::Any(count)) => {
                    // any() with a comma separated list of configuration predicates.
                    let mut result = T::bottom();

                    for _ in 0..*count {
                        let r = result_stack.pop().unwrap();
                        result = result.or(r);
                    }

                    result_stack.push(result);
                }
                ExprNode::Fn(Func::Not) => {
                    // not() with a configuration predicate.
                    // It is true if its predicate is false
                    // and false if its predicate is true.
                    let r = result_stack.pop().unwrap();
                    result_stack.push(r.not());
                }
            }
        }

        result_stack.pop().unwrap()
    }

    /// The original string which has been parsed to produce this [`Expression`].
    ///
    /// ```
    /// use cfg_expr::Expression;
    ///
    /// assert_eq!(
    ///     Expression::parse("any()").unwrap().original(),
    ///     "any()"
    /// );
    /// ```
    #[inline]
    pub fn original(&self) -> &str {
        &self.original
    }
}

/// [`PartialEq`] will do a **syntactical** comparison, so will just check if both
/// expressions have been parsed from the same string, **not** if they are semantically
/// equivalent.
///
/// ```
/// use cfg_expr::Expression;
///
/// assert_eq!(
///     Expression::parse("any()").unwrap(),
///     Expression::parse("any()").unwrap()
/// );
/// assert_ne!(
///     Expression::parse("any()").unwrap(),
///     Expression::parse("unix").unwrap()
/// );
/// ```
impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.original.eq(&other.original)
    }
}

impl std::str::FromStr for Expression {
    type Err = crate::error::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Expression::parse(s)
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.original)
    }
}

/// A propositional logic used to evaluate `Expression` instances.
///
/// An `Expression` consists of some predicates and the `any`, `all` and `not` operators. An
/// implementation of `Logic` defines how the `any`, `all` and `not` operators should be evaluated.
pub trait Logic {
    /// The result of an `all` operation with no operands, akin to Boolean `true`.
    fn top() -> Self;

    /// The result of an `any` operation with no operands, akin to Boolean `false`.
    fn bottom() -> Self;

    /// `AND`, which corresponds to the `all` operator.
    fn and(self, other: Self) -> Self;

    /// `OR`, which corresponds to the `any` operator.
    fn or(self, other: Self) -> Self;

    /// `NOT`, which corresponds to the `not` operator.
    fn not(self) -> Self;
}

/// A boolean logic.
impl Logic for bool {
    #[inline]
    fn top() -> Self {
        true
    }

    #[inline]
    fn bottom() -> Self {
        false
    }

    #[inline]
    fn and(self, other: Self) -> Self {
        self && other
    }

    #[inline]
    fn or(self, other: Self) -> Self {
        self || other
    }

    #[inline]
    fn not(self) -> Self {
        !self
    }
}

/// A three-valued logic -- `None` stands for the value being unknown.
///
/// The truth tables for this logic are described on
/// [Wikipedia](https://en.wikipedia.org/wiki/Three-valued_logic#Kleene_and_Priest_logics).
impl Logic for Option<bool> {
    #[inline]
    fn top() -> Self {
        Some(true)
    }

    #[inline]
    fn bottom() -> Self {
        Some(false)
    }

    #[inline]
    fn and(self, other: Self) -> Self {
        match (self, other) {
            // If either is false, the expression is false.
            (Some(false), _) | (_, Some(false)) => Some(false),
            // If both are true, the expression is true.
            (Some(true), Some(true)) => Some(true),
            // One or both are unknown -- the result is unknown.
            _ => None,
        }
    }

    #[inline]
    fn or(self, other: Self) -> Self {
        match (self, other) {
            // If either is true, the expression is true.
            (Some(true), _) | (_, Some(true)) => Some(true),
            // If both are false, the expression is false.
            (Some(false), Some(false)) => Some(false),
            // One or both are unknown -- the result is unknown.
            _ => None,
        }
    }

    #[inline]
    fn not(self) -> Self {
        self.map(|v| !v)
    }
}
