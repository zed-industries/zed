use crate::{
    compiler, keywords::CompilationResult, paths::Location, types::JsonType, validator::Validate,
    ValidationError,
};
use serde_json::{Map, Value};

fn compile_reference_validator<'a>(
    ctx: &compiler::Context,
    reference: &str,
    keyword: &str,
) -> Option<CompilationResult<'a>> {
    let current_location = match ctx.absolute_location_uri().map_err(ValidationError::from) {
        Ok(uri) => uri,
        Err(error) => return Some(Err(error)),
    };
    let alias = match ctx
        .resolve_reference_uri(reference)
        .map_err(ValidationError::from)
    {
        Ok(uri) => uri,
        Err(error) => return Some(Err(error)),
    };

    if alias == current_location
        || (reference.is_empty() && alias.strip_fragment() == current_location.strip_fragment())
    {
        // Direct self-reference would recurse indefinitely, treat it as an annotation-only schema.
        // Empty string $ref ("") is a same-document reference per RFC 3986, equivalent to "#"
        return None;
    }

    match ctx.lookup_maybe_recursive(reference) {
        Ok(Some(validator)) => return Some(Ok(validator)),
        Ok(None) => {}
        Err(error) => return Some(Err(error)),
    }

    if let Err(error) = ctx.mark_seen(reference) {
        return Some(Err(ValidationError::from(error)));
    }

    let (contents, resolver, draft) = match ctx.lookup(reference) {
        Ok(resolved) => resolved.into_inner(),
        Err(error) => return Some(Err(ValidationError::from(error))),
    };
    let vocabularies = ctx.registry.find_vocabularies(draft, contents);
    let resource_ref = draft.create_resource_ref(contents);
    let ctx = ctx.with_resolver_and_draft(
        resolver,
        resource_ref.draft(),
        vocabularies,
        ctx.location().join(keyword),
    );
    Some(
        compiler::compile_with_alias(&ctx, resource_ref, alias)
            .map(|node| {
                Box::new(node.clone_with_location(ctx.location().clone(), ctx.base_uri()))
                    as Box<dyn Validate>
            })
            .map_err(ValidationError::to_owned),
    )
}

fn compile_recursive_validator<'a>(
    ctx: &compiler::Context,
    reference: &str,
) -> CompilationResult<'a> {
    // Check if this is a circular reference first
    match ctx.lookup_maybe_recursive(reference) {
        Ok(Some(validator)) => return Ok(validator),
        Ok(None) => {}
        Err(error) => return Err(error),
    }

    if let Err(error) = ctx.mark_seen(reference) {
        return Err(ValidationError::from(error));
    }

    let alias = ctx
        .resolve_reference_uri(reference)
        .map_err(ValidationError::from)?;
    let resolved = ctx
        .lookup_recursive_reference()
        .map_err(ValidationError::from)?;
    let (contents, resolver, draft) = resolved.into_inner();
    let vocabularies = ctx.registry.find_vocabularies(draft, contents);
    let resource_ref = draft.create_resource_ref(contents);
    let ctx = ctx.with_resolver_and_draft(
        resolver,
        resource_ref.draft(),
        vocabularies,
        ctx.location().join("$recursiveRef"),
    );
    compiler::compile_with_alias(&ctx, resource_ref, alias)
        .map(|node| {
            Box::new(node.clone_with_location(ctx.location().clone(), ctx.base_uri()))
                as Box<dyn Validate>
        })
        .map_err(ValidationError::to_owned)
}

fn invalid_reference<'a>(ctx: &compiler::Context, schema: &'a Value) -> ValidationError<'a> {
    ValidationError::single_type_error(
        Location::new(),
        ctx.location().clone(),
        schema,
        JsonType::String,
    )
}

#[inline]
pub(crate) fn compile_impl<'a>(
    ctx: &compiler::Context,
    _parent: &'a Map<String, Value>,
    schema: &'a Value,
    keyword: &str,
) -> Option<CompilationResult<'a>> {
    if let Some(reference) = schema.as_str() {
        compile_reference_validator(ctx, reference, keyword)
    } else {
        Some(Err(invalid_reference(ctx, schema)))
    }
}

#[inline]
pub(crate) fn compile_dynamic_ref<'a>(
    ctx: &compiler::Context,
    parent: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    compile_impl(ctx, parent, schema, "$dynamicRef")
}

#[inline]
pub(crate) fn compile_ref<'a>(
    ctx: &compiler::Context,
    parent: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    compile_impl(ctx, parent, schema, "$ref")
}

#[inline]
pub(crate) fn compile_recursive_ref<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    Some(
        schema
            .as_str()
            .ok_or_else(|| invalid_reference(ctx, schema))
            .and_then(|reference| compile_recursive_validator(ctx, reference)),
    )
}

#[cfg(test)]
mod tests {
    use crate::tests_util;
    use ahash::HashMap;
    use referencing::{Retrieve, Uri};
    use serde_json::{json, Value};
    use test_case::test_case;

    struct MyRetrieve;

    impl Retrieve for MyRetrieve {
        fn retrieve(
            &self,
            uri: &Uri<String>,
        ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
            match uri.path().as_str() {
                "/indirection" => Ok(json!({
                    "$id": "/indirection",
                    "baz": {
                        "$ref": "/types#/foo"
                    }
                })),
                "/types" => Ok(json!({
                    "$id": "/types",
                    "foo": {
                        "$id": "#/foo",
                        "$ref": "#/bar"
                    },
                    "bar": {
                        "type": "integer"
                    }
                })),
                _ => panic!("Not found"),
            }
        }
    }

    #[test]
    fn custom_retrieve_can_load_remote() {
        let retriever = MyRetrieve;
        let uri = Uri::try_from("https://example.com/types".to_string()).expect("valid uri");
        let value: Value = retriever
            .retrieve(&uri)
            .expect("should load the remote document");
        let bar = value
            .get("bar")
            .and_then(|schema| schema.get("type"))
            .cloned();
        assert_eq!(bar, Some(json!("integer")));
    }

    struct TestRetrieve {
        storage: HashMap<String, Value>,
    }

    impl Retrieve for TestRetrieve {
        fn retrieve(
            &self,
            uri: &Uri<String>,
        ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
            self.storage
                .get(uri.path().as_str())
                .cloned()
                .ok_or_else(|| "Document not found".into())
        }
    }

    struct NestedRetrieve;

    impl Retrieve for NestedRetrieve {
        fn retrieve(
            &self,
            uri: &Uri<String>,
        ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
            match uri.as_str() {
                "foo://schema_2.json" => Ok(json!({
                    "$id": "foo://schema_2.json",
                    "type": "string"
                })),
                _ => panic!("Unexpected URI: {}", uri.path()),
            }
        }
    }

    struct FragmentRetrieve;

    impl Retrieve for FragmentRetrieve {
        fn retrieve(
            &self,
            uri: &Uri<String>,
        ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
            match uri.path().as_str() {
                "/tmp/schemas/one.json" => Ok(json!({
                    "$defs": {
                        "obj": {
                            "$ref": "other.json#/$defs/obj"
                        }
                    }
                })),
                "/tmp/schemas/other.json" => Ok(json!({
                    "$defs": {
                        "obj": {
                            "type": "number"
                        }
                    }
                })),
                _ => panic!("Unexpected URI: {}", uri.path()),
            }
        }
    }

    #[test_case(
        &json!({
            "properties": {
                "foo": {"$ref": "#/definitions/foo"}
            },
            "definitions": {
                "foo": {"type": "string"}
            }
        }),
        &json!({"foo": 42}),
        "/properties/foo/$ref/type"
    )]
    fn location(schema: &Value, instance: &Value, expected: &str) {
        tests_util::assert_schema_location(schema, instance, expected);
    }

    #[test]
    fn multiple_errors_locations() {
        let instance = json!({
            "things": [
                { "code": "CC" },
                { "code": "CC" },
            ]
        });
        let schema = json!({
                "type": "object",
                "properties": {
                    "things": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "code": {
                                    "type": "string",
                                    "$ref": "#/$defs/codes"
                                }
                            },
                            "required": ["code"]
                        }
                    }
                },
                "required": ["things"],
                "$defs": { "codes": { "enum": ["AA", "BB"] } }
        });
        let validator = crate::validator_for(&schema).expect("Invalid schema");
        let mut iter = validator.iter_errors(&instance);
        let expected = "/properties/things/items/properties/code/$ref/enum";
        assert_eq!(
            iter.next()
                .expect("Should be present")
                .schema_path()
                .to_string(),
            expected
        );
        assert_eq!(
            iter.next()
                .expect("Should be present")
                .schema_path()
                .to_string(),
            expected
        );
    }

    #[test]
    fn test_relative_base_uri() {
        let schema = json!({
            "$id": "/root",
            "$ref": "#/foo",
            "foo": {
                "$id": "#/foo",
                "$ref": "#/bar"
            },
            "bar": {
                "$id": "#/bar",
                "type": "integer"
            },
        });
        let validator = crate::validator_for(&schema).expect("Invalid schema");
        assert!(validator.is_valid(&json!(2)));
        assert!(!validator.is_valid(&json!("a")));
    }

    #[test_case(
        &json!({
            "$id": "https://example.com/schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "foo": {
                    "type": "array",
                    "items": { "$ref": "#/$defs/item" }
                }
            },
            "$defs": {
                "item": {
                    "type": "object",
                    "required": ["name", "value"],
                    "properties": {
                        "name": { "type": "string" },
                        "value": { "type": "boolean" }
                    }
                }
            }
        }),
        &json!({
            "foo": [{"name": "item1", "value": true}]
        }),
        vec![
            ("", "/properties"),
            ("/foo", "/properties/foo/items"),
            ("/foo/0", "/properties/foo/items/$ref/properties"),
        ]
    ; "standard $ref")]
    #[test_case(
        &json!({
            "$id": "https://example.com/schema.json",
            "$schema": "https://json-schema.org/draft/2019-09/schema",
            "$recursiveAnchor": true,
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "child": { "$recursiveRef": "#" }
            }
        }),
        &json!({
            "name": "parent",
            "child": {
                "name": "child",
                "child": { "name": "grandchild" }
            }
        }),
        vec![
            ("", "/properties"),
            ("/child", "/properties/child/$recursiveRef/properties"),
            ("/child/child", "/properties/child/$recursiveRef/properties"),
        ]
    ; "$recursiveRef")]
    fn keyword_locations(schema: &Value, instance: &Value, expected: Vec<(&str, &str)>) {
        let validator = crate::validator_for(schema).expect("Invalid schema");
        for (pointer, keyword_location) in expected {
            tests_util::assert_keyword_location(&validator, instance, pointer, keyword_location);
        }
    }

    #[test]
    fn test_resolving_finds_references_in_referenced_resources() {
        let schema = json!({"$ref": "/indirection#/baz"});

        let validator = crate::options()
            .with_retriever(MyRetrieve)
            .build(&schema)
            .expect("Failed to build validator");

        assert!(validator.is_valid(&json!(2)));
        assert!(!validator.is_valid(&json!("")));
    }

    #[test_case(
        &json!({"$ref": "/doc#/definitions/foo"}),
        &json!({
            "$id": "/doc",
            "definitions": {
                "foo": {"type": "integer"}
            }
        }),
        None
        ; "basic_fragment"
    )]
    #[test_case(
        &json!({"$ref": "/doc1#/definitions/foo"}),
        &json!({
            "$id": "/doc1",
            "definitions": {
                "foo": {"$ref": "#/definitions/bar"},
                "bar": {"type": "integer"}
            }
        }),
        None
        ; "intermediate_reference"
    )]
    #[test_case(
        &json!({"$ref": "/doc2#/refs/first"}),
        &json!({
            "$id": "/doc2",
            "refs": {
                "first": {"$ref": "/doc3#/refs/second"}
            }
        }),
        Some(&json!({
            "/doc3": {
                "$id": "/doc3",
                "refs": {
                    "second": {"type": "integer"}
                }
            }
        }))
        ; "multiple_documents"
    )]
    #[test_case(
        &json!({"$ref": "/doc4#/defs/foo"}),
        &json!({
            "$id": "/doc4",
            "defs": {
                "foo": {
                    "$id": "#/defs/foo",
                    "$ref": "#/defs/bar"
                },
                "bar": {"type": "integer"}
            }
        }),
        None
        ; "id_and_fragment"
    )]
    #[test_case(
        &json!({"$ref": "/doc5#/outer"}),
        &json!({
            "$id": "/doc5",
            "outer": {
                "$ref": "#/middle",
            },
            "middle": {
                "$id": "#/middle",
                "$ref": "#/inner"
            },
            "inner": {"type": "integer"}
        }),
        None
        ; "nested_references"
    )]
    fn test_fragment_resolution(schema: &Value, root: &Value, extra: Option<&Value>) {
        let mut storage = HashMap::default();

        let doc_path = schema["$ref"]
            .as_str()
            .and_then(|r| r.split('#').next())
            .expect("Invalid $ref");

        storage.insert(doc_path.to_string(), root.clone());

        if let Some(extra) = extra {
            for (path, document) in extra.as_object().unwrap() {
                storage.insert(path.clone(), document.clone());
            }
        }

        let retriever = TestRetrieve { storage };

        let validator = crate::options()
            .with_retriever(retriever)
            .build(schema)
            .expect("Invalid schema");

        assert!(validator.is_valid(&json!(42)));
        assert!(!validator.is_valid(&json!("string")));
    }

    #[test]
    fn test_infinite_loop() {
        let validator = crate::validator_for(&json!({"$ref": "#"})).expect("Invalid schema");
        assert!(validator.is_valid(&json!(42)));
    }

    #[test]
    fn test_nested_external_reference() {
        let schema = json!({
            "$id": "foo://schema_1.json",
            "$ref": "#/$defs/a/b",
            "$defs": {
                "a": {
                    "b": {
                        "description": "nested schema with external ref",
                        "$ref": "foo://schema_2.json"
                    }
                }
            }
        });

        let validator = crate::options()
            .with_retriever(NestedRetrieve)
            .build(&schema)
            .expect("Failed to build validator");

        assert!(validator.is_valid(&json!("test")));
        assert!(!validator.is_valid(&json!(42)));
    }

    #[test]
    fn test_relative_reference_with_fragment() {
        let schema = json!({
            "$id": "file:///tmp/schemas/root.json",
            "$ref": "one.json#/$defs/obj"
        });

        let validator = crate::options()
            .with_retriever(FragmentRetrieve)
            .build(&schema)
            .expect("Failed to build validator");

        assert!(validator.is_valid(&json!(42)));
        assert!(!validator.is_valid(&json!("string")));
    }

    #[test]
    fn test_missing_file() {
        let schema = json!({"$ref": "./virtualNetwork.json"});
        let error = crate::validator_for(&schema).expect_err("Should fail");
        assert_eq!(
            error.to_string(),
            "Resource './virtualNetwork.json' is not present in a registry and retrieving it failed: No base URI is available"
        );
    }

    #[test]
    fn test_empty_ref_no_stack_overflow() {
        // Empty string is a same-document reference per RFC 3986, should behave like $ref: "#"
        let schema = json!({"$ref": ""});
        let instance = json!(-1);

        // Should compile without error and validate without stack overflow
        let validator = crate::validator_for(&schema).expect("Should compile");
        assert!(validator.is_valid(&instance));
    }

    struct IndirectExternalRetrieve;

    impl Retrieve for IndirectExternalRetrieve {
        fn retrieve(
            &self,
            uri: &Uri<String>,
        ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
            match uri.as_str() {
                "file:///ext.yaml" => Ok(json!({
                    "components": {
                        "schemas": {
                            "c3": {
                                "type": "integer"
                            }
                        }
                    }
                })),
                _ => Err(format!("Unexpected URI: {uri}").into()),
            }
        }
    }

    #[test]
    fn test_indirect_local_refs_to_external_resource() {
        // GH-892: Chained local $refs where the final ref points to an external resource
        // should properly discover and retrieve the external resource.
        //
        // The chain is:
        //   root $ref -> #/components/schemas/c1
        //   c1 $ref   -> #/components/schemas/c2
        //   c2 $ref   -> ext.yaml#/components/schemas/c3  (EXTERNAL)
        let schema = json!({
            "$id": "file:///tmp",
            "$ref": "#/components/schemas/c1",
            "components": {
                "schemas": {
                    "c1": {
                        "$ref": "#/components/schemas/c2"
                    },
                    "c2": {
                        "$ref": "ext.yaml#/components/schemas/c3"
                    }
                }
            }
        });

        let validator = crate::options()
            .with_retriever(IndirectExternalRetrieve)
            .build(&schema)
            .expect("Failed to build validator - external resource was not discovered");

        assert!(validator.is_valid(&json!(42)));
        assert!(!validator.is_valid(&json!("string")));
    }

    #[test]
    fn test_local_ref_with_nested_external_ref_in_properties() {
        // GH-892 follow-up: Local $ref points to a schema that has an external $ref
        // nested within properties (not a direct $ref chain).
        //
        // The structure is:
        //   root $ref -> #/components/schemas/c1
        //   c1 is a full schema with type/properties
        //   c1.properties.p contains an external $ref
        let schema = json!({
            "$id": "file:///tmp",
            "$ref": "#/components/schemas/c1",
            "components": {
                "schemas": {
                    "c1": {
                        "type": "object",
                        "properties": {
                            "p": {
                                "$ref": "ext.yaml#/components/schemas/c3"
                            }
                        }
                    }
                }
            }
        });

        let validator = crate::options()
            .with_retriever(IndirectExternalRetrieve)
            .build(&schema)
            .expect("Failed to build validator - external resource was not discovered");

        assert!(validator.is_valid(&json!({"p": 42})));
        assert!(!validator.is_valid(&json!({"p": "string"})));
    }

    struct CrossFileRetrieve;

    impl Retrieve for CrossFileRetrieve {
        fn retrieve(
            &self,
            uri: &Uri<String>,
        ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
            match uri.as_str() {
                "file:///tmp/json" => Ok(json!({
                    "components": {
                        "schemas": {
                            "c1": {
                                "type": "array",
                                "items": {
                                    "$ref": "#/components/schemas/c2"
                                }
                            },
                            "c2": {
                                "$ref": "ext.json#/components/schemas/c3"
                            }
                        }
                    }
                })),
                "file:///tmp/ext.json" => Ok(json!({
                    "components": {
                        "schemas": {
                            "c3": {
                                "type": "integer"
                            }
                        }
                    }
                })),
                _ => Err(format!("Unexpected URI: {uri}").into()),
            }
        }
    }

    #[test]
    fn test_cross_file_local_ref_resolution() {
        // GH-892: External ref with fragment pointing to a schema that has local refs.
        // The local refs within the external file need to resolve against that file's
        // document root, not the original schema's root.
        //
        // Structure:
        //   root $ref -> file:///tmp/json#/components/schemas/c1
        //   /tmp/json has:
        //     c1.items.$ref -> #/components/schemas/c2 (local ref within /tmp/json)
        //     c2.$ref -> ext.json#/components/schemas/c3 (external ref)
        let schema = json!({
            "$ref": "file:///tmp/json#/components/schemas/c1"
        });

        let validator = crate::options()
            .with_retriever(CrossFileRetrieve)
            .build(&schema)
            .expect("Failed to build validator - external resource was not discovered");

        assert!(validator.is_valid(&json!([1, 2, 3])));
        assert!(!validator.is_valid(&json!(["a", "b"])));
    }

    #[test]
    fn test_circular_local_refs_compile() {
        let schema = json!({
            "$defs": {
                "a": {"$ref": "#/$defs/b"},
                "b": {"$ref": "#/$defs/a"}
            },
            "$ref": "#/$defs/a"
        });
        let validator = crate::validator_for(&schema).expect("Should compile");

        // A pure $ref cycle is equivalent to `true` schema
        for instance in [
            json!(42),
            json!("string"),
            json!(null),
            json!({"nested": [1, 2, 3]}),
        ] {
            assert!(validator.is_valid(&instance));
            assert!(validator.validate(&instance).is_ok());
            assert_eq!(validator.iter_errors(&instance).count(), 0);
            assert!(validator.evaluate(&instance).flag().valid);
        }
    }

    #[test]
    fn test_circular_refs_with_constraints() {
        let schema = json!({
            "$defs": {
                "node": {
                    "type": "object",
                    "properties": {
                        "value": {"type": "integer"},
                        "next": {"$ref": "#/$defs/node"}
                    }
                }
            },
            "$ref": "#/$defs/node"
        });
        let validator = crate::validator_for(&schema).expect("Should compile");

        let valid = json!({"value": 1, "next": {"value": 2, "next": {"value": 3}}});
        assert!(validator.is_valid(&valid));
        assert!(validator.validate(&valid).is_ok());
        assert_eq!(validator.iter_errors(&valid).count(), 0);

        let invalid = json!({"value": "not an int"});
        assert!(!validator.is_valid(&invalid));
        assert!(validator.validate(&invalid).is_err());
        assert!(validator.iter_errors(&invalid).count() > 0);

        let invalid_nested = json!({"value": 1, "next": {"value": "bad"}});
        assert!(!validator.is_valid(&invalid_nested));
        assert!(validator.validate(&invalid_nested).is_err());
        assert!(validator.iter_errors(&invalid_nested).count() > 0);
    }

    #[test]
    fn test_longer_circular_chain() {
        let schema = json!({
            "$defs": {
                "a": {"$ref": "#/$defs/b"},
                "b": {"$ref": "#/$defs/c"},
                "c": {"$ref": "#/$defs/a"}
            },
            "$ref": "#/$defs/a"
        });
        let validator = crate::validator_for(&schema).expect("Should compile");

        let instance = json!({"any": "value"});
        assert!(validator.is_valid(&instance));
        assert!(validator.validate(&instance).is_ok());
        assert_eq!(validator.iter_errors(&instance).count(), 0);
        assert!(validator.evaluate(&instance).flag().valid);
    }

    #[test]
    fn test_dependencies_with_array_form() {
        // Tests NodeValidators::Array branch via dependencies with array form
        let schema = json!({
            "dependencies": {
                "foo": ["bar", "baz"]
            }
        });
        let validator = crate::validator_for(&schema).expect("Should compile");

        let valid = json!({"foo": 1, "bar": 2, "baz": 3});
        assert!(validator.is_valid(&valid));
        assert!(validator.validate(&valid).is_ok());
        assert_eq!(validator.iter_errors(&valid).count(), 0);
        assert!(validator.evaluate(&valid).flag().valid);

        let invalid = json!({"foo": 1});
        assert!(!validator.is_valid(&invalid));
        assert!(validator.validate(&invalid).is_err());
        assert!(validator.iter_errors(&invalid).count() > 0);
        assert!(!validator.evaluate(&invalid).flag().valid);
    }

    #[test]
    fn test_dependent_required_array_form() {
        // Tests NodeValidators::Array branch via dependentRequired (Draft 2019-09+)
        let schema = json!({
            "$schema": "https://json-schema.org/draft/2019-09/schema",
            "dependentRequired": {
                "foo": ["bar"]
            }
        });
        let validator = crate::validator_for(&schema).expect("Should compile");

        let valid = json!({"foo": 1, "bar": 2});
        assert!(validator.is_valid(&valid));
        assert!(validator.validate(&valid).is_ok());
        assert_eq!(validator.iter_errors(&valid).count(), 0);
        assert!(validator.evaluate(&valid).flag().valid);

        let invalid = json!({"foo": 1});
        assert!(!validator.is_valid(&invalid));
        assert!(validator.validate(&invalid).is_err());
        assert!(validator.iter_errors(&invalid).count() > 0);
    }
}
