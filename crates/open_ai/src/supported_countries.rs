use std::collections::HashSet;
use std::sync::LazyLock;

/// Returns whether the given country code is supported by OpenAI.
///
/// https://platform.openai.com/docs/supported-countries
pub fn is_supported_country(country_code: &str) -> bool {
    SUPPORTED_COUNTRIES.contains(&country_code)
}

/// The list of country codes supported by OpenAI.
///
/// https://platform.openai.com/docs/supported-countries
static SUPPORTED_COUNTRIES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    vec![
        "AL", // Albania
        "DZ", // Algeria
        "AS", // American Samoa (US)
        "AI", // Anguilla (UK)
        "AF", // Afghanistan
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
        "BM", // Bermuda (UK)
        "BT", // Bhutan
        "BO", // Bolivia
        "BA", // Bosnia and Herzegovina
        "BW", // Botswana
        "BR", // Brazil
        "IO", // British Indian Ocean Territory (UK)
        "BN", // Brunei
        "BG", // Bulgaria
        "BF", // Burkina Faso
        "BI", // Burundi
        "CV", // Cabo Verde
        "KH", // Cambodia
        "CM", // Cameroon
        "CA", // Canada
        "KY", // Cayman Islands (UK)
        "CF", // Central African Republic
        "TD", // Chad
        "CL", // Chile
        "CX", // Christmas Island (AU)
        "CC", // Cocos (Keeling) Islands (AU)
        "CO", // Colombia
        "KM", // Comoros
        "CG", // Congo (Brazzaville)
        "CD", // Congo (DRC)
        "CK", // Cook Islands (NZ)
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
        "ER", // Eritrea
        "EE", // Estonia
        "SZ", // Eswatini (Swaziland)
        "ET", // Ethiopia
        "FK", // Falkland Islands (UK)
        "FJ", // Fiji
        "FI", // Finland
        "FR", // France
        "GF", // French Guiana (FR)
        "PF", // French Polynesia (FR)
        "TF", // French Southern Territories
        "GA", // Gabon
        "GM", // Gambia
        "GE", // Georgia
        "DE", // Germany
        "GH", // Ghana
        "GI", // Gibraltar (UK)
        "GR", // Greece
        "GD", // Grenada
        "GT", // Guatemala
        "GU", // Guam (US)
        "GN", // Guinea
        "GW", // Guinea-Bissau
        "GY", // Guyana
        "HT", // Haiti
        "HM", // Heard Island and McDonald Islands (AU)
        "VA", // Holy See (Vatican City)
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
        "LY", // Libya
        "LI", // Liechtenstein
        "LT", // Lithuania
        "LU", // Luxembourg
        "MG", // Madagascar
        "MW", // Malawi
        "MY", // Malaysia
        "MV", // Maldives
        "ML", // Mali
        "MT", // Malta
        "MH", // Marshall Islands
        "MR", // Mauritania
        "MU", // Mauritius
        "MX", // Mexico
        "FM", // Micronesia
        "MD", // Moldova
        "MC", // Monaco
        "MN", // Mongolia
        "MS", // Montserrat (UK)
        "ME", // Montenegro
        "MA", // Morocco
        "MZ", // Mozambique
        "MM", // Myanmar
        "NA", // Namibia
        "NR", // Nauru
        "NP", // Nepal
        "NL", // Netherlands
        "NZ", // New Zealand
        "NI", // Nicaragua
        "NE", // Niger
        "NG", // Nigeria
        "NF", // Norfolk Island (AU)
        "MK", // North Macedonia
        "MI", // Northern Mariana Islands (UK)
        "NO", // Norway
        "NU", // Niue (NZ)
        "OM", // Oman
        "PK", // Pakistan
        "PW", // Palau
        "PS", // Palestine
        "PA", // Panama
        "PG", // Papua New Guinea
        "PY", // Paraguay
        "PE", // Peru
        "PH", // Philippines
        "PN", // Pitcairn (UK)
        "PL", // Poland
        "PT", // Portugal
        "PR", // Puerto Rico (US)
        "QA", // Qatar
        "RO", // Romania
        "RW", // Rwanda
        "BL", // Saint Barthélemy (FR)
        "KN", // Saint Kitts and Nevis
        "LC", // Saint Lucia
        "MF", // Saint Martin (FR)
        "PM", // Saint Pierre and Miquelon (FR)
        "VC", // Saint Vincent and the Grenadines
        "WS", // Samoa
        "SM", // San Marino
        "ST", // Sao Tome and Principe
        "SA", // Saudi Arabia
        "SN", // Senegal
        "RS", // Serbia
        "SC", // Seychelles
        "SH", // Saint Helena, Ascension and Tristan da Cunha (UK)
        "SL", // Sierra Leone
        "SG", // Singapore
        "SK", // Slovakia
        "SI", // Slovenia
        "SB", // Solomon Islands
        "SO", // Somalia
        "ZA", // South Africa
        "KR", // South Korea
        "SS", // South Sudan
        "ES", // Spain
        "LK", // Sri Lanka
        "SR", // Suriname
        "SE", // Sweden
        "CH", // Switzerland
        "SD", // Sudan
        "TW", // Taiwan
        "TJ", // Tajikistan
        "TZ", // Tanzania
        "TH", // Thailand
        "TL", // Timor-Leste (East Timor)
        "TG", // Togo
        "TK", // Tokelau (NZ)
        "TO", // Tonga
        "TT", // Trinidad and Tobago
        "TN", // Tunisia
        "TR", // Turkey
        "TM", // Turkmenistan
        "TC", // Turks and Caicos Islands (UK)
        "TV", // Tuvalu
        "UG", // Uganda
        "UA", // Ukraine (with certain exceptions)
        "AE", // United Arab Emirates
        "GB", // United Kingdom
        "UM", // United States Minor Outlying Islands (US)
        "US", // United States of America
        "UY", // Uruguay
        "UZ", // Uzbekistan
        "VU", // Vanuatu
        "VN", // Vietnam
        "VI", // Virgin Islands (US)
        "VG", // Virgin Islands (UK)
        "WF", // Wallis and Futuna (FR)
        "YE", // Yemen
        "ZM", // Zambia
        "ZW", // Zimbabwe
    ]
    .into_iter()
    .collect()
});
