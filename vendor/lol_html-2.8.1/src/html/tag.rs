macro_rules! declare_tags {
    ($($name:ident = $val:expr),+) => {
        // Pub only for integration tests
        #[repr(u64)]
        #[derive(Debug, Copy, Clone)]
        pub enum Tag {
            $($name = $val),+
        }

        // NOTE: used in the test that checks the consistency of
        // hash values with the hashing algorithm.
        #[cfg(test)]
        static TAG_STR_PAIRS: &[(Tag, &str)] = &[
            $((Tag::$name, stringify!($name))),+
        ];
    };
}

declare_tags! {
    A = 6u64,
    Area = 220_486u64,
    B = 7u64,
    Base = 236_298u64,
    Basefont = 247_776_793_209u64,
    Bgsound = 7_944_694_377u64,
    Big = 7_628u64,
    Blockquote = 265_678_647_808_810u64,
    Body = 250_174u64,
    Br = 247u64,
    Center = 279_569_751u64,
    Code = 282_922u64,
    Col = 8849u64,
    Dd = 297u64,
    Desc = 305_928u64,
    Div = 9691u64,
    Dl = 305u64,
    Dt = 313u64,
    Em = 338u64,
    Embed = 11_083_081u64,
    Font = 381_561u64,
    ForeignObject = 13_428_975_859_192_539_417u64,
    Frameset = 402_873_737_561u64,
    H1 = 416u64,
    H2 = 417u64,
    H3 = 418u64,
    H4 = 419u64,
    H5 = 420u64,
    H6 = 421u64,
    Head = 436_425u64,
    Hr = 439u64,
    I = 14u64,
    Iframe = 482_056_778u64,
    Img = 14_924u64,
    Input = 15_325_017u64,
    Keygen = 548_352_339u64,
    Li = 558u64,
    Link = 572_016u64,
    Listing = 18_749_373_036u64,
    Math = 596_781u64,
    Menu = 600_698u64,
    Meta = 600_870u64,
    Mi = 590u64,
    Mn = 595u64,
    Mo = 596u64,
    Ms = 600u64,
    Mtext = 19_704_761u64,
    Nobr = 643_319u64,
    Noembed = 21_083_266_377u64,
    Noframes = 674_703_296_856u64,
    Noscript = 675_124_329_145u64,
    Ol = 657u64,
    P = 21u64,
    Param = 22_240_466u64,
    Plaintext = 23_680_792_701_881u64,
    Pre = 22_250u64,
    Ruby = 780_542u64,
    S = 24u64,
    Script = 814_463_673u64,
    Select = 816_359_705u64,
    Small = 25_762_353u64,
    Source = 827_153_674u64,
    Span = 808_147u64,
    Strike = 832_289_290u64,
    Strong = 832_295_532u64,
    Style = 26_016_298u64,
    Sub = 25_415u64,
    Sup = 25_429u64,
    Svg = 25_452u64,
    Table = 26_418_730u64,
    Template = 870_357_441_322u64,
    Textarea = 870_730_390_854u64,
    Title = 26_699_306u64,
    Track = 26_974_480u64,
    Tt = 825u64,
    U = 26u64,
    Ul = 849u64,
    Var = 27_863u64,
    Xmp = 30_293u64,
    Wbr = 28_919u64
}

macro_rules! tag_is_one_of {
    ($hash:expr, [$($tag:ident),+]) => {
        $($hash == Tag::$tag)||+
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::LocalNameHash;

    #[test]
    fn precalculated_hash_values_consistency_with_current_implementation() {
        for &(tag, tag_string) in TAG_STR_PAIRS {
            assert_eq!(LocalNameHash::from(tag_string), tag);
        }
    }
}
