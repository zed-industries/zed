use sea_orm::{entity::prelude::StringLen, ActiveEnum};
use sea_orm_macros::{DeriveActiveEnum, EnumIter};

#[derive(Debug, EnumIter, DeriveActiveEnum, Eq, PartialEq)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "test_enum",
    rename_all = "camelCase"
)]
enum TestEnum {
    DefaultVariant,
    #[sea_orm(rename = "camelCase")]
    VariantCamelCase,
    #[sea_orm(rename = "kebab-case")]
    VariantKebabCase,
    #[sea_orm(rename = "mixed_case")]
    VariantMixedCase,
    #[sea_orm(rename = "SCREAMING_SNAKE_CASE")]
    VariantShoutySnakeCase,
    #[sea_orm(rename = "snake_case")]
    VariantSnakeCase,
    #[sea_orm(rename = "title_case")]
    VariantTitleCase,
    #[sea_orm(rename = "UPPERCASE")]
    VariantUpperCase,
    #[sea_orm(rename = "lowercase")]
    VariantLowerCase,
    #[sea_orm(rename = "SCREAMING-KEBAB-CASE")]
    VariantScreamingKebabCase,
    #[sea_orm(rename = "PascalCase")]
    VariantPascalCase,
    #[sea_orm(string_value = "CuStOmStRiNgVaLuE")]
    CustomStringValue,
}

#[derive(Debug, EnumIter, DeriveActiveEnum, Eq, PartialEq)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum TestEnum2 {
    HelloWorld,
    #[sea_orm(rename = "camelCase")]
    HelloWorldTwo,
}

#[derive(Debug, EnumIter, DeriveActiveEnum, Eq, PartialEq)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum TestEnum3 {
    HelloWorld,
}

#[test]
fn derive_active_enum_value() {
    assert_eq!(TestEnum::DefaultVariant.to_value(), "defaultVariant");
    assert_eq!(TestEnum::VariantCamelCase.to_value(), "variantCamelCase");
    assert_eq!(TestEnum::VariantKebabCase.to_value(), "variant-kebab-case");
    assert_eq!(TestEnum::VariantMixedCase.to_value(), "variantMixedCase");
    assert_eq!(
        TestEnum::VariantShoutySnakeCase.to_value(),
        "VARIANT_SHOUTY_SNAKE_CASE"
    );
    assert_eq!(TestEnum::VariantSnakeCase.to_value(), "variant_snake_case");
    assert_eq!(TestEnum::VariantTitleCase.to_value(), "Variant Title Case");
    assert_eq!(TestEnum::VariantUpperCase.to_value(), "VARIANTUPPERCASE");
    assert_eq!(TestEnum::VariantLowerCase.to_value(), "variantlowercase");
    assert_eq!(
        TestEnum::VariantScreamingKebabCase.to_value(),
        "VARIANT-SCREAMING-KEBAB-CASE"
    );
    assert_eq!(TestEnum::VariantPascalCase.to_value(), "VariantPascalCase");
    assert_eq!(TestEnum::CustomStringValue.to_value(), "CuStOmStRiNgVaLuE");
}

#[test]
fn derive_active_enum_from_value() {
    assert_eq!(
        TestEnum::try_from_value(&"defaultVariant".to_string()),
        Ok(TestEnum::DefaultVariant)
    );
    assert_eq!(
        TestEnum::try_from_value(&"variantCamelCase".to_string()),
        Ok(TestEnum::VariantCamelCase)
    );
    assert_eq!(
        TestEnum::try_from_value(&"variant-kebab-case".to_string()),
        Ok(TestEnum::VariantKebabCase)
    );
    assert_eq!(
        TestEnum::try_from_value(&"variantMixedCase".to_string()),
        Ok(TestEnum::VariantMixedCase)
    );
    assert_eq!(
        TestEnum::try_from_value(&"VARIANT_SHOUTY_SNAKE_CASE".to_string()),
        Ok(TestEnum::VariantShoutySnakeCase),
    );
    assert_eq!(
        TestEnum::try_from_value(&"variant_snake_case".to_string()),
        Ok(TestEnum::VariantSnakeCase)
    );
    assert_eq!(
        TestEnum::try_from_value(&"Variant Title Case".to_string()),
        Ok(TestEnum::VariantTitleCase)
    );
    assert_eq!(
        TestEnum::try_from_value(&"VARIANTUPPERCASE".to_string()),
        Ok(TestEnum::VariantUpperCase)
    );
    assert_eq!(
        TestEnum::try_from_value(&"variantlowercase".to_string()),
        Ok(TestEnum::VariantLowerCase)
    );
    assert_eq!(
        TestEnum::try_from_value(&"VARIANT-SCREAMING-KEBAB-CASE".to_string()),
        Ok(TestEnum::VariantScreamingKebabCase),
    );
    assert_eq!(
        TestEnum::try_from_value(&"VariantPascalCase".to_string()),
        Ok(TestEnum::VariantPascalCase)
    );
    assert_eq!(
        TestEnum::try_from_value(&"CuStOmStRiNgVaLuE".to_string()),
        Ok(TestEnum::CustomStringValue)
    );
}

#[test]
fn derive_active_enum_value_2() {
    assert_eq!(TestEnum2::HelloWorld.to_value(), "hello_world");
    assert_eq!(TestEnum2::HelloWorldTwo.to_value(), "helloWorldTwo");

    assert_eq!(TestEnum3::HelloWorld.to_value(), "hello_world");
}
