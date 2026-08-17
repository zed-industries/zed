#define Str_ ""
#define Str_str "str"
#define Str_unicode u"unicode"
#define Str_long L"long"
#define Str_concat u"con" L"cat"
#define Str_concat_parens ("concat" U"_parens")
#define Str_concat_identifier (Str_concat L"_identifier")
#define Str_hex_escape_all "\x68\x65\x78\x5f\x65\x73\x63\x61\x70\x65\x5f\x61\x6c\x6c"
#define Str_hex_escape_hex "h\x65x_\x65s\x63\x61p\x65_h\x65x"
#define Str_quote_U000022_escape "quote_\"_escape"
#define Str_Fly_away_in_my_space_U01F680_You_no_need_put_U01F4B5_in_my_pocket \
	u8"Fly_away_in_my_space_🚀_You_no_need_put_💵_in_my_pocket"
#define Fn_Str_no_args() "no_args"
#define Fn_Str_no_args_concat() "no_args_" Str_concat
#define Fn_Str_prepend_arg(arg) "prepend_" arg
#define Fn_Str_two_args(two, args) two "_" args
#define Fn_Str_three_args(three, _, args) three _ args
