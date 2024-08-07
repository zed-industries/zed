use std::collections::HashSet;
use std::sync::LazyLock;

/// Returns whether the given country code is supported by Google Gemini.
///
/// https://ai.google.dev/gemini-api/docs/available-regions
pub fn is_supported_country(country_code: &str) -> bool {
    SUPPORTED_COUNTRIES.contains(&country_code)
}

/// The list of country codes supported by Google Gemini.
///
/// https://ai.google.dev/gemini-api/docs/available-regions
static SUPPORTED_COUNTRIES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    vec![
        "DZ", // Algeria
        "AS", // American Samoa
        "AO", // Angola
        "AI", // Anguilla
        "AQ", // Antarctica
        "AG", // Antigua and Barbuda
        "AR", // Argentina
        "AM", // Armenia
        "AW", // Aruba
        "AU", // Australia
        "AT", // Austria
        "AZ", // Azerbaijan
        "BS", // The Bahamas
        "BH", // Bahrain
        "BD", // Bangladesh
        "BB", // Barbados
        "BE", // Belgium
        "BZ", // Belize
        "BJ", // Benin
        "BM", // Bermuda
        "BT", // Bhutan
        "BO", // Bolivia
        "BW", // Botswana
        "BR", // Brazil
        "IO", // British Indian Ocean Territory
        "VG", // British Virgin Islands
        "BN", // Brunei
        "BG", // Bulgaria
        "BF", // Burkina Faso
        "BI", // Burundi
        "CV", // Cabo Verde
        "KH", // Cambodia
        "CM", // Cameroon
        "CA", // Canada
        "BQ", // Caribbean Netherlands
        "KY", // Cayman Islands
        "CF", // Central African Republic
        "TD", // Chad
        "CL", // Chile
        "CX", // Christmas Island
        "CC", // Cocos (Keeling) Islands
        "CO", // Colombia
        "KM", // Comoros
        "CK", // Cook Islands
        "CI", // Côte d'Ivoire
        "CR", // Costa Rica
        "HR", // Croatia
        "CW", // Curaçao
        "CZ", // Czech Republic
        "CD", // Democratic Republic of the Congo
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
        "SZ", // Eswatini
        "ET", // Ethiopia
        "FK", // Falkland Islands (Islas Malvinas)
        "FJ", // Fiji
        "FI", // Finland
        "FR", // France
        "GA", // Gabon
        "GM", // The Gambia
        "GE", // Georgia
        "DE", // Germany
        "GH", // Ghana
        "GI", // Gibraltar
        "GR", // Greece
        "GD", // Grenada
        "GU", // Guam
        "GT", // Guatemala
        "GG", // Guernsey
        "GN", // Guinea
        "GW", // Guinea-Bissau
        "GY", // Guyana
        "HT", // Haiti
        "HM", // Heard Island and McDonald Islands
        "HN", // Honduras
        "HU", // Hungary
        "IS", // Iceland
        "IN", // India
        "ID", // Indonesia
        "IQ", // Iraq
        "IE", // Ireland
        "IM", // Isle of Man
        "IL", // Israel
        "IT", // Italy
        "JM", // Jamaica
        "JP", // Japan
        "JE", // Jersey
        "JO", // Jordan
        "KZ", // Kazakhstan
        "KE", // Kenya
        "KI", // Kiribati
        "KG", // Kyrgyzstan
        "KW", // Kuwait
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
        "MN", // Mongolia
        "MS", // Montserrat
        "MA", // Morocco
        "MZ", // Mozambique
        "NA", // Namibia
        "NR", // Nauru
        "NP", // Nepal
        "NL", // Netherlands
        "NC", // New Caledonia
        "NZ", // New Zealand
        "NI", // Nicaragua
        "NE", // Niger
        "NG", // Nigeria
        "NU", // Niue
        "NF", // Norfolk Island
        "MP", // Northern Mariana Islands
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
        "PN", // Pitcairn Islands
        "PL", // Poland
        "PT", // Portugal
        "PR", // Puerto Rico
        "QA", // Qatar
        "CY", // Republic of Cyprus
        "CG", // Republic of the Congo
        "RO", // Romania
        "RW", // Rwanda
        "BL", // Saint Barthélemy
        "KN", // Saint Kitts and Nevis
        "LC", // Saint Lucia
        "PM", // Saint Pierre and Miquelon
        "VC", // Saint Vincent and the Grenadines
        "SH", // Saint Helena, Ascension and Tristan da Cunha
        "WS", // Samoa
        "ST", // São Tomé and Príncipe
        "SA", // Saudi Arabia
        "SN", // Senegal
        "SC", // Seychelles
        "SL", // Sierra Leone
        "SG", // Singapore
        "SK", // Slovakia
        "SI", // Slovenia
        "SB", // Solomon Islands
        "SO", // Somalia
        "ZA", // South Africa
        "GS", // South Georgia and the South Sandwich Islands
        "KR", // South Korea
        "SS", // South Sudan
        "ES", // Spain
        "LK", // Sri Lanka
        "SD", // Sudan
        "SR", // Suriname
        "SE", // Sweden
        "CH", // Switzerland
        "TW", // Taiwan
        "TJ", // Tajikistan
        "TZ", // Tanzania
        "TH", // Thailand
        "TL", // Timor-Leste
        "TG", // Togo
        "TK", // Tokelau
        "TO", // Tonga
        "TT", // Trinidad and Tobago
        "TN", // Tunisia
        "TR", // Türkiye
        "TM", // Turkmenistan
        "TC", // Turks and Caicos Islands
        "TV", // Tuvalu
        "UG", // Uganda
        "GB", // United Kingdom
        "AE", // United Arab Emirates
        "US", // United States
        "UM", // United States Minor Outlying Islands
        "VI", // U.S. Virgin Islands
        "UY", // Uruguay
        "UZ", // Uzbekistan
        "VU", // Vanuatu
        "VE", // Venezuela
        "VN", // Vietnam
        "WF", // Wallis and Futuna
        "EH", // Western Sahara
        "YE", // Yemen
        "ZM", // Zambia
        "ZW", // Zimbabwe
    ]
    .into_iter()
    .collect()
});
