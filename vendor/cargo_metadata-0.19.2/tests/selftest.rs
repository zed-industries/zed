use std::env::current_dir;
use std::path::PathBuf;

use semver::Version;

use cargo_metadata::{CargoOpt, Error, MetadataCommand};
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct TestPackageMetadata {
    some_field: bool,
    other_field: String,
}

#[test]
fn metadata() {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();

    let this = &metadata.packages[0];
    assert_eq!(this.name, "cargo_metadata");
    assert_eq!(this.targets.len(), 3);

    let lib = this
        .targets
        .iter()
        .find(|t| t.name == "cargo_metadata")
        .unwrap();
    assert_eq!(lib.kind[0], "lib".into());
    assert_eq!(lib.crate_types[0], "lib".into());

    let selftest = this.targets.iter().find(|t| t.name == "selftest").unwrap();
    assert_eq!(selftest.name, "selftest");
    assert_eq!(selftest.kind[0], "test".into());
    assert_eq!(selftest.crate_types[0], "bin".into());

    let package_metadata = &metadata.packages[0]
        .metadata
        .as_object()
        .expect("package.metadata must be a table.");
    assert_eq!(package_metadata.len(), 1);

    let value = package_metadata.get("cargo_metadata_test").unwrap();
    let test_package_metadata: TestPackageMetadata = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(
        test_package_metadata,
        TestPackageMetadata {
            some_field: true,
            other_field: "foo".into(),
        }
    );
}

#[test]
fn builder_interface() {
    let _ = MetadataCommand::new()
        .manifest_path("Cargo.toml")
        .exec()
        .unwrap();
    let _ = MetadataCommand::new()
        .manifest_path(String::from("Cargo.toml"))
        .exec()
        .unwrap();
    let _ = MetadataCommand::new()
        .manifest_path(PathBuf::from("Cargo.toml"))
        .exec()
        .unwrap();
    let _ = MetadataCommand::new()
        .manifest_path("Cargo.toml")
        .no_deps()
        .exec()
        .unwrap();
    let _ = MetadataCommand::new()
        .manifest_path("Cargo.toml")
        .features(CargoOpt::AllFeatures)
        .exec()
        .unwrap();
    let _ = MetadataCommand::new()
        .manifest_path("Cargo.toml")
        .current_dir(current_dir().unwrap())
        .exec()
        .unwrap();
}

#[test]
fn error1() {
    match MetadataCommand::new().manifest_path("foo").exec() {
        Err(Error::CargoMetadata { stderr }) => assert_eq!(
            stderr.trim(),
            "error: the manifest-path must be a path to a Cargo.toml file"
        ),
        _ => unreachable!(),
    }
}

#[test]
fn error2() {
    match MetadataCommand::new()
        .manifest_path("foo/Cargo.toml")
        .exec()
    {
        Err(Error::CargoMetadata { stderr }) => assert_eq!(
            stderr.trim(),
            "error: manifest path `foo/Cargo.toml` does not exist"
        ),
        _ => unreachable!(),
    }
}

#[test]
fn cargo_path() {
    match MetadataCommand::new()
        .cargo_path("this does not exist")
        .exec()
    {
        Err(Error::Io(e)) => assert_eq!(e.kind(), std::io::ErrorKind::NotFound),
        _ => unreachable!(),
    }
}

#[test]
fn metadata_deps() {
    std::env::set_var("CARGO_PROFILE", "3");
    let metadata = MetadataCommand::new()
        .manifest_path("Cargo.toml")
        .exec()
        .unwrap();
    let this_id = metadata
        .workspace_members
        .first()
        .expect("Did not find ourselves");
    let this = &metadata[this_id];

    assert_eq!(this.name, "cargo_metadata");

    let workspace_packages = metadata.workspace_packages();
    assert_eq!(workspace_packages.len(), 1);
    assert_eq!(&workspace_packages[0].id, this_id);

    let lib = this
        .targets
        .iter()
        .find(|t| t.name == "cargo_metadata")
        .unwrap();
    assert_eq!(lib.kind[0], "lib".into());
    assert_eq!(lib.crate_types[0], "lib".into());

    let selftest = this.targets.iter().find(|t| t.name == "selftest").unwrap();
    assert_eq!(selftest.name, "selftest");
    assert_eq!(selftest.kind[0], "test".into());
    assert_eq!(selftest.crate_types[0], "bin".into());

    let dependencies = &this.dependencies;

    let serde = dependencies
        .iter()
        .find(|dep| dep.name == "serde")
        .expect("Did not find serde dependency");

    assert_eq!(serde.kind, cargo_metadata::DependencyKind::Normal);
    assert!(!serde.req.matches(&Version::parse("1.0.0").unwrap()));
    assert!(serde.req.matches(&Version::parse("1.99.99").unwrap()));
    assert!(!serde.req.matches(&Version::parse("2.0.0").unwrap()));
}

#[test]
fn workspace_default_packages() {
    let metadata = MetadataCommand::new()
        .manifest_path("Cargo.toml")
        .exec()
        .unwrap();
    let workspace_packages = metadata.workspace_packages();
    // this will only trigger on cargo versions that expose
    // workspace_default_members (that is, cargo >= 1.71)
    if metadata.workspace_default_members.is_available() {
        let default_packages = metadata.workspace_default_packages();
        assert_eq!(default_packages, workspace_packages);
    }
}
