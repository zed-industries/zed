//! Character inclusion in binary or General_Category value Unicode sets.
//!
//! We rely on dead code elimination to remove the tables that aren't needed.

#![allow(bad_style)]
#![allow(clippy::all)]

use alloc::boxed::Box;

macro_rules! property_functions {
    ($module:ident, $property_names:ident, [$(
        $prop:ident,
    )*]) => {
        #[allow(unused)]
        mod $module;
        // unicode::ALPHABETIC('a')
        $(pub fn $prop(c: char) -> bool {
            self::$module::$prop.contains_char(c)
        })*

        pub static $property_names: &[&str] = &[
            $(stringify!($prop),)*
        ];
    };
}

macro_rules! char_property_functions {
    // For define custom property names
    {$(
        mod $module:ident;
        static $property_names:ident = [$(
            $prop:ident,
        )*];
    )*} => {$(
        property_functions!($module, $property_names, [$(
            $prop,
        )*]);
    )*};
    // For define property by copy BY_NAME values from `ucd-generate` generated.
    {$(
        mod $module:ident;
        static $property_names:ident = [$(
            ($_name:tt, $prop:ident),
        )*];
    )*} => {$(
        property_functions!($module, $property_names, [$(
            $prop,
        )*]);
    )*};
}

char_property_functions! {
    mod binary;
    static BINARY_PROPERTY_NAMES = [
        // ASCII_HEX_DIGIT, // let this one be stripped out -- the full trie is wasteful for ASCII
        ALPHABETIC, BIDI_CONTROL, CASE_IGNORABLE, CASED, CHANGES_WHEN_CASEFOLDED,
        CHANGES_WHEN_CASEMAPPED, CHANGES_WHEN_LOWERCASED, CHANGES_WHEN_TITLECASED,
        CHANGES_WHEN_UPPERCASED, DASH, DEFAULT_IGNORABLE_CODE_POINT, DEPRECATED, DIACRITIC,
        EMOJI, EMOJI_COMPONENT, EMOJI_MODIFIER, EMOJI_MODIFIER_BASE, EMOJI_PRESENTATION, EXTENDED_PICTOGRAPHIC,
        EXTENDER, GRAPHEME_BASE, GRAPHEME_EXTEND, GRAPHEME_LINK, HEX_DIGIT, HYPHEN,
        IDS_BINARY_OPERATOR, IDS_TRINARY_OPERATOR, ID_CONTINUE, ID_START, IDEOGRAPHIC, JOIN_CONTROL,
        LOGICAL_ORDER_EXCEPTION, LOWERCASE, MATH, NONCHARACTER_CODE_POINT, OTHER_ALPHABETIC,
        OTHER_DEFAULT_IGNORABLE_CODE_POINT, OTHER_GRAPHEME_EXTEND, OTHER_ID_CONTINUE,
        OTHER_ID_START, OTHER_LOWERCASE, OTHER_MATH, OTHER_UPPERCASE, PATTERN_SYNTAX,
        PATTERN_WHITE_SPACE, PREPENDED_CONCATENATION_MARK, QUOTATION_MARK, RADICAL,
        REGIONAL_INDICATOR, SENTENCE_TERMINAL, SOFT_DOTTED, TERMINAL_PUNCTUATION, UNIFIED_IDEOGRAPH,
        UPPERCASE, VARIATION_SELECTOR, WHITE_SPACE, XID_CONTINUE, XID_START,
    ];
}

char_property_functions! {
    mod category;
    // Copy from category::BY_NAME
    static CATEGORY_PROPERTY_NAMES = [
        ("Cased_Letter", CASED_LETTER), ("Close_Punctuation", CLOSE_PUNCTUATION),
        ("Connector_Punctuation", CONNECTOR_PUNCTUATION), ("Control", CONTROL),
        ("Currency_Symbol", CURRENCY_SYMBOL),
        ("Dash_Punctuation", DASH_PUNCTUATION), ("Decimal_Number", DECIMAL_NUMBER),
        ("Enclosing_Mark", ENCLOSING_MARK),
        ("Final_Punctuation", FINAL_PUNCTUATION), ("Format", FORMAT),
        ("Initial_Punctuation", INITIAL_PUNCTUATION), ("Letter", LETTER),
        ("Letter_Number", LETTER_NUMBER), ("Line_Separator", LINE_SEPARATOR),
        ("Lowercase_Letter", LOWERCASE_LETTER), ("Mark", MARK),
        ("Math_Symbol", MATH_SYMBOL), ("Modifier_Letter", MODIFIER_LETTER),
        ("Modifier_Symbol", MODIFIER_SYMBOL), ("Nonspacing_Mark", NONSPACING_MARK),
        ("Number", NUMBER), ("Open_Punctuation", OPEN_PUNCTUATION),
        ("Other", OTHER), ("Other_Letter", OTHER_LETTER),
        ("Other_Number", OTHER_NUMBER), ("Other_Punctuation", OTHER_PUNCTUATION),
        ("Other_Symbol", OTHER_SYMBOL),
        ("Paragraph_Separator", PARAGRAPH_SEPARATOR), ("Private_Use", PRIVATE_USE),
        ("Punctuation", PUNCTUATION), ("Separator", SEPARATOR),
        ("Space_Separator", SPACE_SEPARATOR), ("Spacing_Mark", SPACING_MARK),
        ("Surrogate", SURROGATE), ("Symbol", SYMBOL),
        ("Titlecase_Letter", TITLECASE_LETTER), ("Unassigned", UNASSIGNED),
        ("Uppercase_Letter", UPPERCASE_LETTER),
    ];

    mod script;
    // Copy from script::BY_NAME
    static SCRIPT_PROPERTY_NAMES = [
        ("Adlam", ADLAM),
        ("Ahom", AHOM),
        ("Anatolian_Hieroglyphs", ANATOLIAN_HIEROGLYPHS),
        ("Arabic", ARABIC),
        ("Armenian", ARMENIAN),
        ("Avestan", AVESTAN),
        ("Balinese", BALINESE),
        ("Bamum", BAMUM),
        ("Bassa_Vah", BASSA_VAH),
        ("Batak", BATAK),
        ("Bengali", BENGALI),
        ("Bhaiksuki", BHAIKSUKI),
        ("Bopomofo", BOPOMOFO),
        ("Brahmi", BRAHMI),
        ("Braille", BRAILLE),
        ("Buginese", BUGINESE),
        ("Buhid", BUHID),
        ("Canadian_Aboriginal", CANADIAN_ABORIGINAL),
        ("Carian", CARIAN),
        ("Caucasian_Albanian", CAUCASIAN_ALBANIAN),
        ("Chakma", CHAKMA),
        ("Cham", CHAM),
        ("Cherokee", CHEROKEE),
        ("Chorasmian", CHORASMIAN),
        ("Common", COMMON),
        ("Coptic", COPTIC),
        ("Cuneiform", CUNEIFORM),
        ("Cypriot", CYPRIOT),
        ("Cypro_Minoan", CYPRO_MINOAN),
        ("Cyrillic", CYRILLIC),
        ("Deseret", DESERET),
        ("Devanagari", DEVANAGARI),
        ("Dives_Akuru", DIVES_AKURU),
        ("Dogra", DOGRA),
        ("Duployan", DUPLOYAN),
        ("Egyptian_Hieroglyphs", EGYPTIAN_HIEROGLYPHS),
        ("Elbasan", ELBASAN),
        ("Elymaic", ELYMAIC),
        ("Ethiopic", ETHIOPIC),
        ("Georgian", GEORGIAN),
        ("Glagolitic", GLAGOLITIC),
        ("Gothic", GOTHIC),
        ("Grantha", GRANTHA),
        ("Greek", GREEK),
        ("Gujarati", GUJARATI),
        ("Gunjala_Gondi", GUNJALA_GONDI),
        ("Gurmukhi", GURMUKHI),
        ("Han", HAN),
        ("Hangul", HANGUL),
        ("Hanifi_Rohingya", HANIFI_ROHINGYA),
        ("Hanunoo", HANUNOO),
        ("Hatran", HATRAN),
        ("Hebrew", HEBREW),
        ("Hiragana", HIRAGANA),
        ("Imperial_Aramaic", IMPERIAL_ARAMAIC),
        ("Inherited", INHERITED),
        ("Inscriptional_Pahlavi", INSCRIPTIONAL_PAHLAVI),
        ("Inscriptional_Parthian", INSCRIPTIONAL_PARTHIAN),
        ("Javanese", JAVANESE),
        ("Kaithi", KAITHI),
        ("Kannada", KANNADA),
        ("Katakana", KATAKANA),
        ("Kawi", KAWI),
        ("Kayah_Li", KAYAH_LI),
        ("Kharoshthi", KHAROSHTHI),
        ("Khitan_Small_Script", KHITAN_SMALL_SCRIPT),
        ("Khmer", KHMER),
        ("Khojki", KHOJKI),
        ("Khudawadi", KHUDAWADI),
        ("Lao", LAO),
        ("Latin", LATIN),
        ("Lepcha", LEPCHA),
        ("Limbu", LIMBU),
        ("Linear_A", LINEAR_A),
        ("Linear_B", LINEAR_B),
        ("Lisu", LISU),
        ("Lycian", LYCIAN),
        ("Lydian", LYDIAN),
        ("Mahajani", MAHAJANI),
        ("Makasar", MAKASAR),
        ("Malayalam", MALAYALAM),
        ("Mandaic", MANDAIC),
        ("Manichaean", MANICHAEAN),
        ("Marchen", MARCHEN),
        ("Masaram_Gondi", MASARAM_GONDI),
        ("Medefaidrin", MEDEFAIDRIN),
        ("Meetei_Mayek", MEETEI_MAYEK),
        ("Mende_Kikakui", MENDE_KIKAKUI),
        ("Meroitic_Cursive", MEROITIC_CURSIVE),
        ("Meroitic_Hieroglyphs", MEROITIC_HIEROGLYPHS),
        ("Miao", MIAO),
        ("Modi", MODI),
        ("Mongolian", MONGOLIAN),
        ("Mro", MRO),
        ("Multani", MULTANI),
        ("Myanmar", MYANMAR),
        ("Nabataean", NABATAEAN),
        ("Nag_Mundari", NAG_MUNDARI),
        ("Nandinagari", NANDINAGARI),
        ("New_Tai_Lue", NEW_TAI_LUE),
        ("Newa", NEWA),
        ("Nko", NKO),
        ("Nushu", NUSHU),
        ("Nyiakeng_Puachue_Hmong", NYIAKENG_PUACHUE_HMONG),
        ("Ogham", OGHAM),
        ("Ol_Chiki", OL_CHIKI),
        ("Old_Hungarian", OLD_HUNGARIAN),
        ("Old_Italic", OLD_ITALIC),
        ("Old_North_Arabian", OLD_NORTH_ARABIAN),
        ("Old_Permic", OLD_PERMIC),
        ("Old_Persian", OLD_PERSIAN),
        ("Old_Sogdian", OLD_SOGDIAN),
        ("Old_South_Arabian", OLD_SOUTH_ARABIAN),
        ("Old_Turkic", OLD_TURKIC),
        ("Old_Uyghur", OLD_UYGHUR),
        ("Oriya", ORIYA),
        ("Osage", OSAGE),
        ("Osmanya", OSMANYA),
        ("Pahawh_Hmong", PAHAWH_HMONG),
        ("Palmyrene", PALMYRENE),
        ("Pau_Cin_Hau", PAU_CIN_HAU),
        ("Phags_Pa", PHAGS_PA),
        ("Phoenician", PHOENICIAN),
        ("Psalter_Pahlavi", PSALTER_PAHLAVI),
        ("Rejang", REJANG),
        ("Runic", RUNIC),
        ("Samaritan", SAMARITAN),
        ("Saurashtra", SAURASHTRA),
        ("Sharada", SHARADA),
        ("Shavian", SHAVIAN),
        ("Siddham", SIDDHAM),
        ("SignWriting", SIGNWRITING),
        ("Sinhala", SINHALA),
        ("Sogdian", SOGDIAN),
        ("Sora_Sompeng", SORA_SOMPENG),
        ("Soyombo", SOYOMBO),
        ("Sundanese", SUNDANESE),
        ("Syloti_Nagri", SYLOTI_NAGRI),
        ("Syriac", SYRIAC),
        ("Tagalog", TAGALOG),
        ("Tagbanwa", TAGBANWA),
        ("Tai_Le", TAI_LE),
        ("Tai_Tham", TAI_THAM),
        ("Tai_Viet", TAI_VIET),
        ("Takri", TAKRI),
        ("Tamil", TAMIL),
        ("Tangsa", TANGSA),
        ("Tangut", TANGUT),
        ("Telugu", TELUGU),
        ("Thaana", THAANA),
        ("Thai", THAI),
        ("Tibetan", TIBETAN),
        ("Tifinagh", TIFINAGH),
        ("Tirhuta", TIRHUTA),
        ("Toto", TOTO),
        ("Ugaritic", UGARITIC),
        ("Vai", VAI),
        ("Vithkuqi", VITHKUQI),
        ("Wancho", WANCHO),
        ("Warang_Citi", WARANG_CITI),
        ("Yezidi", YEZIDI),
        ("Yi", YI),
        ("Zanabazar_Square", ZANABAZAR_SQUARE),
    ];
}

/// Return all available unicode property names
pub fn unicode_property_names() -> Box<dyn Iterator<Item = &'static str>> {
    Box::new(
        BINARY_PROPERTY_NAMES
            .iter()
            .map(|name| *name)
            .chain(CATEGORY_PROPERTY_NAMES.iter().map(|name| *name))
            .chain(SCRIPT_PROPERTY_NAMES.iter().map(|name| *name)),
    )
}

pub fn by_name(name: &str) -> Option<Box<dyn Fn(char) -> bool>> {
    for property in binary::BY_NAME {
        if name == property.0.to_uppercase() {
            return Some(Box::new(move |c| property.1.contains_char(c)));
        }
    }

    for property in category::BY_NAME {
        if name == property.0.to_uppercase() {
            return Some(Box::new(move |c| property.1.contains_char(c)));
        }
    }

    for property in script::BY_NAME {
        if name == property.0.to_uppercase() {
            return Some(Box::new(move |c| property.1.contains_char(c)));
        }
    }

    None
}
