use cfg_expr::{Expression, Predicate, targets::get_builtin_target_by_triple};

fn main() {
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
            feature = "cool_thing",
        )"#,
    )
    .unwrap();

    // cfg_expr includes a list of every builtin target in rustc
    let x86_win = get_builtin_target_by_triple("i686-pc-windows-msvc").unwrap();
    let x86_pentium_win = get_builtin_target_by_triple("i586-pc-windows-msvc").unwrap();
    let uwp_win = get_builtin_target_by_triple("i686-uwp-windows-msvc").unwrap();
    let mac = get_builtin_target_by_triple("x86_64-apple-darwin").unwrap();

    let avail_targ_feats = ["fxsr", "sse", "sse2"];

    // This will satisfy all requirements
    assert!(specific.eval(|pred| {
        match pred {
            Predicate::Target(tp) => tp.matches(x86_win),
            Predicate::TargetFeature(feat) => avail_targ_feats.contains(feat),
            Predicate::Feature(feat) => *feat == "cool_thing",
            _ => false,
        }
    }));

    // This won't, it doesnt' have the cool_thing feature!
    assert!(!specific.eval(|pred| {
        match pred {
            Predicate::Target(tp) => tp.matches(x86_pentium_win),
            Predicate::TargetFeature(feat) => avail_targ_feats.contains(feat),
            _ => false,
        }
    }));

    // This will *not* satisfy the vendor predicate
    assert!(!specific.eval(|pred| {
        match pred {
            Predicate::Target(tp) => tp.matches(uwp_win),
            Predicate::TargetFeature(feat) => avail_targ_feats.contains(feat),
            _ => false,
        }
    }));

    // This will *not* satisfy the vendor, os, or env predicates
    assert!(!specific.eval(|pred| {
        match pred {
            Predicate::Target(tp) => tp.matches(mac),
            Predicate::TargetFeature(feat) => avail_targ_feats.contains(feat),
            _ => false,
        }
    }));
}
