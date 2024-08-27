use std::collections::HashSet;
use std::sync::LazyLock;

/// Returns whether the given country code is supported by Anthropic.
///
/// https://www.anthropic.com/supported-countries
pub fn is_supported_country(country_code: &str) -> bool {
    SUPPORTED_COUNTRIES.contains(&country_code)
}

/// The list of country codes supported by Anthropic.
///
/// https://www.anthropic.com/supported-countries
static SUPPORTED_COUNTRIES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    vec![
        "AL", // Albania
        "DZ", // Algeria
        "AD", // Andorra
        "AO", // Angola
        "AG", // Antigua and Barbuda
        "AR", // Argentina
        "AM", // Armenia
        "AU", // Australia
        "AT", // Austria
        "AZ", // Azerbaijan
        "BS", // Bahamas
        "BH", // Bahrain
        "BD", // Bangladesh
        "BB", // Barbados
        "BE", // Belgium
        "BZ", // Belize
        "BJ", // Benin
        "BT", // Bhutan
        "BO", // Bolivia
        "BA", // Bosnia and Herzegovina
        "BW", // Botswana
        "BR", // Brazil
        "BN", // Brunei
        "BG", // Bulgaria
        "BF", // Burkina Faso
        "BI", // Burundi
        "CV", // Cabo Verde
        "KH", // Cambodia
        "CM", // Cameroon
        "CA", // Canada
        "TD", // Chad
        "CL", // Chile
        "CO", // Colombia
        "KM", // Comoros
        "CG", // Congo (Brazzaville)
        "CR", // Costa Rica
        "CI", // Côte d'Ivoire
        "HR", // Croatia
        "CY", // Cyprus
        "CZ", // Czechia (Czech Republic)
        "DK", // Denmark
        "DJ", // Djibouti
        "DM", // Dominica
        "DO", // Dominican Republic
        "EC", // Ecuador
        "EG", // Egypt
        "SV", // El Salvador
        "GQ", // Equatorial Guinea
        "EE", // Estonia
        "SZ", // Eswatini
        "FJ", // Fiji
        "FI", // Finland
        "FR", // France
        "GA", // Gabon
        "GM", // Gambia
        "GE", // Georgia
        "DE", // Germany
        "GH", // Ghana
        "GR", // Greece
        "GD", // Grenada
        "GT", // Guatemala
        "GN", // Guinea
        "GW", // Guinea-Bissau
        "GY", // Guyana
        "HT", // Haiti
        "HN", // Honduras
        "HU", // Hungary
        "IS", // Iceland
        "IN", // India
        "ID", // Indonesia
        "IQ", // Iraq
        "IE", // Ireland
        "IL", // Israel
        "IT", // Italy
        "JM", // Jamaica
        "JP", // Japan
        "JO", // Jordan
        "KZ", // Kazakhstan
        "KE", // Kenya
        "KI", // Kiribati
        "KW", // Kuwait
        "KG", // Kyrgyzstan
        "LA", // Laos
        "LV", // Latvia
        "LB", // Lebanon
        "LS", // Lesotho
        "LR", // Liberia
        "LI", // Liechtenstein
        "LT", // Lithuania
        "LU", // Luxembourg
        "MG", // Madagascar
        "MW", // Malawi
        "MY", // Malaysia
        "MV", // Maldives
        "MT", // Malta
        "MH", // Marshall Islands
        "MR", // Mauritania
        "MU", // Mauritius
        "MX", // Mexico
        "FM", // Micronesia
        "MD", // Moldova
        "MC", // Monaco
        "MN", // Mongolia
        "ME", // Montenegro
        "MA", // Morocco
        "MZ", // Mozambique
        "NA", // Namibia
        "NR", // Nauru
        "NP", // Nepal
        "NL", // Netherlands
        "NZ", // New Zealand
        "NE", // Niger
        "NG", // Nigeria
        "MK", // North Macedonia
        "NO", // Norway
        "OM", // Oman
        "PK", // Pakistan
        "PW", // Palau
        "PS", // Palestine
        "PA", // Panama
        "PG", // Papua New Guinea
        "PY", // Paraguay
        "PE", // Peru
        "PH", // Philippines
        "PL", // Poland
        "PT", // Portugal
        "QA", // Qatar
        "RO", // Romania
        "RW", // Rwanda
        "KN", // Saint Kitts and Nevis
        "LC", // Saint Lucia
        "VC", // Saint Vincent and the Grenadines
        "WS", // Samoa
        "SM", // San Marino
        "ST", // São Tomé and Príncipe
        "SA", // Saudi Arabia
        "SN", // Senegal
        "RS", // Serbia
        "SC", // Seychelles
        "SL", // Sierra Leone
        "SG", // Singapore
        "SK", // Slovakia
        "SI", // Slovenia
        "SB", // Solomon Islands
        "ZA", // South Africa
        "KR", // South Korea
        "ES", // Spain
        "LK", // Sri Lanka
        "SR", // Suriname
        "SE", // Sweden
        "CH", // Switzerland
        "TW", // Taiwan
        "TJ", // Tajikistan
        "TZ", // Tanzania
        "TH", // Thailand
        "TL", // Timor-Leste
        "TG", // Togo
        "TO", // Tonga
        "TT", // Trinidad and Tobago
        "TN", // Tunisia
        "TR", // Türkiye (Turkey)
        "TM", // Turkmenistan
        "TV", // Tuvalu
        "UG", // Uganda
        "UA", // Ukraine (except Crimea, Donetsk, and Luhansk regions)
        "AE", // United Arab Emirates
        "GB", // United Kingdom
        "US", // United States of America
        "UY", // Uruguay
        "UZ", // Uzbekistan
        "VU", // Vanuatu
        "VA", // Vatican City
        "VN", // Vietnam
        "ZM", // Zambia
        "ZW", // Zimbabwe
    ]
    .into_iter()
    .collect()
});
