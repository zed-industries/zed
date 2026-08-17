#![cfg(feature = "general-category")]

#[test]
fn general_category_test() {
    use std::ops::Not;
    use unicode_properties::UnicodeGeneralCategory;
    use unicode_properties::{GeneralCategory, GeneralCategoryGroup};
    assert_eq!('A'.general_category(), GeneralCategory::UppercaseLetter);
    assert_eq!('A'.general_category_group(), GeneralCategoryGroup::Letter);
    assert!('A'.is_letter_cased());
    assert_eq!(' '.general_category(), GeneralCategory::SpaceSeparator);
    assert_eq!(
        ' '.general_category_group(),
        GeneralCategoryGroup::Separator
    );
    assert!(' '.is_letter_cased().not());
    assert_eq!('一'.general_category(), GeneralCategory::OtherLetter);
    assert_eq!('一'.general_category_group(), GeneralCategoryGroup::Letter);
    assert!('一'.is_letter_cased().not());
    assert_eq!('🦀'.general_category(), GeneralCategory::OtherSymbol);
    assert_eq!('🦀'.general_category_group(), GeneralCategoryGroup::Symbol);
    assert!('🦀'.is_letter_cased().not());
}
