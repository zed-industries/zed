/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_protocol_test::{validate_body, MediaType};
use aws_smithy_xml::encode::{ScopeWriter, XmlEncodeError, XmlWriter};

// @namespace http://www.example.com
struct WithNamespace {
    foo: String,
    bar: String,
}

struct Nested {
    // @xmlAttribute("a")
    a: String,
    inner: WithNamespace,
}

fn serialize_nested(nested: &Nested) -> Result<String, XmlEncodeError> {
    let mut out = String::new();
    {
        let mut writer = XmlWriter::new(&mut out);
        let mut start_el = writer.start_el("Nested");
        start_el.write_attribute("a", &nested.a);
        let mut tag = start_el.finish();
        let mut inner = tag.start_el("inner").finish();
        with_namespace_inner(&mut inner, &nested.inner);
    }
    Ok(out)
}

fn serialize_with_namespace(with_namespace: &WithNamespace) -> Result<String, XmlEncodeError> {
    let mut out = String::new();
    {
        let mut writer = XmlWriter::new(&mut out);
        let root = writer.start_el("MyStructure");
        let mut root_scope = root.write_ns("http://foo.com", None).finish();
        with_namespace_inner(&mut root_scope, with_namespace);
        root_scope.finish();
    }

    Ok(out)
}

fn with_namespace_inner(tag: &mut ScopeWriter, with_namespace: &WithNamespace) {
    let mut foo_scope = tag.start_el("foo").finish();
    foo_scope.data(&with_namespace.foo);
    foo_scope.finish();

    let mut bar_scope = tag.start_el("bar").finish();
    bar_scope.data(&with_namespace.bar);
    bar_scope.finish();
}

#[test]
fn test_serialize_with_namespace() {
    let inp = WithNamespace {
        foo: "FooFoo".to_string(),
        bar: "BarBar".to_string(),
    };

    validate_body(
        serialize_with_namespace(&inp).unwrap(),
        r#"<MyStructure xmlns="http://foo.com">
            <foo>FooFoo</foo>
            <bar>BarBar</bar>
        </MyStructure>"#,
        MediaType::Xml,
    )
    .expect("correct XML should be generated");
}

#[test]
fn test_serialize_nested() {
    let inp = Nested {
        a: "avalue".to_string(),
        inner: WithNamespace {
            foo: "foovalue".to_string(),
            bar: "barvalue".to_string(),
        },
    };

    validate_body(
        serialize_nested(&inp).unwrap(),
        r#"<Nested a="avalue">
            <inner>
                <foo>foovalue</foo>
                <bar>barvalue</bar>
            </inner>
        </Nested>"#,
        MediaType::Xml,
    )
    .expect("correct XML should be generated");
}
