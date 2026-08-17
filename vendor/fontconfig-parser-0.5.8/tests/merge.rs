use fontconfig_parser::*;

#[test]
fn merge_full() {
    let mut c = FontConfig::default();
    c.merge_config("./test-conf/fonts.conf").unwrap();

    // 00-5_reset-dirs.conf
    assert!(!c.dirs.contains(&DirData {
        path: "/before/reset".into(),
        salt: "".into(),
    }));
    assert!(c.dirs.contains(&DirData {
        path: "/after/reset".into(),
        salt: "".into(),
    }));

    // 00-6_reset-dirs-lex-order.conf
    assert!(c.dirs.contains(&DirData {
        path: "/after/after/reset".into(),
        salt: "".into(),
    }));

    // 00-nixos-cache.conf
    assert!(c.dirs.contains(&DirData {
        path: "/nix/store/i1yhgnfvaihqzs079lcx4zjnrdzcvaak-noto-fonts-2020-01-23".into(),
        salt: "".into(),
    }));

    // 69-unifont.yaml
    assert!(c.aliases.contains(&Alias {
        alias: "serif".into(),
        prefer: vec!["FreeSerif".into(), "Code2000".into(), "Code2001".into(),],
        accept: vec![],
        default: vec![],
    }));
}
