#![allow(unused, non_upper_case_globals)]

use crate::FontFeatures;
use cocoa::appkit::CGFloat;
use core_foundation::{base::TCFType, number::CFNumber};
use core_graphics::geometry::CGAffineTransform;
use core_text::{
    font::{CTFont, CTFontRef},
    font_descriptor::{
        CTFontDescriptor, CTFontDescriptorCreateCopyWithFeature, CTFontDescriptorRef,
    },
};
use font_kit::font::Font;
use std::ptr;

const kCaseSensitiveLayoutOffSelector: i32 = 1;
const kCaseSensitiveLayoutOnSelector: i32 = 0;
const kCaseSensitiveLayoutType: i32 = 33;
const kCaseSensitiveSpacingOffSelector: i32 = 3;
const kCaseSensitiveSpacingOnSelector: i32 = 2;
const kCharacterAlternativesType: i32 = 17;
const kCommonLigaturesOffSelector: i32 = 3;
const kCommonLigaturesOnSelector: i32 = 2;
const kContextualAlternatesOffSelector: i32 = 1;
const kContextualAlternatesOnSelector: i32 = 0;
const kContextualAlternatesType: i32 = 36;
const kContextualLigaturesOffSelector: i32 = 19;
const kContextualLigaturesOnSelector: i32 = 18;
const kContextualSwashAlternatesOffSelector: i32 = 5;
const kContextualSwashAlternatesOnSelector: i32 = 4;
const kDefaultLowerCaseSelector: i32 = 0;
const kDefaultUpperCaseSelector: i32 = 0;
const kDiagonalFractionsSelector: i32 = 2;
const kFractionsType: i32 = 11;
const kHistoricalLigaturesOffSelector: i32 = 21;
const kHistoricalLigaturesOnSelector: i32 = 20;
const kHojoCharactersSelector: i32 = 12;
const kInferiorsSelector: i32 = 2;
const kJIS2004CharactersSelector: i32 = 11;
const kLigaturesType: i32 = 1;
const kLowerCasePetiteCapsSelector: i32 = 2;
const kLowerCaseSmallCapsSelector: i32 = 1;
const kLowerCaseType: i32 = 37;
const kLowerCaseNumbersSelector: i32 = 0;
const kMathematicalGreekOffSelector: i32 = 11;
const kMathematicalGreekOnSelector: i32 = 10;
const kMonospacedNumbersSelector: i32 = 0;
const kNLCCharactersSelector: i32 = 13;
const kNoFractionsSelector: i32 = 0;
const kNormalPositionSelector: i32 = 0;
const kNoStyleOptionsSelector: i32 = 0;
const kNumberCaseType: i32 = 21;
const kNumberSpacingType: i32 = 6;
const kOrdinalsSelector: i32 = 3;
const kProportionalNumbersSelector: i32 = 1;
const kQuarterWidthTextSelector: i32 = 4;
const kScientificInferiorsSelector: i32 = 4;
const kSlashedZeroOffSelector: i32 = 5;
const kSlashedZeroOnSelector: i32 = 4;
const kStyleOptionsType: i32 = 19;
const kStylisticAltEighteenOffSelector: i32 = 37;
const kStylisticAltEighteenOnSelector: i32 = 36;
const kStylisticAltEightOffSelector: i32 = 17;
const kStylisticAltEightOnSelector: i32 = 16;
const kStylisticAltElevenOffSelector: i32 = 23;
const kStylisticAltElevenOnSelector: i32 = 22;
const kStylisticAlternativesType: i32 = 35;
const kStylisticAltFifteenOffSelector: i32 = 31;
const kStylisticAltFifteenOnSelector: i32 = 30;
const kStylisticAltFiveOffSelector: i32 = 11;
const kStylisticAltFiveOnSelector: i32 = 10;
const kStylisticAltFourOffSelector: i32 = 9;
const kStylisticAltFourOnSelector: i32 = 8;
const kStylisticAltFourteenOffSelector: i32 = 29;
const kStylisticAltFourteenOnSelector: i32 = 28;
const kStylisticAltNineOffSelector: i32 = 19;
const kStylisticAltNineOnSelector: i32 = 18;
const kStylisticAltNineteenOffSelector: i32 = 39;
const kStylisticAltNineteenOnSelector: i32 = 38;
const kStylisticAltOneOffSelector: i32 = 3;
const kStylisticAltOneOnSelector: i32 = 2;
const kStylisticAltSevenOffSelector: i32 = 15;
const kStylisticAltSevenOnSelector: i32 = 14;
const kStylisticAltSeventeenOffSelector: i32 = 35;
const kStylisticAltSeventeenOnSelector: i32 = 34;
const kStylisticAltSixOffSelector: i32 = 13;
const kStylisticAltSixOnSelector: i32 = 12;
const kStylisticAltSixteenOffSelector: i32 = 33;
const kStylisticAltSixteenOnSelector: i32 = 32;
const kStylisticAltTenOffSelector: i32 = 21;
const kStylisticAltTenOnSelector: i32 = 20;
const kStylisticAltThirteenOffSelector: i32 = 27;
const kStylisticAltThirteenOnSelector: i32 = 26;
const kStylisticAltThreeOffSelector: i32 = 7;
const kStylisticAltThreeOnSelector: i32 = 6;
const kStylisticAltTwelveOffSelector: i32 = 25;
const kStylisticAltTwelveOnSelector: i32 = 24;
const kStylisticAltTwentyOffSelector: i32 = 41;
const kStylisticAltTwentyOnSelector: i32 = 40;
const kStylisticAltTwoOffSelector: i32 = 5;
const kStylisticAltTwoOnSelector: i32 = 4;
const kSuperiorsSelector: i32 = 1;
const kSwashAlternatesOffSelector: i32 = 3;
const kSwashAlternatesOnSelector: i32 = 2;
const kTitlingCapsSelector: i32 = 4;
const kTypographicExtrasType: i32 = 14;
const kVerticalFractionsSelector: i32 = 1;
const kVerticalPositionType: i32 = 10;

pub fn apply_features(font: &mut Font, features: FontFeatures) {
    // See https://chromium.googlesource.com/chromium/src/+/66.0.3359.158/third_party/harfbuzz-ng/src/hb-coretext.cc
    // for a reference implementation.
    toggle_open_type_feature(
        font,
        features.calt(),
        kContextualAlternatesType,
        kContextualAlternatesOnSelector,
        kContextualAlternatesOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.case(),
        kCaseSensitiveLayoutType,
        kCaseSensitiveLayoutOnSelector,
        kCaseSensitiveLayoutOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.cpsp(),
        kCaseSensitiveLayoutType,
        kCaseSensitiveSpacingOnSelector,
        kCaseSensitiveSpacingOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.frac(),
        kFractionsType,
        kDiagonalFractionsSelector,
        kNoFractionsSelector,
    );
    toggle_open_type_feature(
        font,
        features.liga(),
        kLigaturesType,
        kCommonLigaturesOnSelector,
        kCommonLigaturesOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.onum(),
        kNumberCaseType,
        kLowerCaseNumbersSelector,
        2,
    );
    toggle_open_type_feature(
        font,
        features.ordn(),
        kVerticalPositionType,
        kOrdinalsSelector,
        kNormalPositionSelector,
    );
    toggle_open_type_feature(
        font,
        features.pnum(),
        kNumberSpacingType,
        kProportionalNumbersSelector,
        4,
    );
    toggle_open_type_feature(
        font,
        features.ss01(),
        kStylisticAlternativesType,
        kStylisticAltOneOnSelector,
        kStylisticAltOneOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss02(),
        kStylisticAlternativesType,
        kStylisticAltTwoOnSelector,
        kStylisticAltTwoOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss03(),
        kStylisticAlternativesType,
        kStylisticAltThreeOnSelector,
        kStylisticAltThreeOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss04(),
        kStylisticAlternativesType,
        kStylisticAltFourOnSelector,
        kStylisticAltFourOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss05(),
        kStylisticAlternativesType,
        kStylisticAltFiveOnSelector,
        kStylisticAltFiveOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss06(),
        kStylisticAlternativesType,
        kStylisticAltSixOnSelector,
        kStylisticAltSixOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss07(),
        kStylisticAlternativesType,
        kStylisticAltSevenOnSelector,
        kStylisticAltSevenOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss08(),
        kStylisticAlternativesType,
        kStylisticAltEightOnSelector,
        kStylisticAltEightOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss09(),
        kStylisticAlternativesType,
        kStylisticAltNineOnSelector,
        kStylisticAltNineOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss10(),
        kStylisticAlternativesType,
        kStylisticAltTenOnSelector,
        kStylisticAltTenOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss11(),
        kStylisticAlternativesType,
        kStylisticAltElevenOnSelector,
        kStylisticAltElevenOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss12(),
        kStylisticAlternativesType,
        kStylisticAltTwelveOnSelector,
        kStylisticAltTwelveOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss13(),
        kStylisticAlternativesType,
        kStylisticAltThirteenOnSelector,
        kStylisticAltThirteenOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss14(),
        kStylisticAlternativesType,
        kStylisticAltFourteenOnSelector,
        kStylisticAltFourteenOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss15(),
        kStylisticAlternativesType,
        kStylisticAltFifteenOnSelector,
        kStylisticAltFifteenOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss16(),
        kStylisticAlternativesType,
        kStylisticAltSixteenOnSelector,
        kStylisticAltSixteenOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss17(),
        kStylisticAlternativesType,
        kStylisticAltSeventeenOnSelector,
        kStylisticAltSeventeenOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss18(),
        kStylisticAlternativesType,
        kStylisticAltEighteenOnSelector,
        kStylisticAltEighteenOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss19(),
        kStylisticAlternativesType,
        kStylisticAltNineteenOnSelector,
        kStylisticAltNineteenOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.ss20(),
        kStylisticAlternativesType,
        kStylisticAltTwentyOnSelector,
        kStylisticAltTwentyOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.subs(),
        kVerticalPositionType,
        kInferiorsSelector,
        kNormalPositionSelector,
    );
    toggle_open_type_feature(
        font,
        features.sups(),
        kVerticalPositionType,
        kSuperiorsSelector,
        kNormalPositionSelector,
    );
    toggle_open_type_feature(
        font,
        features.swsh(),
        kContextualAlternatesType,
        kSwashAlternatesOnSelector,
        kSwashAlternatesOffSelector,
    );
    toggle_open_type_feature(
        font,
        features.titl(),
        kStyleOptionsType,
        kTitlingCapsSelector,
        kNoStyleOptionsSelector,
    );
    toggle_open_type_feature(
        font,
        features.tnum(),
        kNumberSpacingType,
        kMonospacedNumbersSelector,
        4,
    );
    toggle_open_type_feature(
        font,
        features.zero(),
        kTypographicExtrasType,
        kSlashedZeroOnSelector,
        kSlashedZeroOffSelector,
    );
}

fn toggle_open_type_feature(
    font: &mut Font,
    enabled: Option<bool>,
    type_identifier: i32,
    on_selector_identifier: i32,
    off_selector_identifier: i32,
) {
    if let Some(enabled) = enabled {
        let native_font = font.native_font();
        unsafe {
            let selector_identifier = if enabled {
                on_selector_identifier
            } else {
                off_selector_identifier
            };
            let new_descriptor = CTFontDescriptorCreateCopyWithFeature(
                native_font.copy_descriptor().as_concrete_TypeRef(),
                CFNumber::from(type_identifier).as_concrete_TypeRef(),
                CFNumber::from(selector_identifier).as_concrete_TypeRef(),
            );
            let new_descriptor = CTFontDescriptor::wrap_under_create_rule(new_descriptor);
            let new_font = CTFontCreateCopyWithAttributes(
                font.native_font().as_concrete_TypeRef(),
                0.0,
                ptr::null(),
                new_descriptor.as_concrete_TypeRef(),
            );
            let new_font = CTFont::wrap_under_create_rule(new_font);
            *font = Font::from_native_font(&new_font);
        }
    }
}

#[link(name = "CoreText", kind = "framework")]
extern "C" {
    fn CTFontCreateCopyWithAttributes(
        font: CTFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
}
