pub(crate) mod additional_items;
pub(crate) mod additional_properties;
pub(crate) mod all_of;
pub(crate) mod any_of;
pub(crate) mod boolean;
pub(crate) mod const_;
pub(crate) mod contains;
pub(crate) mod content;
pub(crate) mod custom;
pub(crate) mod dependencies;
pub(crate) mod enum_;
pub(crate) mod format;
pub(crate) mod helpers;
pub(crate) mod if_;
pub(crate) mod items;
pub(crate) mod legacy;
pub(crate) mod max_items;
pub(crate) mod max_length;
pub(crate) mod max_properties;
pub(crate) mod min_items;
pub(crate) mod min_length;
pub(crate) mod min_properties;
pub(crate) mod minmax;
pub(crate) mod multiple_of;
pub(crate) mod not;
pub(crate) mod one_of;
pub(crate) mod pattern;
pub(crate) mod pattern_properties;
pub(crate) mod prefix_items;
pub(crate) mod properties;
pub(crate) mod property_names;
pub(crate) mod ref_;
pub(crate) mod required;
pub(crate) mod type_;
pub(crate) mod unevaluated_items;
pub(crate) mod unevaluated_properties;
pub(crate) mod unique_items;
use core::fmt;

use referencing::{Draft, Vocabulary};
use serde_json::{Map, Value};

use crate::{compiler, error, validator::Validate};

pub(crate) type CompilationResult<'a> = Result<BoxedValidator, error::ValidationError<'a>>;
pub(crate) type BoxedValidator = Box<dyn Validate>;

type CompileFunc<'a> =
    fn(&'a compiler::Context, &'a Map<String, Value>, &'a Value) -> Option<CompilationResult<'a>>;

#[derive(Debug, Clone)]
pub(crate) enum Keyword {
    Builtin(BuiltinKeyword),
    Custom(Box<str>),
}

#[derive(Debug, Clone)]
pub(crate) enum BuiltinKeyword {
    Ref,
    AdditionalItems,
    AdditionalProperties,
    AllOf,
    AnyOf,
    Dependencies,
    Enum,
    Format,
    Items,
    MaxItems,
    MaxLength,
    MaxProperties,
    MinItems,
    MinLength,
    MinProperties,
    MultipleOf,
    Not,
    OneOf,
    Pattern,
    PatternProperties,
    Properties,
    Required,
    UniqueItems,
    Maximum,
    Minimum,
    Type,
    Const,
    Contains,
    ExclusiveMaximum,
    ExclusiveMinimum,
    PropertyNames,
    ContentMediaType,
    ContentEncoding,
    If,
    RecursiveRef,
    DependentRequired,
    DependentSchemas,
    PrefixItems,
    UnevaluatedItems,
    UnevaluatedProperties,
    DynamicRef,
}

impl BuiltinKeyword {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Ref => "$ref",
            Self::AdditionalItems => "additionalItems",
            Self::AdditionalProperties => "additionalProperties",
            Self::AllOf => "allOf",
            Self::AnyOf => "anyOf",
            Self::Dependencies => "dependencies",
            Self::Enum => "enum",
            Self::Format => "format",
            Self::Items => "items",
            Self::MaxItems => "maxItems",
            Self::MaxLength => "maxLength",
            Self::MaxProperties => "maxProperties",
            Self::MinItems => "minItems",
            Self::MinLength => "minLength",
            Self::MinProperties => "minProperties",
            Self::MultipleOf => "multipleOf",
            Self::Not => "not",
            Self::OneOf => "oneOf",
            Self::Pattern => "pattern",
            Self::PatternProperties => "patternProperties",
            Self::Properties => "properties",
            Self::Required => "required",
            Self::UniqueItems => "uniqueItems",
            Self::Maximum => "maximum",
            Self::Minimum => "minimum",
            Self::Type => "type",
            Self::Const => "const",
            Self::Contains => "contains",
            Self::ExclusiveMaximum => "exclusiveMaximum",
            Self::ExclusiveMinimum => "exclusiveMinimum",
            Self::PropertyNames => "propertyNames",
            Self::ContentMediaType => "contentMediaType",
            Self::ContentEncoding => "contentEncoding",
            Self::If => "if",
            Self::RecursiveRef => "$recursiveRef",
            Self::DependentRequired => "dependentRequired",
            Self::DependentSchemas => "dependentSchemas",
            Self::PrefixItems => "prefixItems",
            Self::UnevaluatedItems => "unevaluatedItems",
            Self::UnevaluatedProperties => "unevaluatedProperties",
            Self::DynamicRef => "$dynamicRef",
        }
    }
}

impl Keyword {
    pub(crate) fn custom(name: impl Into<String>) -> Self {
        Keyword::Custom(name.into().into_boxed_str())
    }
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Builtin(d) => d.as_str(),
            Self::Custom(s) => s,
        }
    }
}

impl From<BuiltinKeyword> for Keyword {
    fn from(value: BuiltinKeyword) -> Self {
        Keyword::Builtin(value)
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub(crate) fn get_for_draft<'a>(
    ctx: &compiler::Context<'a>,
    keyword: &'a str,
) -> Option<(Keyword, CompileFunc<'a>)> {
    match (ctx.draft(), keyword) {
        // Keywords common to all drafts
        (_, "$ref") => Some((BuiltinKeyword::Ref.into(), ref_::compile_ref)),
        (_, "additionalItems") if ctx.has_vocabulary(&Vocabulary::Applicator) => Some((
            BuiltinKeyword::AdditionalItems.into(),
            additional_items::compile,
        )),
        (_, "additionalProperties") if ctx.has_vocabulary(&Vocabulary::Applicator) => Some((
            BuiltinKeyword::AdditionalProperties.into(),
            additional_properties::compile,
        )),
        (_, "allOf") if ctx.has_vocabulary(&Vocabulary::Applicator) => {
            Some((BuiltinKeyword::AllOf.into(), all_of::compile))
        }
        (_, "anyOf") if ctx.has_vocabulary(&Vocabulary::Applicator) => {
            Some((BuiltinKeyword::AnyOf.into(), any_of::compile))
        }
        (_, "dependencies") if ctx.has_vocabulary(&Vocabulary::Applicator) => {
            Some((BuiltinKeyword::Dependencies.into(), dependencies::compile))
        }
        (_, "enum") if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::Enum.into(), enum_::compile))
        }
        (Draft::Draft201909, "format") if ctx.has_vocabulary(&Vocabulary::Format) => {
            Some((BuiltinKeyword::Format.into(), format::compile))
        }
        (Draft::Draft202012 | Draft::Unknown, "format")
            if ctx.has_vocabulary(&Vocabulary::FormatAnnotation) =>
        {
            Some((BuiltinKeyword::Format.into(), format::compile))
        }
        (_, "format") => Some((BuiltinKeyword::Format.into(), format::compile)),
        (_, "items") if ctx.has_vocabulary(&Vocabulary::Applicator) => {
            Some((BuiltinKeyword::Items.into(), items::compile))
        }
        (_, "maxItems") if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::MaxItems.into(), max_items::compile))
        }
        (_, "maxLength") if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::MaxLength.into(), max_length::compile))
        }
        (_, "maxProperties") if ctx.has_vocabulary(&Vocabulary::Validation) => Some((
            BuiltinKeyword::MaxProperties.into(),
            max_properties::compile,
        )),
        (_, "minItems") if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::MinItems.into(), min_items::compile))
        }
        (_, "minLength") if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::MinLength.into(), min_length::compile))
        }
        (_, "minProperties") if ctx.has_vocabulary(&Vocabulary::Validation) => Some((
            BuiltinKeyword::MinProperties.into(),
            min_properties::compile,
        )),
        (_, "multipleOf") if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::MultipleOf.into(), multiple_of::compile))
        }
        (_, "not") if ctx.has_vocabulary(&Vocabulary::Applicator) => {
            Some((BuiltinKeyword::Not.into(), not::compile))
        }
        (_, "oneOf") if ctx.has_vocabulary(&Vocabulary::Applicator) => {
            Some((BuiltinKeyword::OneOf.into(), one_of::compile))
        }
        (_, "pattern") if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::Pattern.into(), pattern::compile))
        }
        (_, "patternProperties") if ctx.has_vocabulary(&Vocabulary::Applicator) => Some((
            BuiltinKeyword::PatternProperties.into(),
            pattern_properties::compile,
        )),
        (_, "properties") if ctx.has_vocabulary(&Vocabulary::Applicator) => {
            Some((BuiltinKeyword::Properties.into(), properties::compile))
        }
        (_, "required") if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::Required.into(), required::compile))
        }
        (_, "uniqueItems") if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::UniqueItems.into(), unique_items::compile))
        }
        // Draft 4 specific
        (Draft::Draft4, "maximum") => Some((
            BuiltinKeyword::Maximum.into(),
            legacy::maximum_draft_4::compile,
        )),
        (Draft::Draft4, "minimum") => Some((
            BuiltinKeyword::Minimum.into(),
            legacy::minimum_draft_4::compile,
        )),
        (Draft::Draft4, "type") => {
            Some((BuiltinKeyword::Type.into(), legacy::type_draft_4::compile))
        }
        // Draft 6 and later
        (
            Draft::Draft6
            | Draft::Draft7
            | Draft::Draft201909
            | Draft::Draft202012
            | Draft::Unknown,
            "const",
        ) if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::Const.into(), const_::compile))
        }
        (
            Draft::Draft6
            | Draft::Draft7
            | Draft::Draft201909
            | Draft::Draft202012
            | Draft::Unknown,
            "contains",
        ) if ctx.has_vocabulary(&Vocabulary::Applicator) => {
            Some((BuiltinKeyword::Contains.into(), contains::compile))
        }
        (
            Draft::Draft6
            | Draft::Draft7
            | Draft::Draft201909
            | Draft::Draft202012
            | Draft::Unknown,
            "exclusiveMaximum",
        ) if ctx.has_vocabulary(&Vocabulary::Validation) => Some((
            BuiltinKeyword::ExclusiveMaximum.into(),
            minmax::compile_exclusive_maximum,
        )),
        (
            Draft::Draft6
            | Draft::Draft7
            | Draft::Draft201909
            | Draft::Draft202012
            | Draft::Unknown,
            "exclusiveMinimum",
        ) if ctx.has_vocabulary(&Vocabulary::Validation) => Some((
            BuiltinKeyword::ExclusiveMinimum.into(),
            minmax::compile_exclusive_minimum,
        )),
        (
            Draft::Draft6
            | Draft::Draft7
            | Draft::Draft201909
            | Draft::Draft202012
            | Draft::Unknown,
            "maximum",
        ) if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::Maximum.into(), minmax::compile_maximum))
        }
        (
            Draft::Draft6
            | Draft::Draft7
            | Draft::Draft201909
            | Draft::Draft202012
            | Draft::Unknown,
            "minimum",
        ) if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::Minimum.into(), minmax::compile_minimum))
        }
        (
            Draft::Draft6
            | Draft::Draft7
            | Draft::Draft201909
            | Draft::Draft202012
            | Draft::Unknown,
            "propertyNames",
        ) if ctx.has_vocabulary(&Vocabulary::Applicator) => Some((
            BuiltinKeyword::PropertyNames.into(),
            property_names::compile,
        )),
        (
            Draft::Draft6
            | Draft::Draft7
            | Draft::Draft201909
            | Draft::Draft202012
            | Draft::Unknown,
            "type",
        ) if ctx.has_vocabulary(&Vocabulary::Validation) => {
            Some((BuiltinKeyword::Type.into(), type_::compile))
        }
        (Draft::Draft6 | Draft::Draft7, "contentMediaType") => Some((
            BuiltinKeyword::ContentMediaType.into(),
            content::compile_media_type,
        )),
        (Draft::Draft6 | Draft::Draft7, "contentEncoding") => Some((
            BuiltinKeyword::ContentEncoding.into(),
            content::compile_content_encoding,
        )),
        (Draft::Draft7 | Draft::Draft201909 | Draft::Draft202012 | Draft::Unknown, "if")
            if ctx.has_vocabulary(&Vocabulary::Applicator) =>
        {
            Some((BuiltinKeyword::If.into(), if_::compile))
        }
        // Draft 2019-09 specific
        (Draft::Draft201909, "$recursiveRef") => Some((
            BuiltinKeyword::RecursiveRef.into(),
            ref_::compile_recursive_ref,
        )),
        (Draft::Draft201909 | Draft::Draft202012 | Draft::Unknown, "dependentRequired")
            if ctx.has_vocabulary(&Vocabulary::Validation) =>
        {
            Some((
                BuiltinKeyword::DependentRequired.into(),
                dependencies::compile_dependent_required,
            ))
        }
        (Draft::Draft201909 | Draft::Draft202012 | Draft::Unknown, "dependentSchemas")
            if ctx.has_vocabulary(&Vocabulary::Applicator) =>
        {
            Some((
                BuiltinKeyword::DependentSchemas.into(),
                dependencies::compile_dependent_schemas,
            ))
        }
        (Draft::Draft201909, "unevaluatedItems") if ctx.has_vocabulary(&Vocabulary::Applicator) => {
            Some((
                BuiltinKeyword::UnevaluatedItems.into(),
                unevaluated_items::compile,
            ))
        }
        (Draft::Draft202012 | Draft::Unknown, "unevaluatedItems")
            if ctx.has_vocabulary(&Vocabulary::Unevaluated) =>
        {
            Some((
                BuiltinKeyword::UnevaluatedItems.into(),
                unevaluated_items::compile,
            ))
        }
        (Draft::Draft201909, "unevaluatedProperties")
            if ctx.has_vocabulary(&Vocabulary::Applicator) =>
        {
            Some((
                BuiltinKeyword::UnevaluatedProperties.into(),
                unevaluated_properties::compile,
            ))
        }
        (Draft::Draft202012 | Draft::Unknown, "unevaluatedProperties")
            if ctx.has_vocabulary(&Vocabulary::Unevaluated) =>
        {
            Some((
                BuiltinKeyword::UnevaluatedProperties.into(),
                unevaluated_properties::compile,
            ))
        }
        // Draft 2020-12 specific
        (Draft::Draft202012 | Draft::Unknown, "prefixItems")
            if ctx.has_vocabulary(&Vocabulary::Applicator) =>
        {
            Some((BuiltinKeyword::PrefixItems.into(), prefix_items::compile))
        }
        (Draft::Draft202012 | Draft::Unknown, "$dynamicRef") => {
            Some((BuiltinKeyword::DynamicRef.into(), ref_::compile_dynamic_ref))
        }
        // Unknown or not-yet-implemented keyword
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};
    use test_case::test_case;

    #[test_case(&json!({"prefixItems": [{}], "items": {"type": "integer"}}), &json!([ null, 2, 3, "foo" ]), r#""foo" is not of type "integer""#)]
    #[test_case(&json!({"prefixItems": [{}, {}, {}], "items": false}), &json!([ 1, 2, 3, 4 ]), r"False schema does not allow 4")]
    #[test_case(&json!({"prefixItems": [{}, {}, {}], "items": false}), &json!([ 1, 2, 3, 4, 5 ]), r"False schema does not allow 4")]
    #[test_case(&json!({"properties": {"foo": {}, "bar": {}}, "patternProperties": { "^v": {} }, "additionalProperties": false}), &json!({"foo" : 1, "bar" : 2, "quux" : "boom"}), r"Additional properties are not allowed ('quux' was unexpected)")]
    #[test_case(&json!({"anyOf": [{"type": "integer"}, {"minimum": 2}]}), &json!(1.5), r"1.5 is not valid under any of the schemas listed in the 'anyOf' keyword")]
    #[test_case(&json!({"const": 2}), &json!(5), r"2 was expected")]
    #[test_case(&json!({"contains": {"minimum": 5}}), &json!([2, 3, 4]), r"None of [2,3,4] are valid under the given schema")]
    #[test_case(&json!({"enum": [1]}), &json!(4), r"4 is not one of 1")]
    #[test_case(&json!({"enum": [1, 2]}), &json!(4), r"4 is not one of 1 or 2")]
    #[test_case(&json!({"enum": [1, 2, 3]}), &json!(4), r"4 is not one of 1, 2 or 3")]
    #[test_case(&json!({"enum": [1, 2, 3, 4]}), &json!(5), r"5 is not one of 1, 2 or 2 other candidates")]
    #[test_case(&json!({"enum": [1, 2, 3, 4, 5]}), &json!(6), r"6 is not one of 1, 2 or 3 other candidates")]
    #[test_case(&json!({"exclusiveMaximum": 3}), &json!(3.0), r"3.0 is greater than or equal to the maximum of 3")]
    #[test_case(&json!({"exclusiveMaximum": 3.0}), &json!(3.0), r"3.0 is greater than or equal to the maximum of 3.0")]
    #[test_case(&json!({"exclusiveMinimum": 1}), &json!(1.0), r"1.0 is less than or equal to the minimum of 1")]
    #[test_case(&json!({"exclusiveMinimum": 1.0}), &json!(1), r"1 is less than or equal to the minimum of 1.0")]
    #[test_case(&json!({"format": "ipv4"}), &json!("2001:0db8:85a3:0000:0000:8a2e:0370:7334"), r#""2001:0db8:85a3:0000:0000:8a2e:0370:7334" is not a "ipv4""#)]
    #[test_case(&json!({"maximum": 3}), &json!(3.5), r"3.5 is greater than the maximum of 3")]
    #[test_case(&json!({"maximum": 3.0}), &json!(3.5), r"3.5 is greater than the maximum of 3.0")]
    #[test_case(&json!({"minimum": 3}), &json!(2.5), r"2.5 is less than the minimum of 3")]
    #[test_case(&json!({"minimum": 3.0}), &json!(2.5), r"2.5 is less than the minimum of 3.0")]
    #[test_case(&json!({"maxItems": 2}), &json!([1, 2, 3]), r"[1,2,3] has more than 2 items")]
    #[test_case(&json!({"maxLength": 2}), &json!("foo"), r#""foo" is longer than 2 characters"#)]
    #[test_case(&json!({"maxProperties": 2}), &json!({"foo": 1, "bar": 2, "baz": 3}), r#"{"bar":2,"baz":3,"foo":1} has more than 2 properties"#)]
    #[test_case(&json!({"minimum": 1.1}), &json!(0.6), r"0.6 is less than the minimum of 1.1")]
    #[test_case(&json!({"minItems": 1}), &json!([]), r"[] has less than 1 item")]
    #[test_case(&json!({"minLength": 2}), &json!("f"), r#""f" is shorter than 2 characters"#)]
    #[test_case(&json!({"minProperties": 1}), &json!({}), r"{} has less than 1 property")]
    #[test_case(&json!({"multipleOf": 2}), &json!(7), r"7 is not a multiple of 2")]
    #[test_case(&json!({"not": {"type": "integer"}}), &json!(1), r#"{"type":"integer"} is not allowed for 1"#)]
    #[test_case(&json!({"oneOf": [{"type": "integer"}, {"minimum": 2}]}), &json!(1.1), r"1.1 is not valid under any of the schemas listed in the 'oneOf' keyword")]
    #[test_case(&json!({"oneOf": [{"type": "integer"}, {"minimum": 2}]}), &json!(3), r"3 is valid under more than one of the schemas listed in the 'oneOf' keyword")]
    #[test_case(&json!({"pattern": "^a*$"}), &json!("abc"), r#""abc" does not match "^a*$""#)]
    #[test_case(&json!({"properties": {"foo": {}, "bar": {}}, "required": ["foo"]}), &json!({"bar": 1}), r#""foo" is a required property"#)]
    #[test_case(&json!({"type": "integer"}), &json!(1.1), r#"1.1 is not of type "integer""#)]
    #[test_case(&json!({"type": ["integer", "string"]}), &json!(null), r#"null is not of types "integer", "string""#)]
    #[test_case(&json!({"uniqueItems": true}), &json!([1, 1]), r"[1,1] has non-unique elements")]
    fn error_message(schema: &Value, instance: &Value, expected: &str) {
        let validator = crate::options()
            .should_validate_formats(true)
            .build(schema)
            .expect("Invalid schema");
        let errors: Vec<_> = validator.iter_errors(instance).collect();
        assert_eq!(errors[0].to_string(), expected);
    }

    // Extra cases not covered by JSON test suite
    #[test_case(&json!({"additionalProperties": {"type": "string"}}))]
    #[test_case(&json!({"additionalProperties": {"type": "string"}, "properties": {"foo": {}}}))]
    #[test_case(&json!({"additionalProperties": {"type": "string"}, "patternProperties": {"f.*o": {"type": "integer"}}}))]
    #[test_case(&json!({"additionalProperties": {"type": "string"}, "properties": {"foo": {}}, "patternProperties": {"f.*o": {"type": "integer"}}}))]
    #[test_case(&json!({"additionalProperties": false}))]
    #[test_case(&json!({"additionalProperties": false, "properties": {"foo": {}}}))]
    #[test_case(&json!({"additionalProperties": false, "patternProperties": {"f.*o": {"type": "integer"}}}))]
    #[test_case(&json!({"additionalProperties": false, "properties": {"foo": {}}, "patternProperties": {"f.*o": {"type": "integer"}}}))]
    #[test_case(&json!({"additionalItems": false, "prefixItems": [{"type": "string"}]}))]
    #[test_case(&json!({"additionalItems": {"type": "integer"}, "prefixItems": [{"type": "string"}]}))]
    #[test_case(&json!({"contains": {"minimum": 5}}))]
    #[test_case(&json!({"contentMediaType": "application/json"}))]
    #[test_case(&json!({"contentEncoding": "base64"}))]
    #[test_case(&json!({"contentEncoding": "base64", "contentMediaType": "application/json"}))]
    #[test_case(&json!({"dependencies": {"bar": ["foo"]}}))]
    #[test_case(&json!({"exclusiveMaximum": 5}))]
    #[test_case(&json!({"exclusiveMinimum": 5}))]
    #[test_case(&json!({"format": "ipv4"}))]
    #[test_case(&json!({"maximum": 2}))]
    #[test_case(&json!({"maxItems": 2}))]
    #[test_case(&json!({"maxProperties": 2}))]
    #[test_case(&json!({"minProperties": 2}))]
    #[test_case(&json!({"multipleOf": 2.5}))]
    #[test_case(&json!({"multipleOf": 2}))]
    #[test_case(&json!({"required": ["a"]}))]
    #[test_case(&json!({"pattern": "^a"}))]
    #[test_case(&json!({"patternProperties": {"f.*o": {"type": "integer"}}}))]
    #[test_case(&json!({"propertyNames": {"maxLength": 3}}))]
    fn is_valid_another_type(schema: &Value) {
        let instance = json!(null);
        assert!(crate::options()
            .should_validate_formats(true)
            .build(schema)
            .expect("Invalid schema")
            .is_valid(&instance));
    }
    #[test_case(&json!({"additionalProperties": false}), &json!({}))]
    #[test_case(&json!({"additionalItems": false, "items": true}), &json!([]))]
    fn is_valid(schema: &Value, instance: &Value) {
        assert!(crate::is_valid(schema, instance));
    }

    #[test_case(&json!({"type": "number"}), &json!(42))]
    #[test_case(&json!({"type": ["number", "null"]}), &json!(42))]
    fn integer_is_valid_number_multi_type(schema: &Value, instance: &Value) {
        // See: GH-147
        assert!(crate::is_valid(schema, instance));
    }
    // enum: Number
    #[test_case(&json!({"enum": [0.0]}), &json!(0))]
    // enum: Array
    #[test_case(&json!({"enum": [[1.0]]}), &json!([1]))]
    // enum: Object
    #[test_case(&json!({"enum": [{"a": 1.0}]}), &json!({"a": 1}))]
    // enum:: Object in Array
    #[test_case(&json!({"enum": [[{"b": 1.0}]]}), &json!([{"b": 1}]))]
    // enum:: Array in Object
    #[test_case(&json!({"enum": [{"c": [1.0]}]}), &json!({"c": [1]}))]
    // const: Number
    #[test_case(&json!({"const": 0.0}), &json!(0))]
    // const: Array
    #[test_case(&json!({"const": [1.0]}), &json!([1]))]
    // const: Object
    #[test_case(&json!({"const": {"a": 1.0}}), &json!({"a": 1}))]
    // const:: Object in Array
    #[test_case(&json!({"const": [{"b": 1.0}]}), &json!([{"b": 1}]))]
    // const:: Array in Object
    #[test_case(&json!({"const": {"c": [1.0]}}), &json!({"c": [1]}))]
    fn numeric_equivalence(schema: &Value, instance: &Value) {
        // See: GH-149
        assert!(crate::is_valid(schema, instance));
    }

    #[test]
    fn required_all_properties() {
        // See: GH-190
        let schema = json!({"required": ["foo", "bar"]});
        let instance = json!({});
        let validator = crate::validator_for(&schema).unwrap();
        let errors: Vec<_> = validator.iter_errors(&instance).collect();
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].to_string(), r#""foo" is a required property"#);
        assert_eq!(errors[1].to_string(), r#""bar" is a required property"#);
    }
}
