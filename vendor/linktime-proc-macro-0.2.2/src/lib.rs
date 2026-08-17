#![doc = include_str!("../README.md")]

mod combine;
mod fallback;
mod generate;
mod hash;

use proc_macro::TokenStream;

/// Generates macros in the low-level crate and the linktime crate.
macro_rules! generators {
    ( $( ($crate_name:ident/$crate_name_str:literal: $( $macro_name:ident/$macro_name_linktime:ident ),*) )* ) => {
        $($(
            #[cfg(feature = $crate_name_str)]
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[proc_macro_attribute]
            pub fn $macro_name(attribute: TokenStream, item: TokenStream) -> TokenStream {
                crate::generate::generate(stringify!($crate_name), stringify!($macro_name), attribute, item)
            }
        )*)*
        $($(
            #[cfg(feature = $crate_name_str)]
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[proc_macro_attribute]
            pub fn $macro_name_linktime(attribute: TokenStream, item: TokenStream) -> TokenStream {
                crate::generate::generate("linktime", stringify!($macro_name), attribute, item)
            }
        )*)*
    };
}

generators! {
    (ctor/"ctor": ctor/ctor_linktime)
    (dtor/"dtor": dtor/dtor_linktime)
    (link_section/"link_section": in_section/in_section_linktime, section/section_linktime)
    (scattered_collect/"scattered_collect": scatter/scatter_linktime, gather/gather_linktime)
}

/// Combines idents and strings into a single ident or string.
///
/// Both idents and strings are decoded as literal strings. Punctuation is
/// ignored when building `ident` output.
///
/// Arguments are specified via named arguments:
///
/// - `output`: The type of the combined value. Must be either `ident`, `string`
///   or `isize`.
/// - `input`: The input tokens to combine.
/// - `prefix`: The prefix tokens to emit before the combined value.
/// - `suffix`: The suffix tokens to emit after the combined value.
/// - `paren`: The group to emit around the combined value.
/// - `paren_prefix`: The prefix tokens to emit before the value inside the
///   group.
/// - `paren_suffix`: The suffix tokens to emit after the value inside the
///   group.
/// - `span`: The span of the combined value. If not specified, the call-site's
///   span is used.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// // Idents, strings and numeric literals are combined as-is.
/// let str = combine!(output=string input=(prefix _ "NAME" _ suffix));
/// assert_eq!(str, "prefix_NAME_suffix");
/// let str = combine!(output=string input=(prefix _ NAME _ 1 _ suffix));
/// assert_eq!(str, "prefix_NAME_1_suffix");
///
/// // Resolves to `let prefix_NAME_suffix = 1;`
/// combine!(output=ident input=(prefix _ "NAME" _ suffix) prefix=(let ) suffix=(= 1;));
/// assert_eq!(prefix_NAME_suffix, 1);
///
/// // Set the span of the combined value to one of a specific token (useful for macros)
/// combine!(output=ident input=(prefix _ "NAME" _ suffix) span=token);
/// ```
///
/// ## Token handling
///
/// The macro will ignore grouping tokens. For ident output, the macro will also
/// ignore punctuation tokens and will strip any non-ident-compatible
/// characters.
///
/// ## Functions
///
/// The macro also supports a number of special function tokens which are
/// resolved recursively and can be nested arbitrarily deep.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// macro_rules! make_name_max_length {
///     ($prefix:ident $name:ident $suffix:ident) => {
///       // Ensure user calls are not expanded recursively
///       /* $crate:: */ make_name_max_length!(@internal (__RAW__(input=($prefix))) (__RAW__(input=($name))) (__RAW__(input=($suffix))))
///     };
///     (@internal $prefix:tt $name:tt $suffix:tt) => {
///         combine!(output=string input=(
///             __IF__(
///               test=(__GT__(a=(__LENGTH__(string=($prefix _ $name _ $suffix))) b=20))
///               then=(
///                 __SUBSTRING__(input=($prefix _ $name _ $suffix) start=0 end=10)
///                 _ $
///                 // uppercase hash
///                 __TRANSLATE__(
///                   input=(__SUBSTRING__(input=(__HASH__(string=($prefix _ $name _ $suffix))) length=10))
///                   pattern=[a-f] replacement=[A-F]
///                 )
///               )
///               else=(
///                 $prefix _ $name _ $suffix _ __LENGTH__(string=($name))
///               )
///             )
///         ))
///     };
/// }
/// assert_eq!(make_name_max_length!(prefix NAME suffix), "prefix_NAME_suffix_4");
/// assert_eq!(make_name_max_length!(prefix LONG_NAME suffix), "prefix_LON_$64470473B5");
/// ```
///
/// ## Source span functions
///
///  - `__FILE__(of=token)`: The file name of the file containing the `token`.
///    Supported on Rust 1.88+, returns "" otherwise.
///  - `__LINE__(of=token)`: The line number of the `token`. Supported on Rust
///    1.88+, returns 0 otherwise.
///  - `__COLUMN__(of=token)`: The column number of the `token`. Supported on
///    Rust 1.88+, returns 0 otherwise.
///
/// ```rust,ignore
/// # use linktime_proc_macro::combine;
/// let file = combine!(output=string input=("file:" __FILE__(of=token) ":" __LINE__(of=token) ":" __COLUMN__(of=token)));
/// assert_eq!(file, "file:linktime-proc-macro/src/lib.rs:6:110");
/// ```
///
///  - `__LOCATIONHASH__(of=token, alphabet=[chars...], ignore_base=token)`:
///    Returns a hash of location information for all tokens within the tree of
///    `token`. If `alphabet` is specified, the hash is converted to a string
///    using the characters in the `alphabet`. No zero padding is applied in this
///    case.
///
///    `ignore_base` hashes tokens from that crate by content instead of location,
///    so macro-synthesized def-site spans don't shift the hash when that crate
///    is edited.
/// ```rust,ignore
/// # use linktime_proc_macro::combine;
/// let location_hash = combine!(output=string input=("location_hash:" __LOCATIONHASH__(of=(a bunch of tokens) alphabet=[a-z])));
/// assert_eq!(location_hash, "location_hash:dwsrxapjetrwwu");
/// ```
///
///  - `__SOURCE__(of=token)`: The source text content of the `token`.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// macro_rules! source_of {
///     ($token:item) => {
///         combine!(output=string input=("source" "(@" __LINE__(of=$token) "):" __SOURCE__(of=$token)))
///     };
/// }
/// // Use a macro to get the line and source text of a token
/// assert_eq!(source_of!(fn foo() {}), "source(@12):fn foo () {}");
/// assert_eq!(source_of!(static X: u32 = 1;), "source(@13):static X : u32 = 1 ;");
///
/// let source = combine!(output=string input=("source:" __SOURCE__(of=(my token))));
/// assert_eq!(source, "source:my token");
/// ```
///
/// ## String functions
///
///  - `__HASH__(string=(tokens...) [alphabet=[chars...]])`: The hash of the
///    `tokens` when converted to a string, by default as a zero-padding
///    lowercase hexadecimal string. `tokens` may contain nested function calls.
///    If `alphabet` is specified, the hash is converted to a string using the
///    characters in the `alphabet`. No zero padding is applied in this case.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// let hash = combine!(output=string input=("hash:" __HASH__(string=(__LENGTH__(string="a")))));
/// assert_eq!(hash, "hash:65cd25028f98f158");
/// let hash = combine!(output=string input=("hash:" __HASH__(string=(1))));
/// assert_eq!(hash, "hash:65cd25028f98f158");
/// let hash = combine!(output=string input=("hash:" __HASH__(string=(1) alphabet=[0-9a-f])));
/// assert_eq!(hash, "hash:65cd25028f98f158");
/// let hash = combine!(output=string input=("hash:" __HASH__(string=(1) alphabet=[a-z])));
/// assert_eq!(hash, "hash:cywprjlaqhbdie");
/// ```
///
///  - `__LENGTH__(string=(tokens...))`: The length of the `tokens` when
///    converted to a string.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// let length = combine!(output=string input=("length:" __LENGTH__(string="a")));
/// assert_eq!(length, "length:1");
/// ```
///
///  - `__SUBSTRING__(input=(input) start=start end=end length=length)`: The
///    substring of the `input` from the `start` to the `end` index (exclusive)
///    or `length` characters from the `start` index. If `end` or `length` would
///    exceed the length of the input, the substring is truncated to the end of
///    the input. If `end` or `length` are missing, the substring is the entire
///    input from `start` to the end of the input.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// let substring = combine!(output=string input=("substring:" __SUBSTRING__(input="abc" start=1 end=2)));
/// assert_eq!(substring, "substring:b");
/// let substring = combine!(output=string input=("substring:" __SUBSTRING__(input="abc" start=1 length=2)));
/// assert_eq!(substring, "substring:bc");
/// ```
///
///  - `__PAD__(input=(input) length=length left=(padding...)
///    right=(padding...))`: Pad the `input` to the `length` with the
///    `padding...` (converted to a string).
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// let pad = combine!(output=string input=("pad:" __PAD__(input=123 length=5 left=0)));
/// assert_eq!(pad, "pad:00123");
/// ```
///
/// ## Pattern functions
///
///  - `__REPLACE__(input=(input) pattern=(pattern...)|[chars...]
///    replacement=(replacement))`: Replace all occurrences of the `pattern...`
///    (converted to a string) in the `input` with the `replacement`. If
///    `replacement` is missing or empty, the pattern is removed.
///
/// Note: `pattern` may be specified as a regex-like character group. Use square
/// brackets to specify a character group.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// let replace = combine!(output=string input=("replace:" __REPLACE__(input=(a b c) pattern=(b) replacement=(x))));
/// assert_eq!(replace, "replace:axc");
///
/// // Remove non-alnum characters
/// let translate = combine!(output=string input=("replace:" __REPLACE__(input="⚠️ thx 1138" pattern=[^a-zA-Z0-9] replacement="")));
/// assert_eq!(translate, "replace:thx1138");
/// ```
///
///  - `__TRANSLATE__(input=(input) pattern=(pattern...)|[chars...]
///    replacement=(replacement)|[chars...])`: Replace all occurrences of the
///    `pattern...` (converted to a string) characters in the `input` with the
///    `replacement` characters. If replacement is missing or empty, the
///    character is removed. Otherwise, the replacement character is selected
///    from the replacement string in order (repeating the last character if
///    necessary).
///
/// Note: `pattern` and `replacement` may be specified as regex-like character
/// groups. Use square brackets to specify a character group.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// // Deletes all digits
/// let translate = combine!(output=string input=("translate:" __TRANSLATE__(input=(thx 1138) pattern=(0 1 2 3 4 5 6 7 8 9))));
/// assert_eq!(translate, "translate:thx");
/// // Uppercase all ASCII letters
/// let translate = combine!(output=string input=("translate:" __TRANSLATE__(input=(thx 1138) pattern=[a-z] replacement=[A-Z])));
/// assert_eq!(translate, "translate:THX1138");
/// ```
///
///  - `__TRIM__(input=(input) left=(padding...)|[chars...]
///    right=(padding...)|[chars...])`: Trim the
///    `input` of the `left` and `right` patterns.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// // Note: because we are using escaped characters, we need to put them in a string
/// let trim = combine!(output=string input=("trim:" __TRIM__(input="  thx 1138  " left=[" \n\t"] right=[" \n\t"])));
/// assert_eq!(trim, "trim:thx 1138");
/// let trim = combine!(output=string input=("trim:" __TRIM__(input="  thx 1138  " left=[' '] right=[' '])));
/// assert_eq!(trim, "trim:thx 1138");
/// ```
///
///  - `__CONTAINS__(input=(input) pattern=(pattern...)|[chars...])`: Check if
///    the `input` contains the `pattern...`. If so, returns `1`, otherwise `0`.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// let contains = combine!(output=isize input=("contains:" __CONTAINS__(input="thx 1138" pattern=[0-9])));
/// assert_eq!(contains, 1);
/// let contains = combine!(output=isize input=("contains:" __CONTAINS__(input="thx 1138" pattern=[aeiou])));
/// assert_eq!(contains, 0);
/// let contains = combine!(output=isize input=("contains:" __CONTAINS__(input="thx 1138" pattern="1138")));
/// assert_eq!(contains, 1);
/// ```
///
///  - `__STREQ__(a=(tokens...) b=(tokens...))`: Compare `a` and `b` as strings.
///    Returns `1` if equal, `0` otherwise.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// let eq = combine!(output=isize input=("eq:" __STREQ__(a="thx 1138" b="thx 1138")));
/// assert_eq!(eq, 1);
/// let eq = combine!(output=isize input=("eq:" __STREQ__(a="thx 1138" b="thx 1139")));
/// assert_eq!(eq, 0);
/// ```
///
/// ## Comparison functions
///
///  - `__IF__(test=(tokens...) then=(tokens...) else=(tokens...))`: If test is
///    non-zero, emit the `then` tokens, otherwise emit the `else` tokens.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// // Outputs true or false idents
/// assert!(combine!(output=ident input=(__IF__(test=1 then="true" else="false"))));
/// assert!(combine!(output=ident input=(__IF__(test=(__CONTAINS__(input="thx 1138" pattern="1138")) then="true" else="false"))));
/// ```
///
/// - `__LT__(a=(tokens...) b=(tokens...))`: Compare `a` < `b` as numbers.
/// - `__GT__(a=(tokens...) b=(tokens...))`: Compare `a` > `b` as numbers.
/// - `__LE__(a=(tokens...) b=(tokens...))`: Compare `a` <= `b` as numbers.
/// - `__GE__(a=(tokens...) b=(tokens...))`: Compare `a` >= `b` as numbers.
/// - `__EQ__(a=(tokens...) b=(tokens...))`: Compare `a` == `b` as numbers.
/// - `__NE__(a=(tokens...) b=(tokens...))`: Compare `a` != `b` as numbers.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// let lt = combine!(output=isize input=(__LT__(a=1 b=2)));
/// assert_eq!(lt, 1);
/// let gt = combine!(output=isize input=(__GT__(a=1 b=2)));
/// assert_eq!(gt, 0);
/// ```
///
/// ## Math functions
///
///  - `__ADD__(a=(tokens...) b=(tokens...))`: Add the `a` and `b` tokens.
///  - `__SUB__(a=(tokens...) b=(tokens...))`: Subtract the `b` from `a`.
///  - `__MUL__(a=(tokens...) b=(tokens...))`: Multiply the `a` and `b` tokens.
///  - `__DIV__(a=(tokens...) b=(tokens...))`: Divide the `a` by `b`.
///  - `__AND__(a=(tokens...) b=(tokens...))`: Bitwise AND the `a` and `b`
///    tokens.
///  - `__OR__(a=(tokens...) b=(tokens...))`: Bitwise OR the `a` and `b` tokens.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// assert_eq!(combine!(output=isize input=("add:" __ADD__(a=1 b=2))), 3);
/// assert_eq!(combine!(output=isize input=("sub:" __SUB__(a=1 b=2))), -1);
/// assert_eq!(combine!(output=isize input=("mul:" __MUL__(a=1 b=2))), 2);
/// assert_eq!(combine!(output=isize input=("div:" __DIV__(a=1 b=2))), 0);
/// assert_eq!(combine!(output=isize input=("and:" __AND__(a=1 b=2))), 0);
/// assert_eq!(combine!(output=isize input=("or:" __OR__(a=1 b=2))), 3);
/// ```
///
/// ## Conversion functions
///
///  - `__TOSTRING__(input=(tokens...))`: Convert the `tokens` to a string
///    literal.
///  - `__RAW__(input=(tokens...))`: Convert the `tokens` to a string, ignoring
///    nested function calls (recommended for user input).
///  - `__TOIDENT__(input=(tokens...))`: Convert the `tokens` to an ident,
///    stripping invalid characters.
///  - `__TONUMBER__(input=(tokens...))`: Convert the `tokens` to a numeric
///    literal.
///
/// ```rust
/// # use linktime_proc_macro::combine;
/// let string = combine!(output=string input=("string:" __TOSTRING__(input=(a b c))));
/// assert_eq!(string, "string:abc");
///
/// let number = combine!(output=isize input=("number:" __TONUMBER__(input=(1 2 3))));
/// assert_eq!(number, 123);
/// let number = combine!(output=isize input=("number:" __TONUMBER__(input=("0x" 123 _ 456))));
/// assert_eq!(number, 0x123456);
///
/// let ident = combine!(output=string input=("ident:" __TOIDENT__(input=(a $ b _ c))));
/// assert_eq!(ident, "ident:ab_c");
///
/// let raw = combine!(output=string input=("raw:" __RAW__(input=(__TOSTRING__(input=(a b c))))));
/// assert_eq!(raw, "raw:__TOSTRING__input=abc");
/// ```
#[proc_macro]
pub fn combine(item: TokenStream) -> TokenStream {
    combine::combine(item)
}
