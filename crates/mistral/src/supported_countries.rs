use std::collections::HashSet;
use std::sync::LazyLock;

pub fn is_supported_country(country_code: &str) -> bool {
    SUPPORTED_COUNTRIES.contains(&country_code)
}

static SUPPORTED_COUNTRIES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    vec![
        "SE", // Sweden
        "FR", // France
    ]
    .into_iter()
    .collect()
});
