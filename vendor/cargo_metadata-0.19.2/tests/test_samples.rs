extern crate cargo_metadata;
extern crate semver;
#[macro_use]
extern crate serde_json;

use camino::Utf8PathBuf;
use cargo_metadata::{
    ArtifactDebuginfo, CargoOpt, DependencyKind, Edition, Message, Metadata, MetadataCommand,
};

/// Output from oldest version ever supported (1.24).
///
/// This intentionally has as many null fields as possible.
/// 1.8 is when metadata was introduced.
/// Older versions not supported because the following are required:
/// - `workspace_members` added in 1.13
/// - `target_directory` added in 1.19
/// - `workspace_root` added in 1.24
const JSON_OLD_MINIMAL: &str = r#"
{
  "packages": [
    {
      "name": "foo",
      "version": "0.1.0",
      "id": "foo 0.1.0 (path+file:///foo)",
      "license": null,
      "license_file": null,
      "description": null,
      "source": null,
      "dependencies": [
        {
          "name": "somedep",
          "source": null,
          "req": "^1.0",
          "kind": null,
          "optional": false,
          "uses_default_features": true,
          "features": [],
          "target": null
        }
      ],
      "targets": [
        {
          "kind": [
            "bin"
          ],
          "crate_types": [
            "bin"
          ],
          "name": "foo",
          "src_path": "/foo/src/main.rs"
        }
      ],
      "features": {},
      "manifest_path": "/foo/Cargo.toml"
    }
  ],
  "workspace_members": [
    "foo 0.1.0 (path+file:///foo)"
  ],
  "resolve": null,
  "target_directory": "/foo/target",
  "version": 1,
  "workspace_root": "/foo"
}
"#;

#[test]
fn old_minimal() {
    let meta: Metadata = serde_json::from_str(JSON_OLD_MINIMAL).unwrap();
    assert_eq!(meta.packages.len(), 1);
    let pkg = &meta.packages[0];
    assert_eq!(pkg.name, "foo");
    assert_eq!(pkg.version, semver::Version::parse("0.1.0").unwrap());
    assert_eq!(pkg.authors.len(), 0);
    assert_eq!(pkg.id.to_string(), "foo 0.1.0 (path+file:///foo)");
    assert_eq!(pkg.description, None);
    assert_eq!(pkg.license, None);
    assert_eq!(pkg.license_file, None);
    assert_eq!(pkg.default_run, None);
    assert_eq!(pkg.rust_version, None);
    assert_eq!(pkg.dependencies.len(), 1);
    let dep = &pkg.dependencies[0];
    assert_eq!(dep.name, "somedep");
    assert_eq!(dep.source, None);
    assert_eq!(dep.req, semver::VersionReq::parse("^1.0").unwrap());
    assert_eq!(dep.kind, DependencyKind::Normal);
    assert!(!dep.optional);
    assert!(dep.uses_default_features);
    assert_eq!(dep.features.len(), 0);
    assert!(dep.target.is_none());
    assert_eq!(dep.rename, None);
    assert_eq!(dep.registry, None);
    assert_eq!(pkg.targets.len(), 1);
    let target = &pkg.targets[0];
    assert_eq!(target.name, "foo");
    assert_eq!(target.kind, vec!["bin".into()]);
    assert_eq!(target.crate_types, vec!["bin".into()]);
    assert_eq!(target.required_features.len(), 0);
    assert_eq!(target.src_path, "/foo/src/main.rs");
    assert_eq!(target.edition, Edition::E2015);
    assert!(target.doctest);
    assert!(target.test);
    assert!(target.doc);
    assert_eq!(pkg.features.len(), 0);
    assert_eq!(pkg.manifest_path, "/foo/Cargo.toml");
    assert_eq!(pkg.categories.len(), 0);
    assert_eq!(pkg.keywords.len(), 0);
    assert_eq!(pkg.readme, None);
    assert_eq!(pkg.repository, None);
    assert_eq!(pkg.homepage, None);
    assert_eq!(pkg.documentation, None);
    assert_eq!(pkg.edition, Edition::E2015);
    assert_eq!(pkg.metadata, serde_json::Value::Null);
    assert_eq!(pkg.links, None);
    assert_eq!(pkg.publish, None);
    assert_eq!(meta.workspace_members.len(), 1);
    assert_eq!(
        meta.workspace_members[0].to_string(),
        "foo 0.1.0 (path+file:///foo)"
    );
    assert!(meta.resolve.is_none());
    assert_eq!(meta.workspace_root, "/foo");
    assert_eq!(meta.workspace_metadata, serde_json::Value::Null);
    assert_eq!(meta.target_directory, "/foo/target");

    assert!(!meta.workspace_default_members.is_available());
    assert!(meta.workspace_default_members.is_missing());

    let serialized = serde_json::to_value(meta).unwrap();
    assert!(!serialized
        .as_object()
        .unwrap()
        .contains_key("workspace_default_members"));
}

macro_rules! sorted {
    ($e:expr) => {{
        let mut v = $e.clone();
        v.sort();
        v
    }};
}

fn cargo_version() -> semver::Version {
    let output = std::process::Command::new("cargo")
        .arg("-V")
        .output()
        .expect("Failed to exec cargo.");
    let out = std::str::from_utf8(&output.stdout)
        .expect("invalid utf8")
        .trim();
    let split: Vec<&str> = out.split_whitespace().collect();
    assert!(split.len() >= 2, "cargo -V output is unexpected: {}", out);
    let mut ver = semver::Version::parse(split[1]).expect("cargo -V semver could not be parsed");
    // Don't care about metadata, it is awkward to compare.
    ver.pre = semver::Prerelease::EMPTY;
    ver.build = semver::BuildMetadata::EMPTY;
    ver
}

#[derive(serde::Deserialize, PartialEq, Eq, Debug)]
struct WorkspaceMetadata {
    testobject: TestObject,
}

#[derive(serde::Deserialize, PartialEq, Eq, Debug)]
struct TestObject {
    myvalue: String,
}

#[test]
fn all_the_fields() {
    // All the fields currently generated as of 1.60. This tries to exercise as
    // much as possible.
    let ver = cargo_version();
    let minimum = semver::Version::parse("1.56.0").unwrap();
    if ver < minimum {
        // edition added in 1.30
        // rename added in 1.31
        // links added in 1.33
        // doctest added in 1.37
        // publish added in 1.39
        // dep_kinds added in 1.41
        // test added in 1.47
        // homepage added in 1.49
        // documentation added in 1.49
        // doc added in 1.50
        // path added in 1.51
        // default_run added in 1.55
        // rust_version added in 1.58
        // workspace_default_members added in 1.71
        eprintln!("Skipping all_the_fields test, cargo {} is too old.", ver);
        return;
    }
    let meta = MetadataCommand::new()
        .manifest_path("tests/all/Cargo.toml")
        .exec()
        .unwrap();
    assert_eq!(meta.workspace_root.file_name().unwrap(), "all");
    assert_eq!(
        serde_json::from_value::<WorkspaceMetadata>(meta.workspace_metadata.clone()).unwrap(),
        WorkspaceMetadata {
            testobject: TestObject {
                myvalue: "abc".to_string()
            }
        }
    );
    assert_eq!(meta.workspace_members.len(), 1);
    assert!(meta.workspace_members[0].to_string().contains("all"));
    if ver >= semver::Version::parse("1.71.0").unwrap() {
        assert_eq!(&*meta.workspace_default_members, &meta.workspace_members);
    }

    assert_eq!(meta.packages.len(), 9);
    let all = meta.packages.iter().find(|p| p.name == "all").unwrap();
    assert_eq!(all.version, semver::Version::parse("0.1.0").unwrap());
    assert_eq!(all.authors, vec!["Jane Doe <user@example.com>"]);
    assert!(all.id.to_string().contains("all"));
    assert_eq!(all.description, Some("Package description.".to_string()));
    assert_eq!(all.license, Some("MIT/Apache-2.0".to_string()));
    assert_eq!(all.license_file, Some(Utf8PathBuf::from("LICENSE")));
    assert!(all.license_file().unwrap().ends_with("tests/all/LICENSE"));
    assert_eq!(all.publish, Some(vec![]));
    assert_eq!(all.links, Some("foo".to_string()));
    assert_eq!(all.default_run, Some("otherbin".to_string()));
    if ver >= semver::Version::parse("1.58.0").unwrap() {
        assert_eq!(
            all.rust_version,
            Some(semver::Version::parse("1.56.0").unwrap())
        );
    }

    assert_eq!(all.dependencies.len(), 8);
    let bitflags = all
        .dependencies
        .iter()
        .find(|d| d.name == "bitflags")
        .unwrap();
    assert_eq!(
        bitflags.source,
        Some("registry+https://github.com/rust-lang/crates.io-index".to_string())
    );
    assert!(bitflags.optional);
    assert_eq!(bitflags.req, semver::VersionReq::parse("^1.0").unwrap());

    let path_dep = all
        .dependencies
        .iter()
        .find(|d| d.name == "path-dep")
        .unwrap();
    assert_eq!(path_dep.source, None);
    assert_eq!(path_dep.kind, DependencyKind::Normal);
    assert_eq!(path_dep.req, semver::VersionReq::parse("*").unwrap());
    assert_eq!(
        path_dep.path.as_ref().map(|p| p.ends_with("path-dep")),
        Some(true),
    );

    all.dependencies
        .iter()
        .find(|d| d.name == "namedep")
        .unwrap();

    let featdep = all
        .dependencies
        .iter()
        .find(|d| d.name == "featdep")
        .unwrap();
    assert_eq!(featdep.features, vec!["i128"]);
    assert!(!featdep.uses_default_features);

    let renamed = all
        .dependencies
        .iter()
        .find(|d| d.name == "oldname")
        .unwrap();
    assert_eq!(renamed.rename, Some("newname".to_string()));

    let devdep = all
        .dependencies
        .iter()
        .find(|d| d.name == "devdep")
        .unwrap();
    assert_eq!(devdep.kind, DependencyKind::Development);

    let bdep = all.dependencies.iter().find(|d| d.name == "bdep").unwrap();
    assert_eq!(bdep.kind, DependencyKind::Build);

    let windep = all
        .dependencies
        .iter()
        .find(|d| d.name == "windep")
        .unwrap();
    assert_eq!(
        windep.target.as_ref().map(|x| x.to_string()),
        Some("cfg(windows)".to_string())
    );

    macro_rules! get_file_name {
        ($v:expr) => {
            all.targets
                .iter()
                .find(|t| t.src_path.file_name().unwrap() == $v)
                .unwrap()
        };
    }
    assert_eq!(all.targets.len(), 8);
    let lib = get_file_name!("lib.rs");
    assert_eq!(lib.name, "all");
    assert_eq!(
        sorted!(lib.kind),
        vec!["cdylib".into(), "rlib".into(), "staticlib".into()]
    );
    assert_eq!(
        sorted!(lib.crate_types),
        vec!["cdylib".into(), "rlib".into(), "staticlib".into()]
    );
    assert_eq!(lib.required_features.len(), 0);
    assert_eq!(lib.edition, Edition::E2018);
    assert!(lib.doctest);
    assert!(lib.test);
    assert!(lib.doc);

    let main = get_file_name!("main.rs");
    assert_eq!(main.crate_types, vec!["bin".into()]);
    assert_eq!(main.kind, vec!["bin".into()]);
    assert!(!main.doctest);
    assert!(main.test);
    assert!(main.doc);

    let otherbin = get_file_name!("otherbin.rs");
    assert_eq!(otherbin.edition, Edition::E2015);
    assert!(!otherbin.doc);

    let reqfeat = get_file_name!("reqfeat.rs");
    assert_eq!(reqfeat.required_features, vec!["feat2"]);

    let ex1 = get_file_name!("ex1.rs");
    assert_eq!(ex1.kind, vec!["example".into()]);
    assert!(!ex1.test);

    let t1 = get_file_name!("t1.rs");
    assert_eq!(t1.kind, vec!["test".into()]);

    let b1 = get_file_name!("b1.rs");
    assert_eq!(b1.kind, vec!["bench".into()]);

    let build = get_file_name!("build.rs");
    assert_eq!(build.kind, vec!["custom-build".into()]);

    if ver >= semver::Version::parse("1.60.0").unwrap() {
        // 1.60 now reports optional dependencies within the features table
        assert_eq!(all.features.len(), 4);
        assert_eq!(all.features["bitflags"], vec!["dep:bitflags"]);
    } else {
        assert_eq!(all.features.len(), 3);
    }
    assert_eq!(all.features["feat1"].len(), 0);
    assert_eq!(all.features["feat2"].len(), 0);
    assert_eq!(sorted!(all.features["default"]), vec!["bitflags", "feat1"]);

    assert!(all.manifest_path.ends_with("all/Cargo.toml"));
    assert_eq!(all.categories, vec!["command-line-utilities"]);
    assert_eq!(all.keywords, vec!["cli"]);
    assert_eq!(all.readme, Some(Utf8PathBuf::from("README.md")));
    assert!(all.readme().unwrap().ends_with("tests/all/README.md"));
    assert_eq!(
        all.repository,
        Some("https://github.com/oli-obk/cargo_metadata/".to_string())
    );
    assert_eq!(
        all.homepage,
        Some("https://github.com/oli-obk/cargo_metadata/".to_string())
    );
    assert_eq!(
        all.documentation,
        Some("https://docs.rs/cargo_metadata/".to_string())
    );
    assert_eq!(all.edition, Edition::E2018);
    assert_eq!(
        all.metadata,
        json!({
            "docs": {
                "rs": {
                    "all-features": true,
                    "default-target": "x86_64-unknown-linux-gnu",
                    "rustc-args": ["--example-rustc-arg"]
                }
            }
        })
    );

    let resolve = meta.resolve.as_ref().unwrap();
    assert!(resolve.root.as_ref().unwrap().to_string().contains("all"));

    assert_eq!(resolve.nodes.len(), 9);
    let path_dep = resolve
        .nodes
        .iter()
        .find(|n| n.id.to_string().contains("path-dep"))
        .unwrap();
    assert_eq!(path_dep.deps.len(), 0);
    assert_eq!(path_dep.dependencies.len(), 0);
    assert_eq!(path_dep.features.len(), 0);

    let bitflags = resolve
        .nodes
        .iter()
        .find(|n| n.id.to_string().contains("bitflags"))
        .unwrap();
    assert_eq!(bitflags.features, vec!["default"]);

    let featdep = resolve
        .nodes
        .iter()
        .find(|n| n.id.to_string().contains("featdep"))
        .unwrap();
    assert_eq!(featdep.features, vec!["i128"]);

    let all = resolve
        .nodes
        .iter()
        .find(|n| n.id.to_string().contains("all"))
        .unwrap();
    assert_eq!(all.dependencies.len(), 8);
    assert_eq!(all.deps.len(), 8);
    let newname = all.deps.iter().find(|d| d.name == "newname").unwrap();
    assert!(newname.pkg.to_string().contains("oldname"));
    // Note the underscore here.
    let path_dep = all.deps.iter().find(|d| d.name == "path_dep").unwrap();
    assert!(path_dep.pkg.to_string().contains("path-dep"));
    assert_eq!(path_dep.dep_kinds.len(), 1);
    let kind = &path_dep.dep_kinds[0];
    assert_eq!(kind.kind, DependencyKind::Normal);
    assert!(kind.target.is_none());

    let namedep = all
        .deps
        .iter()
        .find(|d| d.name == "different_name")
        .unwrap();
    assert!(namedep.pkg.to_string().contains("namedep"));
    assert_eq!(sorted!(all.features), vec!["bitflags", "default", "feat1"]);

    let bdep = all.deps.iter().find(|d| d.name == "bdep").unwrap();
    assert_eq!(bdep.dep_kinds.len(), 1);
    let kind = &bdep.dep_kinds[0];
    assert_eq!(kind.kind, DependencyKind::Build);
    assert!(kind.target.is_none());

    let devdep = all.deps.iter().find(|d| d.name == "devdep").unwrap();
    assert_eq!(devdep.dep_kinds.len(), 1);
    let kind = &devdep.dep_kinds[0];
    assert_eq!(kind.kind, DependencyKind::Development);
    assert!(kind.target.is_none());

    let windep = all.deps.iter().find(|d| d.name == "windep").unwrap();
    assert_eq!(windep.dep_kinds.len(), 1);
    let kind = &windep.dep_kinds[0];
    assert_eq!(kind.kind, DependencyKind::Normal);
    assert_eq!(
        kind.target.as_ref().map(|x| x.to_string()),
        Some("cfg(windows)".to_string())
    );

    let serialized = serde_json::to_value(meta).unwrap();
    if ver >= semver::Version::parse("1.71.0").unwrap() {
        assert!(serialized.as_object().unwrap()["workspace_default_members"]
            .as_array()
            .is_some());
    } else {
        assert!(!serialized
            .as_object()
            .unwrap()
            .contains_key("workspace_default_members"));
    }
}

#[test]
fn alt_registry() {
    // This is difficult to test (would need to set up a custom index).
    // Just manually check the JSON is handled.
    let json = r#"
{
  "packages": [
    {
      "name": "alt",
      "version": "0.1.0",
      "id": "alt 0.1.0 (path+file:///alt)",
      "source": null,
      "dependencies": [
        {
          "name": "alt2",
          "source": "registry+https://example.com",
          "req": "^0.1",
          "kind": null,
          "rename": null,
          "optional": false,
          "uses_default_features": true,
          "features": [],
          "target": null,
          "registry": "https://example.com"
        }
      ],
      "targets": [
        {
          "kind": [
            "lib"
          ],
          "crate_types": [
            "lib"
          ],
          "name": "alt",
          "src_path": "/alt/src/lib.rs",
          "edition": "2018"
        }
      ],
      "features": {},
      "manifest_path": "/alt/Cargo.toml",
      "metadata": null,
      "authors": [],
      "categories": [],
      "keywords": [],
      "readme": null,
      "repository": null,
      "edition": "2018",
      "links": null
    }
  ],
  "workspace_members": [
    "alt 0.1.0 (path+file:///alt)"
  ],
  "resolve": null,
  "target_directory": "/alt/target",
  "version": 1,
  "workspace_root": "/alt"
}
"#;
    let meta: Metadata = serde_json::from_str(json).unwrap();
    assert_eq!(meta.packages.len(), 1);
    let alt = &meta.packages[0];
    let deps = &alt.dependencies;
    assert_eq!(deps.len(), 1);
    let dep = &deps[0];
    assert_eq!(dep.registry, Some("https://example.com".to_string()));
}

#[test]
fn current_dir() {
    let meta = MetadataCommand::new()
        .current_dir("tests/all/namedep")
        .exec()
        .unwrap();
    let namedep = meta.packages.iter().find(|p| p.name == "namedep").unwrap();
    assert!(namedep.name.starts_with("namedep"));
}

#[test]
fn parse_stream_is_robust() {
    // Proc macros can print stuff to stdout, which naturally breaks JSON messages.
    // Let's check that we don't die horribly in this case, and report an error.
    let json_output = r##"{"reason":"compiler-artifact","package_id":"chatty 0.1.0 (path+file:///chatty-macro/chatty)","manifest_path":"chatty-macro/Cargo.toml","target":{"kind":["proc-macro"],"crate_types":["proc-macro"],"name":"chatty","src_path":"/chatty-macro/chatty/src/lib.rs","edition":"2018","doctest":true},"profile":{"opt_level":"0","debuginfo":2,"debug_assertions":true,"overflow_checks":true,"test":false},"features":[],"filenames":["/chatty-macro/target/debug/deps/libchatty-f2adcff24cdf3bb2.so"],"executable":null,"fresh":false}
Evil proc macro was here!
{"reason":"compiler-artifact","package_id":"chatty-macro 0.1.0 (path+file:///chatty-macro)","manifest_path":"chatty-macro/Cargo.toml","target":{"kind":["lib"],"crate_types":["lib"],"name":"chatty-macro","src_path":"/chatty-macro/src/lib.rs","edition":"2018","doctest":true},"profile":{"opt_level":"0","debuginfo":2,"debug_assertions":true,"overflow_checks":true,"test":false},"features":[],"filenames":["/chatty-macro/target/debug/libchatty_macro.rlib","/chatty-macro/target/debug/deps/libchatty_macro-cb5956ed52a11fb6.rmeta"],"executable":null,"fresh":false}
"##;
    let mut n_messages = 0;
    let mut text = String::new();
    for message in cargo_metadata::Message::parse_stream(json_output.as_bytes()) {
        let message = message.unwrap();
        match message {
            cargo_metadata::Message::TextLine(line) => text = line,
            _ => n_messages += 1,
        }
    }
    assert_eq!(n_messages, 2);
    assert_eq!(text, "Evil proc macro was here!");
}

#[test]
fn advanced_feature_configuration() {
    fn build_features<F: FnOnce(&mut MetadataCommand) -> &mut MetadataCommand>(
        func: F,
    ) -> Vec<String> {
        let mut meta = MetadataCommand::new();
        let meta = meta.manifest_path("tests/all/Cargo.toml");

        let meta = func(meta);
        let meta = meta.exec().unwrap();

        let resolve = meta.resolve.as_ref().unwrap();

        let all = resolve
            .nodes
            .iter()
            .find(|n| !n.features.is_empty())
            .unwrap();

        all.features.clone()
    }

    // Default behavior; tested above
    let default_features = build_features(|meta| meta);
    assert_eq!(
        sorted!(default_features),
        vec!["bitflags", "default", "feat1"]
    );

    // Manually specify the same default features
    let manual_features = build_features(|meta| {
        meta.features(CargoOpt::NoDefaultFeatures)
            .features(CargoOpt::SomeFeatures(vec![
                "feat1".into(),
                "bitflags".into(),
            ]))
    });
    assert_eq!(sorted!(manual_features), vec!["bitflags", "feat1"]);

    // Multiple SomeFeatures is same as one longer SomeFeatures
    let manual_features = build_features(|meta| {
        meta.features(CargoOpt::NoDefaultFeatures)
            .features(CargoOpt::SomeFeatures(vec!["feat1".into()]))
            .features(CargoOpt::SomeFeatures(vec!["feat2".into()]))
    });
    assert_eq!(sorted!(manual_features), vec!["feat1", "feat2"]);

    // No features + All features == All features
    let all_features = build_features(|meta| {
        meta.features(CargoOpt::AllFeatures)
            .features(CargoOpt::NoDefaultFeatures)
    });
    assert_eq!(
        sorted!(all_features),
        vec!["bitflags", "default", "feat1", "feat2"]
    );

    // The '--all-features' flag supersedes other feature flags
    let all_flag_variants = build_features(|meta| {
        meta.features(CargoOpt::SomeFeatures(vec!["feat2".into()]))
            .features(CargoOpt::NoDefaultFeatures)
            .features(CargoOpt::AllFeatures)
    });
    assert_eq!(sorted!(all_flag_variants), sorted!(all_features));
}

#[test]
fn depkind_to_string() {
    assert_eq!(DependencyKind::Normal.to_string(), "normal");
    assert_eq!(DependencyKind::Development.to_string(), "dev");
    assert_eq!(DependencyKind::Build.to_string(), "build");
    assert_eq!(DependencyKind::Unknown.to_string(), "Unknown");
}

#[test]
fn basic_workspace_root_package_exists() {
    // First try with dependencies
    let meta = MetadataCommand::new()
        .manifest_path("tests/basic_workspace/Cargo.toml")
        .exec()
        .unwrap();
    assert_eq!(meta.root_package().unwrap().name, "ex_bin");
    // Now with no_deps, it should still work exactly the same
    let meta = MetadataCommand::new()
        .manifest_path("tests/basic_workspace/Cargo.toml")
        .no_deps()
        .exec()
        .unwrap();
    assert_eq!(
        meta.root_package()
            .expect("workspace root still exists when no_deps used")
            .name,
        "ex_bin"
    );
}

#[test]
fn debuginfo_variants() {
    // Checks behavior for the different debuginfo variants.
    let variants = [
        ("0", ArtifactDebuginfo::None),
        ("1", ArtifactDebuginfo::Limited),
        ("2", ArtifactDebuginfo::Full),
        (
            "\"line-directives-only\"",
            ArtifactDebuginfo::LineDirectivesOnly,
        ),
        ("\"line-tables-only\"", ArtifactDebuginfo::LineTablesOnly),
        ("3", ArtifactDebuginfo::UnknownInt(3)),
        (
            "\"abc\"",
            ArtifactDebuginfo::UnknownString("abc".to_string()),
        ),
        ("null", ArtifactDebuginfo::None),
    ];
    for (value, expected) in variants {
        let s = r#"{"reason":"compiler-artifact","package_id":"cargo_metadata 0.16.0 (path+file:////cargo_metadata)","manifest_path":"/cargo_metadata/Cargo.toml","target":{"kind":["lib"],"crate_types":["lib"],"name":"cargo_metadata","src_path":"/cargo_metadata/src/lib.rs","edition":"2018","doc":true,"doctest":true,"test":true},"profile":{"opt_level":"0","debuginfo":DEBUGINFO,"debug_assertions":true,"overflow_checks":true,"test":false},"features":["default"],"filenames":["/cargo_metadata/target/debug/deps/libcargo_metadata-27f582f7187b9a2c.rmeta"],"executable":null,"fresh":false}"#;
        let message: Message = serde_json::from_str(&s.replace("DEBUGINFO", value)).unwrap();
        match message {
            Message::CompilerArtifact(artifact) => {
                assert_eq!(artifact.profile.debuginfo, expected);
                let de_s = serde_json::to_string(&artifact.profile.debuginfo).unwrap();
                // Note: Roundtrip does not retain null value.
                if value == "null" {
                    assert_eq!(artifact.profile.debuginfo.to_string(), "0");
                    assert_eq!(de_s, "0");
                } else {
                    assert_eq!(
                        artifact.profile.debuginfo.to_string(),
                        value.trim_matches('"')
                    );
                    assert_eq!(de_s, value);
                }
            }
            _ => panic!("unexpected {:?}", message),
        }
    }
}

#[test]
#[should_panic = "WorkspaceDefaultMembers should only be dereferenced on Cargo versions >= 1.71"]
fn missing_workspace_default_members() {
    let meta: Metadata = serde_json::from_str(JSON_OLD_MINIMAL).unwrap();
    let _ = &*meta.workspace_default_members;
}

#[test]
fn workspace_default_members_is_available() {
    // generated with cargo +1.71.0 metadata --format-version 1
    let json = r#"
{
  "packages": [
    {
      "name": "basic",
      "version": "0.1.0",
      "id": "basic 0.1.0 (path+file:///example)",
      "license": null,
      "license_file": null,
      "description": null,
      "source": null,
      "dependencies": [],
      "targets": [
        {
          "kind": [
            "lib"
          ],
          "crate_types": [
            "lib"
          ],
          "name": "basic",
          "src_path": "/example/src/lib.rs",
          "edition": "2021",
          "doc": true,
          "doctest": true,
          "test": true
        }
      ],
      "features": {},
      "manifest_path": "/example/Cargo.toml",
      "metadata": null,
      "publish": null,
      "authors": [],
      "categories": [],
      "keywords": [],
      "readme": null,
      "repository": null,
      "homepage": null,
      "documentation": null,
      "edition": "2021",
      "links": null,
      "default_run": null,
      "rust_version": null
    }
  ],
  "workspace_members": [
    "basic 0.1.0 (path+file:///example)"
  ],
  "workspace_default_members": [
    "basic 0.1.0 (path+file:///example)"
  ],
  "resolve": {
    "nodes": [
      {
        "id": "basic 0.1.0 (path+file:///example)",
        "dependencies": [],
        "deps": [],
        "features": []
      }
    ],
    "root": "basic 0.1.0 (path+file:///example)"
  },
  "target_directory": "/example/target",
  "version": 1,
  "workspace_root": "/example",
  "metadata": null
}
"#;

    let meta: Metadata = serde_json::from_str(json).unwrap();

    assert!(meta.workspace_default_members.is_available());
    assert!(!meta.workspace_default_members.is_missing());
}

#[test]
fn workspace_default_members_is_missing() {
    // generated with cargo +1.70.0 metadata --format-version 1
    let json = r#"
{
  "packages": [
    {
      "name": "basic",
      "version": "0.1.0",
      "id": "basic 0.1.0 (path+file:///example)",
      "license": null,
      "license_file": null,
      "description": null,
      "source": null,
      "dependencies": [],
      "targets": [
        {
          "kind": [
            "lib"
          ],
          "crate_types": [
            "lib"
          ],
          "name": "basic",
          "src_path": "/example/src/lib.rs",
          "edition": "2021",
          "doc": true,
          "doctest": true,
          "test": true
        }
      ],
      "features": {},
      "manifest_path": "/example/Cargo.toml",
      "metadata": null,
      "publish": null,
      "authors": [],
      "categories": [],
      "keywords": [],
      "readme": null,
      "repository": null,
      "homepage": null,
      "documentation": null,
      "edition": "2021",
      "links": null,
      "default_run": null,
      "rust_version": null
    }
  ],
  "workspace_members": [
    "basic 0.1.0 (path+file:///example)"
  ],
  "resolve": {
    "nodes": [
      {
        "id": "basic 0.1.0 (path+file:///example)",
        "dependencies": [],
        "deps": [],
        "features": []
      }
    ],
    "root": "basic 0.1.0 (path+file:///example)"
  },
  "target_directory": "/example/target",
  "version": 1,
  "workspace_root": "/example",
  "metadata": null
}
"#;

    let meta: Metadata = serde_json::from_str(json).unwrap();

    assert!(!meta.workspace_default_members.is_available());
    assert!(meta.workspace_default_members.is_missing());
}

#[test]
fn test_unknown_target_kind_and_crate_type() {
    // Both kind and crate_type set to a type not yet known
    let json = r#"
{
  "packages": [
    {
      "name": "alt",
      "version": "0.1.0",
      "id": "alt 0.1.0 (path+file:///alt)",
      "source": null,
      "dependencies": [],
      "targets": [
        {
          "kind": [
            "future-kind"
          ],
          "crate_types": [
            "future-type"
          ],
          "name": "alt",
          "src_path": "/alt/src/lib.rs",
          "edition": "2018"
        }
      ],
      "features": {},
      "manifest_path": "/alt/Cargo.toml",
      "metadata": null,
      "authors": [],
      "categories": [],
      "keywords": [],
      "readme": null,
      "repository": null,
      "edition": "2018",
      "links": null
    }
  ],
  "workspace_members": [
    "alt 0.1.0 (path+file:///alt)"
  ],
  "resolve": null,
  "target_directory": "/alt/target",
  "version": 1,
  "workspace_root": "/alt"
}
"#;
    let meta: Metadata = serde_json::from_str(json).unwrap();
    assert_eq!(meta.packages.len(), 1);
    assert_eq!(meta.packages[0].targets.len(), 1);
    let target = &meta.packages[0].targets[0];
    assert_eq!(target.kind[0], "future-kind".into());
    assert_eq!(target.crate_types[0], "future-type".into());
}
