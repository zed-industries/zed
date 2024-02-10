use language::LanguageConfig;
use schemars::schema_for;
use theme::ThemeFamilyContent;

fn main() {
    let theme_family_schema = schema_for!(ThemeFamilyContent);
    let language_config_schema = schema_for!(LanguageConfig);

    println!(
        "{}",
        serde_json::to_string_pretty(&theme_family_schema).unwrap()
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&language_config_schema).unwrap()
    );
}
