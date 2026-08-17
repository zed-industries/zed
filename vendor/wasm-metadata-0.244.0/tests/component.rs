use std::{str::FromStr, vec};

use auditable_serde::VersionInfo;
use wasm_encoder::{Component, Module};
use wasm_metadata::*;

#[test]
fn add_to_empty_component() {
    let mut add = AddMetadata::default();
    add.name = AddMetadataField::Set("foo".to_owned());
    add.language = vec![("bar".to_owned(), "1.0".to_owned())];
    add.processed_by = vec![("baz".to_owned(), "1.0".to_owned())];
    add.sdk = vec![];
    add.authors = AddMetadataField::Set(Authors::new("Chashu Cat"));
    add.description = AddMetadataField::Set(Description::new("Chashu likes tuna"));
    add.licenses = AddMetadataField::Set(
        Licenses::new("Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT").unwrap(),
    );
    add.source = AddMetadataField::Set(
        Source::new("https://github.com/bytecodealliance/wasm-tools").unwrap(),
    );
    add.homepage = AddMetadataField::Set(
        Homepage::new("https://github.com/bytecodealliance/wasm-tools").unwrap(),
    );
    add.revision = AddMetadataField::Set(Revision::new("de978e17a80c1118f606fce919ba9b7d5a04a5ad"));
    add.version = AddMetadataField::Set(Version::new("1.0.0"));

    let json_str = r#"{"packages":[{"name":"adler","version":"0.2.3","source":"registry"}]}"#;
    let info = VersionInfo::from_str(json_str).unwrap();
    let mut component = Component::new();
    component.section(&Dependencies::new(info.clone()));
    let component = component.finish();
    let component = add.to_wasm(&component).unwrap();

    match Payload::from_binary(&component).unwrap() {
        Payload::Component {
            children,
            metadata:
                Metadata {
                    name,
                    producers,
                    authors,
                    description,
                    licenses,
                    source,
                    range,
                    homepage,
                    revision,
                    version,
                    dependencies,
                },
        } => {
            assert!(children.is_empty());
            assert_eq!(name, Some("foo".to_owned()));
            let producers = producers.expect("some producers");
            assert_eq!(
                producers.get("language").unwrap().get("bar").unwrap(),
                "1.0"
            );
            assert_eq!(
                producers.get("processed-by").unwrap().get("baz").unwrap(),
                "1.0"
            );

            assert_eq!(authors.unwrap(), Authors::new("Chashu Cat"));
            assert_eq!(description.unwrap(), Description::new("Chashu likes tuna"));
            assert_eq!(
                licenses.unwrap(),
                Licenses::new("Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT").unwrap()
            );
            assert_eq!(
                source.unwrap(),
                Source::new("https://github.com/bytecodealliance/wasm-tools").unwrap(),
            );
            assert_eq!(
                homepage.unwrap(),
                Homepage::new("https://github.com/bytecodealliance/wasm-tools").unwrap(),
            );
            assert_eq!(
                revision.unwrap(),
                Revision::new("de978e17a80c1118f606fce919ba9b7d5a04a5ad")
            );
            assert_eq!(version.unwrap(), Version::new("1.0.0"));
            assert_eq!(dependencies.unwrap().version_info(), &info,);

            assert_eq!(range.start, 0);
            assert_eq!(range.end, 465);
        }
        _ => panic!("metadata should be component"),
    }
}

#[test]
fn add_to_nested_component() {
    // Create the same old module, stick some metadata into it
    let mut add = AddMetadata::default();
    add.name = AddMetadataField::Set("foo".to_owned());
    add.language = vec![("bar".to_owned(), "1.0".to_owned())];
    add.processed_by = vec![("baz".to_owned(), "1.0".to_owned())];
    add.sdk = vec![];
    add.authors = AddMetadataField::Set(Authors::new("Chashu Cat"));
    add.description = AddMetadataField::Set(Description::new("Chashu likes tuna"));
    add.licenses = AddMetadataField::Set(
        Licenses::new("Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT").unwrap(),
    );
    add.source = AddMetadataField::Set(
        Source::new("https://github.com/bytecodealliance/wasm-tools").unwrap(),
    );
    add.homepage = AddMetadataField::Set(
        Homepage::new("https://github.com/bytecodealliance/wasm-tools").unwrap(),
    );
    add.revision = AddMetadataField::Set(Revision::new("de978e17a80c1118f606fce919ba9b7d5a04a5ad"));
    add.version = AddMetadataField::Set(Version::new("1.0.0"));

    let json_str = r#"{"packages":[{"name":"adler","version":"0.2.3","source":"registry"}]}"#;
    let info = VersionInfo::from_str(json_str).unwrap();
    let mut component = Module::new();
    component.section(&Dependencies::new(info.clone()));
    let module = component.finish();
    let module = add.to_wasm(&module).unwrap();

    // Stick that module inside a component.
    let mut component = wasm_encoder::Component::new();
    component.section(&wasm_encoder::RawSection {
        id: wasm_encoder::ComponentSectionId::CoreModule.into(),
        data: &module,
    });

    let component = component.finish();

    // Add some different metadata to the component.
    let mut add = AddMetadata::default();
    add.name = AddMetadataField::Set("gussie".to_owned());
    add.sdk = vec![("willa".to_owned(), "sparky".to_owned())];
    let component = add.to_wasm(&component).unwrap();

    match Payload::from_binary(&component).unwrap() {
        Payload::Component {
            children,
            metadata: Metadata {
                name, producers, ..
            },
        } => {
            // Check that the component metadata is in the component
            assert_eq!(name, Some("gussie".to_owned()));
            let producers = producers.as_ref().expect("some producers");
            assert_eq!(
                producers.get("sdk").unwrap().get("willa").unwrap(),
                &"sparky".to_owned()
            );
            // Check that there is a single child with the metadata set for the module
            assert_eq!(children.len(), 1);

            match children.get(0).unwrap() {
                Payload::Module(Metadata {
                    name,
                    producers,
                    authors,
                    licenses,
                    source,
                    range,
                    description,
                    homepage,
                    revision,
                    version,
                    dependencies,
                }) => {
                    assert_eq!(name, &Some("foo".to_owned()));
                    let producers = producers.as_ref().expect("some producers");
                    assert_eq!(
                        producers.get("language").unwrap().get("bar").unwrap(),
                        "1.0"
                    );
                    assert_eq!(
                        producers.get("processed-by").unwrap().get("baz").unwrap(),
                        "1.0"
                    );

                    assert_eq!(authors, &Some(Authors::new("Chashu Cat")));
                    assert_eq!(description, &Some(Description::new("Chashu likes tuna")));
                    assert_eq!(
                        licenses,
                        &Some(
                            Licenses::new("Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT")
                                .unwrap()
                        )
                    );
                    assert_eq!(
                        source,
                        &Some(
                            Source::new("https://github.com/bytecodealliance/wasm-tools").unwrap()
                        ),
                    );
                    assert_eq!(
                        homepage,
                        &Some(
                            Homepage::new("https://github.com/bytecodealliance/wasm-tools")
                                .unwrap()
                        ),
                    );
                    assert_eq!(
                        revision,
                        &Some(Revision::new("de978e17a80c1118f606fce919ba9b7d5a04a5ad"))
                    );
                    assert_eq!(version, &Some(Version::new("1.0.0")));
                    assert_eq!(dependencies.as_ref().unwrap().version_info(), &info);
                    assert_eq!(range.start, 11);
                    assert_eq!(range.end, 466);
                }
                _ => panic!("child is a module"),
            }
        }
        _ => panic!("root should be component"),
    }
}

#[test]
fn add_then_clear_fields() {
    let mut add1 = AddMetadata::default();
    add1.name = AddMetadataField::Set("foo".to_owned());
    add1.language = vec![("bar".to_owned(), "1.0".to_owned())];
    add1.processed_by = vec![("baz".to_owned(), "1.0".to_owned())];
    add1.sdk = vec![];
    add1.authors = AddMetadataField::Set(Authors::new("Chashu Cat"));
    add1.description = AddMetadataField::Set(Description::new("Chashu likes tuna"));
    add1.licenses = AddMetadataField::Set(
        Licenses::new("Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT").unwrap(),
    );
    add1.source = AddMetadataField::Set(
        Source::new("https://github.com/bytecodealliance/wasm-tools").unwrap(),
    );
    add1.homepage = AddMetadataField::Set(
        Homepage::new("https://github.com/bytecodealliance/wasm-tools").unwrap(),
    );
    add1.revision =
        AddMetadataField::Set(Revision::new("de978e17a80c1118f606fce919ba9b7d5a04a5ad"));
    add1.version = AddMetadataField::Set(Version::new("1.0.0"));

    let json_str = r#"{"packages":[{"name":"adler","version":"0.2.3","source":"registry"}]}"#;
    let info = VersionInfo::from_str(json_str).unwrap();
    let mut component1 = Component::new();
    component1.section(&Dependencies::new(info.clone()));
    let component1 = component1.finish();
    let component1 = add1.to_wasm(&component1).unwrap();

    let mut add2 = AddMetadata::default();
    add2.name = AddMetadataField::Clear;
    add2.version = AddMetadataField::Clear;
    add2.description = AddMetadataField::Set(Description::new("Chashu likes something else"));
    add2.revision = AddMetadataField::Clear;

    let component2 = add2.to_wasm(&component1).unwrap();

    match Payload::from_binary(&component2).unwrap() {
        Payload::Component {
            children,
            metadata:
                Metadata {
                    name,
                    producers,
                    authors,
                    description,
                    licenses,
                    source,
                    range,
                    homepage,
                    revision,
                    version,
                    dependencies,
                },
        } => {
            assert!(children.is_empty());
            assert_eq!(name, None);
            let producers = producers.expect("some producers");
            assert_eq!(
                producers.get("language").unwrap().get("bar").unwrap(),
                "1.0"
            );
            assert_eq!(
                producers.get("processed-by").unwrap().get("baz").unwrap(),
                "1.0"
            );

            assert_eq!(authors.unwrap(), Authors::new("Chashu Cat"));
            assert_eq!(
                description.unwrap(),
                Description::new("Chashu likes something else")
            );
            assert_eq!(
                licenses.unwrap(),
                Licenses::new("Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT").unwrap()
            );
            assert_eq!(
                source.unwrap(),
                Source::new("https://github.com/bytecodealliance/wasm-tools").unwrap(),
            );
            assert_eq!(
                homepage.unwrap(),
                Homepage::new("https://github.com/bytecodealliance/wasm-tools").unwrap(),
            );
            assert_eq!(revision, None);
            assert_eq!(version, None);
            assert_eq!(dependencies.unwrap().version_info(), &info,);

            assert_eq!(range.start, 0);
            assert_eq!(range.end, 434);
        }
        _ => panic!("metadata should be component"),
    }
}
