use cfg_expr::{
    Expression, TargetPredicate,
    expr::{Predicate, TargetMatcher},
    targets::{ALL_BUILTINS as all, get_builtin_target_by_triple},
};

struct Target {
    builtin: &'static cfg_expr::targets::TargetInfo,
    #[cfg(feature = "targets")]
    lexicon: Option<target_lexicon::Triple>,
}

impl Target {
    fn make(s: &str) -> Self {
        Self {
            builtin: get_builtin_target_by_triple(s).unwrap(),
            #[cfg(feature = "targets")]
            lexicon: {
                // Hack to workaround the addition in 1.48.0 of this weird, non-conformant
                // target triple, until https://github.com/bytecodealliance/target-lexicon/issues/63 is
                // resolved in a satisfactory manner, not really concerned about
                // the presence of this triple in most normal cases
                use target_lexicon as tl;
                match s {
                    "avr-unknown-gnu-atmega328" => Some(tl::Triple {
                        architecture: tl::Architecture::Avr,
                        vendor: tl::Vendor::Unknown,
                        operating_system: tl::OperatingSystem::Unknown,
                        environment: tl::Environment::Unknown,
                        binary_format: tl::BinaryFormat::Unknown,
                    }),
                    triple => match triple.parse::<tl::Triple>() {
                        Ok(l) => Some(l),
                        Err(e) => {
                            // There are enough new weird architectures added in each version of
                            // Rust that it is difficult to keep target-lexicon aware of all of
                            // them. So try parsing this triple, but don't fail if it doesn't work.
                            eprintln!("failed to parse '{triple}': {e:?}");
                            None
                        }
                    },
                }
            },
        }
    }
}

macro_rules! tg_match {
    ($pred:expr, $target:expr) => {
        match $pred {
            Predicate::Target(tg) => {
                let tinfo = tg.matches($target.builtin);

                #[cfg(feature = "targets")]
                if !matches!(tg, TargetPredicate::HasAtomic(_))
                    && !matches!(tg, TargetPredicate::Panic(_))
                {
                    if let Some(l) = &$target.lexicon {
                        let linfo = tg.matches(l);
                        assert_eq!(
                            tinfo, linfo,
                            "{:#?} builtin didn't match lexicon {:#?} for predicate {tg:#?}",
                            $target.builtin, $target.lexicon,
                        );

                        return linfo;
                    }
                }

                tinfo
            }
            _ => panic!("not a target predicate"),
        }
    };

    ($pred:expr, $target:expr, $feats:expr) => {
        match $pred {
            Predicate::Target(tg) => {
                let tinfo = tg.matches($target.builtin);

                #[cfg(feature = "targets")]
                if !matches!(tg, TargetPredicate::HasAtomic(_))
                    && !matches!(tg, TargetPredicate::Panic(_))
                {
                    if let Some(l) = &$target.lexicon {
                        let linfo = tg.matches(l);
                        assert_eq!(
                            tinfo, linfo,
                            "{:#?} builtin didn't match lexicon {:#?} for predicate {tg:#?}",
                            $target.builtin, $target.lexicon,
                        );

                        return linfo;
                    }
                }

                tinfo
            }
            Predicate::TargetFeature(feat) => $feats.iter().find(|f| *f == feat).is_some(),
            _ => panic!("not a target predicate"),
        }
    };
}

#[test]
fn target_family() {
    let matches_any_family =
        Expression::parse("any(unix, target_family = \"windows\", target_family = \"wasm\")")
            .unwrap();
    let impossible = Expression::parse("all(windows, target_family = \"unix\")").unwrap();

    for target in all {
        let target = Target::make(target.triple.as_str());
        if target.builtin.families.is_empty() {
            assert!(!matches_any_family.eval(|pred| { tg_match!(pred, target) }));
        } else {
            assert!(matches_any_family.eval(|pred| { tg_match!(pred, target) }));
        }
        assert!(!impossible.eval(|pred| { tg_match!(pred, target) }));
    }
}

#[test]
fn tiny() {
    assert!(Expression::parse("all()").unwrap().eval(|_| false));
    assert!(!Expression::parse("any()").unwrap().eval(|_| true));
    assert!(!Expression::parse("not(all())").unwrap().eval(|_| false));
    assert!(Expression::parse("not(any())").unwrap().eval(|_| true));
    assert!(Expression::parse("all(not(blah))").unwrap().eval(|_| false));
    assert!(!Expression::parse("any(not(blah))").unwrap().eval(|_| true));
}

#[test]
fn very_specific() {
    let specific = Expression::parse(
        r#"all(
            target_os = "windows",
            target_arch = "x86",
            windows,
            target_env = "msvc",
            target_feature = "fxsr",
            target_feature = "sse",
            target_feature = "sse2",
            target_pointer_width = "32",
            target_endian = "little",
            not(target_vendor = "uwp"),
            target_has_atomic = "8",
            target_has_atomic = "16",
            target_has_atomic = "32",
            target_has_atomic = "64",
            not(target_has_atomic = "128"),
            target_has_atomic = "ptr",
            panic = "unwind",
            not(panic = "abort"),
        )"#,
    )
    .unwrap();

    for target in all {
        let t = Target::make(target.triple.as_str());
        assert_eq!(
            matches!(
                target.triple.as_str(),
                "i686-pc-windows-msvc" | "i586-pc-windows-msvc" | "i686-win7-windows-msvc"
            ),
            specific.eval(|pred| { tg_match!(pred, t, &["fxsr", "sse", "sse2"]) }),
            "expected true for i686-pc-windows-msvc, but got true for {}",
            target.triple,
        );
    }

    for target in all {
        let expr = format!(
            r#"cfg(
            all(
                target_arch = "{}",
                {}
                {}
                target_env = "{}"
            )
        )"#,
            target.arch.0,
            if let Some(v) = &target.vendor {
                format!(r#"target_vendor = "{}","#, v.0)
            } else {
                "".to_owned()
            },
            if let Some(v) = &target.os {
                format!(r#"target_os = "{}","#, v.0)
            } else {
                "".to_owned()
            },
            target.env.as_ref().map_or("", |e| e.as_str()),
        );

        let specific = Expression::parse(&expr).unwrap();

        let t = Target::make(target.triple.as_str());
        assert!(
            specific.eval(|pred| {
                if target.triple.as_str() == "mips64-openwrt-linux-musl" {
                    if let Predicate::Target(TargetPredicate::Vendor(vendor)) = pred {
                        // This is a special predicate that doesn't follow the usual rules for
                        // target-lexicon.
                        return t.builtin.matches(&TargetPredicate::Vendor(vendor.clone()));
                    }
                }
                tg_match!(pred, t)
            }),
            "failed expression '{}' for {:#?}",
            expr,
            t.builtin,
        );
    }
}

#[test]
fn complex() {
    let complex = Expression::parse(r#"cfg(all(unix, not(any(target_os="macos", target_os="android", target_os="emscripten"))))"#).unwrap();

    // Should match linuxes
    let linux_gnu = Target::make("x86_64-unknown-linux-gnu");
    let linux_musl = Target::make("x86_64-unknown-linux-musl");

    assert!(complex.eval(|pred| tg_match!(pred, linux_gnu)));
    assert!(complex.eval(|pred| tg_match!(pred, linux_musl)));

    // Should *not* match windows or mac or android
    let windows_msvc = Target::make("x86_64-pc-windows-msvc");
    let mac = Target::make("x86_64-apple-darwin");
    let android = Target::make("aarch64-linux-android");

    assert!(!complex.eval(|pred| tg_match!(pred, windows_msvc)));
    assert!(!complex.eval(|pred| tg_match!(pred, mac)));
    assert!(!complex.eval(|pred| tg_match!(pred, android)));

    let complex =
        Expression::parse(r#"all(not(target_os = "ios"), not(target_os = "android"))"#).unwrap();

    assert!(complex.eval(|pred| tg_match!(pred, linux_gnu)));
    assert!(complex.eval(|pred| tg_match!(pred, linux_musl)));
    assert!(complex.eval(|pred| tg_match!(pred, windows_msvc)));
    assert!(complex.eval(|pred| tg_match!(pred, mac)));
    assert!(!complex.eval(|pred| tg_match!(pred, android)));

    let complex = Expression::parse(r#"all(any(unix, target_arch="x86"), not(any(target_os="android", target_os="emscripten")))"#).unwrap();

    // Should match linuxes and mac
    assert!(complex.eval(|pred| tg_match!(pred, linux_gnu)));
    assert!(complex.eval(|pred| tg_match!(pred, linux_musl)));
    assert!(complex.eval(|pred| tg_match!(pred, mac)));

    // Should *not* match x86_64 windows or android
    assert!(!complex.eval(|pred| tg_match!(pred, windows_msvc)));
    assert!(!complex.eval(|pred| tg_match!(pred, android)));

    // Ensure that target_os = "none" matches against Os == None.
    let complex = Expression::parse(r#"all(target_os="none")"#).unwrap();
    let armebv7r_none_eabi = Target::make("armebv7r-none-eabi");
    assert!(!complex.eval(|pred| tg_match!(pred, linux_gnu)));
    assert!(complex.eval(|pred| tg_match!(pred, armebv7r_none_eabi)));
}

#[test]
fn unstable_target_abi() {
    let linux_gnu = Target::make("x86_64-unknown-linux-gnu");
    let linux_musl = Target::make("x86_64-unknown-linux-musl");
    let windows_msvc = Target::make("x86_64-pc-windows-msvc");
    let mac = Target::make("x86_64-apple-darwin");
    let android = Target::make("aarch64-linux-android");

    let target_with_abi_that_matches = cfg_expr::targets::TargetInfo {
        triple: cfg_expr::targets::Triple::new_const("aarch64-apple-darwin"),
        os: None,
        abi: Some(cfg_expr::targets::Abi::new_const("eabihf")),
        arch: cfg_expr::targets::Arch::aarch64,
        env: None,
        vendor: None,
        families: cfg_expr::targets::Families::unix,
        pointer_width: 64,
        endian: cfg_expr::targets::Endian::little,
        has_atomics: cfg_expr::targets::HasAtomics::atomic_8_16_32_64_128_ptr,
        panic: cfg_expr::targets::Panic::unwind,
    };

    let target_with_abi_that_doesnt_match = cfg_expr::targets::TargetInfo {
        abi: Some(cfg_expr::targets::Abi::new_const("ilp32")),
        ..target_with_abi_that_matches.clone()
    };

    let abi_pred =
        Expression::parse(r#"cfg(any(target_arch = "wasm32", target_abi = "eabihf"))"#).unwrap();

    // Should match a specified target_abi that's the same
    assert!(abi_pred.eval(|pred| {
        match pred {
            Predicate::Target(tp) => tp.matches(&target_with_abi_that_matches),
            _ => false,
        }
    }));

    // Should *not* match a specified target_abi that isn't the same
    assert!(!abi_pred.eval(|pred| {
        match pred {
            Predicate::Target(tp) => tp.matches(&target_with_abi_that_doesnt_match),
            _ => false,
        }
    }));

    // Should *not* match any builtins at this point because target_abi isn't stable
    assert!(!abi_pred.eval(|pred| tg_match!(pred, linux_gnu)));
    assert!(!abi_pred.eval(|pred| tg_match!(pred, linux_musl)));
    assert!(!abi_pred.eval(|pred| tg_match!(pred, mac)));
    assert!(!abi_pred.eval(|pred| tg_match!(pred, windows_msvc)));
    assert!(!abi_pred.eval(|pred| tg_match!(pred, android)));
}

#[test]
fn wasm_family() {
    let wasm = Expression::parse(r#"cfg(target_family = "wasm")"#).unwrap();

    let wasm32_unknown = Target::make("wasm32-unknown-unknown");
    let wasm32_emscripten = Target::make("wasm32-unknown-emscripten");
    let wasm32_wasip1 = Target::make("wasm32-wasip1");
    let wasm32_wasip1_threads = Target::make("wasm32-wasip1-threads");
    let wasm32_wasip2 = Target::make("wasm32-wasip2");
    let wasm32v1_none = Target::make("wasm32v1-none");
    let wasm64_unknown = Target::make("wasm64-unknown-unknown");

    // All of the above targets match.
    assert!(wasm.eval(|pred| tg_match!(pred, wasm32_unknown)));
    assert!(wasm.eval(|pred| tg_match!(pred, wasm32_emscripten)));
    assert!(wasm.eval(|pred| tg_match!(pred, wasm32_wasip1)));
    assert!(wasm.eval(|pred| tg_match!(pred, wasm32_wasip1_threads)));
    assert!(wasm.eval(|pred| tg_match!(pred, wasm32_wasip2)));
    assert!(wasm.eval(|pred| tg_match!(pred, wasm32v1_none)));
    assert!(wasm.eval(|pred| tg_match!(pred, wasm64_unknown)));
}

#[test]
fn features() {
    let enabled = ["good", "bad", "ugly"];

    let many_features = Expression::parse(
        r#"all(feature = "good", feature = "bad", feature = "ugly", not(feature = "nope"))"#,
    )
    .unwrap();

    assert!(many_features.eval(|pred| {
        match pred {
            Predicate::Feature(name) => enabled.contains(name),
            _ => false,
        }
    }));

    let feature_and_target_feature =
        Expression::parse(r#"all(feature = "make_fast", target_feature = "sse4.2")"#).unwrap();

    assert!(feature_and_target_feature.eval(|pred| {
        match pred {
            Predicate::Feature(name) => *name == "make_fast",
            Predicate::TargetFeature(feat) => *feat == "sse4.2",
            _ => false,
        }
    }));

    assert_eq!(
        feature_and_target_feature.eval(|pred| {
            match pred {
                Predicate::Feature(_) => Some(false),
                Predicate::TargetFeature(_) => None,
                _ => panic!("unexpected predicate"),
            }
        }),
        Some(false),
        "all() with Some(false) and None evaluates to Some(false)"
    );

    assert_eq!(
        feature_and_target_feature.eval(|pred| {
            match pred {
                Predicate::Feature(_) => Some(true),
                Predicate::TargetFeature(_) => None,
                _ => panic!("unexpected predicate"),
            }
        }),
        None,
        "all() with Some(true) and None evaluates to None"
    );
}
