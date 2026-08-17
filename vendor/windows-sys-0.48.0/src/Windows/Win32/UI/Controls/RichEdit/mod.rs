pub type IRichEditOle = *mut ::core::ffi::c_void;
pub type IRichEditOleCallback = *mut ::core::ffi::c_void;
pub type IRicheditUiaOverrides = *mut ::core::ffi::c_void;
pub type ITextDisplays = *mut ::core::ffi::c_void;
pub type ITextDocument = *mut ::core::ffi::c_void;
pub type ITextDocument2 = *mut ::core::ffi::c_void;
pub type ITextDocument2Old = *mut ::core::ffi::c_void;
pub type ITextFont = *mut ::core::ffi::c_void;
pub type ITextFont2 = *mut ::core::ffi::c_void;
pub type ITextHost = *mut ::core::ffi::c_void;
pub type ITextHost2 = *mut ::core::ffi::c_void;
pub type ITextPara = *mut ::core::ffi::c_void;
pub type ITextPara2 = *mut ::core::ffi::c_void;
pub type ITextRange = *mut ::core::ffi::c_void;
pub type ITextRange2 = *mut ::core::ffi::c_void;
pub type ITextRow = *mut ::core::ffi::c_void;
pub type ITextSelection = *mut ::core::ffi::c_void;
pub type ITextSelection2 = *mut ::core::ffi::c_void;
pub type ITextServices = *mut ::core::ffi::c_void;
pub type ITextServices2 = *mut ::core::ffi::c_void;
pub type ITextStory = *mut ::core::ffi::c_void;
pub type ITextStoryRanges = *mut ::core::ffi::c_void;
pub type ITextStoryRanges2 = *mut ::core::ffi::c_void;
pub type ITextStrings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ATP_CHANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ATP_NOCHANGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ATP_NODELIMITER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ATP_REPLACEALLTEXT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_DISABLEMIXEDLGC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_ENABLEDRIVELETTERS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_ENABLEEA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_ENABLEEAURLS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_ENABLEEMAILADDR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_ENABLETELNO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_ENABLEURL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_CONTEXTALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_CONTEXTREADING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_FORCERECALC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_LEGACYBIDICLASS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_NEUTRALOVERRIDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_PLAINTEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_RTLDIR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_UNICODEBIDI: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_CONTEXTALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_CONTEXTREADING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_DEFPARADIR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_LEGACYBIDICLASS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_NEUTRALOVERRIDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_PLAINTEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_UNICODEBIDI: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CERICHEDIT_CLASSA: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("RichEditCEA");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CERICHEDIT_CLASSW: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("RichEditCEW");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CF_RETEXTOBJ: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("RichEdit Text and Objects");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CF_RTF: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Rich Text Format");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CF_RTFNOOBJS: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Rich Text Format Without Objects");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_CONVERSATION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_DATETIME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_FILENAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_FULLWIDTHALPHANUMERIC: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_HALFWIDTHALPHANUMERIC: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_HALFWIDTHKATAKANA: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_HANGUL: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_HIRAGANA: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_KATAKANA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_NUMERIC: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_READING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECOOP_AND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECOOP_OR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECOOP_SET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECOOP_XOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_AUTOHSCROLL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_AUTOVSCROLL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_AUTOWORDSELECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_NOHIDESEL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_READONLY: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_SAVESEL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_SELECTIONBAR: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_VERTICAL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_WANTRETURN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ELLIPSIS_END: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ELLIPSIS_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ELLIPSIS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ELLIPSIS_WORD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_ENTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_EXIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_EXPAND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_EXPANDDOCUMENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_EXPANDSELECTION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_GETVIEWMODE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_MOVESELECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_PROMOTE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_AUTOURLDETECT: u32 = 1115u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_CALLAUTOCORRECTPROC: u32 = 1279u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_CANPASTE: u32 = 1074u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_CANREDO: u32 = 1109u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_CONVPOSITION: u32 = 1132u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_DISPLAYBAND: u32 = 1075u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_EXGETSEL: u32 = 1076u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_EXLIMITTEXT: u32 = 1077u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_EXLINEFROMCHAR: u32 = 1078u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_EXSETSEL: u32 = 1079u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_FINDTEXT: u32 = 1080u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_FINDTEXTEX: u32 = 1103u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_FINDTEXTEXW: u32 = 1148u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_FINDTEXTW: u32 = 1147u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_FINDWORDBREAK: u32 = 1100u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_FORMATRANGE: u32 = 1081u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETAUTOCORRECTPROC: u32 = 1257u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETAUTOURLDETECT: u32 = 1116u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETBIDIOPTIONS: u32 = 1225u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETCHARFORMAT: u32 = 1082u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETCTFMODEBIAS: u32 = 1261u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETCTFOPENSTATUS: u32 = 1264u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETEDITSTYLE: u32 = 1229u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETEDITSTYLEEX: u32 = 1300u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETELLIPSISMODE: u32 = 1329u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETELLIPSISSTATE: u32 = 1346u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETEVENTMASK: u32 = 1083u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETHYPHENATEINFO: u32 = 1254u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETIMECOLOR: u32 = 1129u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETIMECOMPMODE: u32 = 1146u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETIMECOMPTEXT: u32 = 1266u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETIMEMODEBIAS: u32 = 1151u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETIMEOPTIONS: u32 = 1131u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETIMEPROPERTY: u32 = 1268u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETLANGOPTIONS: u32 = 1145u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETOLEINTERFACE: u32 = 1084u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETOPTIONS: u32 = 1102u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETPAGE: u32 = 1252u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETPAGEROTATE: u32 = 1259u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETPARAFORMAT: u32 = 1085u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETPUNCTUATION: u32 = 1125u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETQUERYRTFOBJ: u32 = 1293u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETREDONAME: u32 = 1111u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETSCROLLPOS: u32 = 1245u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETSELTEXT: u32 = 1086u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETSTORYTYPE: u32 = 1314u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTABLEPARMS: u32 = 1289u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTEXTEX: u32 = 1118u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTEXTLENGTHEX: u32 = 1119u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTEXTMODE: u32 = 1114u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTEXTRANGE: u32 = 1099u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTOUCHOPTIONS: u32 = 1334u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTYPOGRAPHYOPTIONS: u32 = 1227u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETUNDONAME: u32 = 1110u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETVIEWKIND: u32 = 1250u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETWORDBREAKPROCEX: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETWORDWRAPMODE: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETZOOM: u32 = 1248u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_HIDESELECTION: u32 = 1087u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_INSERTIMAGE: u32 = 1338u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_INSERTTABLE: u32 = 1256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_ISIME: u32 = 1267u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_OUTLINE: u32 = 1244u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_PASTESPECIAL: u32 = 1088u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_RECONVERSION: u32 = 1149u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_REDO: u32 = 1108u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_REQUESTRESIZE: u32 = 1089u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SELECTIONTYPE: u32 = 1090u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETAUTOCORRECTPROC: u32 = 1258u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETBIDIOPTIONS: u32 = 1224u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETBKGNDCOLOR: u32 = 1091u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETCHARFORMAT: u32 = 1092u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETCTFMODEBIAS: u32 = 1262u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETCTFOPENSTATUS: u32 = 1265u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETEDITSTYLE: u32 = 1228u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETEDITSTYLEEX: u32 = 1299u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETELLIPSISMODE: u32 = 1330u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETEVENTMASK: u32 = 1093u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETFONTSIZE: u32 = 1247u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETHYPHENATEINFO: u32 = 1255u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETIMECOLOR: u32 = 1128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETIMEMODEBIAS: u32 = 1150u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETIMEOPTIONS: u32 = 1130u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETLANGOPTIONS: u32 = 1144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETOLECALLBACK: u32 = 1094u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETOPTIONS: u32 = 1101u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETPAGE: u32 = 1253u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETPAGEROTATE: u32 = 1260u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETPALETTE: u32 = 1117u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETPARAFORMAT: u32 = 1095u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETPUNCTUATION: u32 = 1124u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETQUERYRTFOBJ: u32 = 1294u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETSCROLLPOS: u32 = 1246u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETSTORYTYPE: u32 = 1315u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETTABLEPARMS: u32 = 1331u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETTARGETDEVICE: u32 = 1096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETTEXTEX: u32 = 1121u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETTEXTMODE: u32 = 1113u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETTOUCHOPTIONS: u32 = 1335u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETTYPOGRAPHYOPTIONS: u32 = 1226u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETUIANAME: u32 = 1344u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETUNDOLIMIT: u32 = 1106u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETVIEWKIND: u32 = 1251u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETWORDBREAKPROCEX: u32 = 1105u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETWORDWRAPMODE: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETZOOM: u32 = 1249u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SHOWSCROLLBAR: u32 = 1120u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_STOPGROUPTYPING: u32 = 1112u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_STREAMIN: u32 = 1097u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_STREAMOUT: u32 = 1098u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_CHANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_CLIPFORMAT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_CORRECTTEXT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_DRAGDROPDONE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_DROPFILES: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_ENDCOMPOSITION: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_GROUPTYPINGCHANGE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_HIDELINKTOOLTIP: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_IMECHANGE: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_KEYEVENTS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_LANGCHANGE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_LINK: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_LOWFIRTF: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_MOUSEEVENTS: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_OBJECTPOSITIONS: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_PAGECHANGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_PARAGRAPHEXPANDED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_PROTECTED: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_REQUESTRESIZE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_SCROLL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_SCROLLEVENTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_SELCHANGE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_STARTCOMPOSITION: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_UPDATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_ALIGNLTR: u32 = 1808u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_ALIGNRTL: u32 = 1809u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_CLIPFORMAT: u32 = 1810u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_CORRECTTEXT: u32 = 1797u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_DRAGDROPDONE: u32 = 1804u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_DROPFILES: u32 = 1795u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_ENDCOMPOSITION: u32 = 1812u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_IMECHANGE: u32 = 1799u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_LINK: u32 = 1803u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_LOWFIRTF: u32 = 1807u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_MSGFILTER: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_OBJECTPOSITIONS: u32 = 1802u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_OLEOPFAILED: u32 = 1801u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_PAGECHANGE: u32 = 1806u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_PARAGRAPHEXPANDED: u32 = 1805u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_PROTECTED: u32 = 1796u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_REQUESTRESIZE: u32 = 1793u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_SAVECLIPBOARD: u32 = 1800u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_SELCHANGE: u32 = 1794u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_STARTCOMPOSITION: u32 = 1811u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_STOPNOUNDO: u32 = 1798u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EPR_0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EPR_180: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EPR_270: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EPR_90: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EPR_SE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_DISABLENOSCROLL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_EX_NOCALLOLEINIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_NOIME: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_NOOLEDRAGDROP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_SAVESEL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_SELECTIONBAR: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_SELFIME: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_SUNKEN: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_VERTICAL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const FR_MATCHALEFHAMZA: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const FR_MATCHDIAC: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const FR_MATCHKASHIDA: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCMF_GRIPPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCMF_MOUSEMENU: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCMF_SPELLING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCMF_TOUCHMENU: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCM_MOUSEMENU: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCM_TOUCHMENU: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICM_CTF: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICM_LEVEL2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICM_LEVEL2_5: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICM_LEVEL2_SUI: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICM_LEVEL3: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICM_NOTOPEN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_AUTOFONT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_AUTOFONTSIZEADJUST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_AUTOKEYBOARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_CLOSESTATUSWINDOW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_DUALFONT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_FORCEACTIVE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_FORCEDISABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_FORCEENABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_FORCEINACTIVE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_FORCENONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_FORCEREMEMBER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_IMEALWAYSSENDNOTIFY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_IMECANCELCOMPLETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_IMEUIINTEGRATION: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_MULTIPLEEDIT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_NOIMPLICITLANG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_NOKBDLIDFIXUP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_NORTFFONTSUBSTITUTE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_SMODE_NONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_SMODE_PLAURALCLAUSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_SPELLCHECKING: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_TKBPREDICTION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_UIFONTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_VERTICAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MAX_TABLE_CELLS: u32 = 63u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MAX_TAB_STOPS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MSFTEDIT_CLASS: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("RICHEDIT50W");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const OLEOP_DOVERB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PC_DELIMITER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PC_FOLLOWING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PC_LEADING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PC_OVERFLOW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_FULL_GLYPHS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_FULL_INTERLETTER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_FULL_INTERWORD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_FULL_NEWSPAPER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_FULL_SCALED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_JUSTIFY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_BORDER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_BOX: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_COLLAPSED: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_DONOTHYPHEN: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_KEEP: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_KEEPNEXT: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_LINESPACING: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_NOLINENUMBER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_NOWIDOWCONTROL: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_NUMBERINGSTART: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_NUMBERINGSTYLE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_NUMBERINGTAB: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_OUTLINELEVEL: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_PAGEBREAKBEFORE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_RESERVED2: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_SHADING: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_SIDEBYSIDE: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_SPACEAFTER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_SPACEBEFORE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_STYLE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_TABLE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_TABLEROWDELIMITER: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_TEXTWRAPPINGBREAK: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_NULL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_READWRITEMASK: i32 = 2047i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RICHEDIT60_CLASS: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("RICHEDIT60W");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RICHEDIT_CLASS: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("RichEdit20W");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RICHEDIT_CLASS10A: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("RICHEDIT");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RICHEDIT_CLASSA: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("RichEdit20A");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RICHEDIT_CLASSW: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("RichEdit20W");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RTO_DISABLEHANDLES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RTO_READINGMODE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RTO_SHOWHANDLES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_ALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_ASSOCIATEFONT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_ASSOCIATEFONT2: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_CHARREPFROMLCID: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_NOKBUPDATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_SELECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_SMARTFONT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_USEUIRULES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_WORD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_ALLOWBEEPS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_BEEPONMAXTEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_BIDI: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_CTFALLOWEMBED: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_CTFALLOWPROOFING: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_CTFALLOWSMARTTAG: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_CTFNOLOCK: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_CUSTOMLOOK: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_DEFAULTLATINLIGA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_DRAFTMODE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EMULATE10: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EMULATESYSEDIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EXTENDBACKCOLOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_HANDLEFRIENDLYURL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_HIDETEMPFORMAT: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_MULTITOUCH: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_NOACETATESELECTION: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_NOMATH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_NOTABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_NOTHEMING: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_USEMOUSEWPARAM: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_USESINGLELINE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_HIDEGRIDLINES: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_HYPERLINKTOOLTIPS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_LBSCROLLNOTIFY: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_LOGICALCARET: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_LOWERCASE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_MAPCPS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_MAX: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_MULTISELECT: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_NOEALINEHEIGHTADJUST: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_NOFOCUSLINKNOTIFY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_NOIME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_NOINPUTSEQUENCECHK: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_SCROLLONKILLFOCUS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_SMARTDRAGDROP: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_UPPERCASE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_USEAIMM: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_USEATFONT: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_USECRLF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_USECTF: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_WORDDRAGDROP: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_XLTCRCRLFTOCR: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SFF_KEEPDOCINFO: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SFF_PERSISTVIEWSCALE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SFF_PLAINRTF: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SFF_PWD: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SFF_SELECTION: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SFF_WRITEXTRAPAR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_NCRFORNONASCII: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_RTF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_RTFNOOBJS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_RTFVAL: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_TEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_TEXTIZED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_UNICODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_USECODEPAGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SPF_DONTSETDEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SPF_SETDEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ST_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ST_KEEPUNDO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ST_NEWCHARS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ST_SELECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ST_UNICODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const S_MSG_KEY_IGNORED: ::windows_sys::core::HRESULT = 262657i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TO_ADVANCEDLAYOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TO_ADVANCEDTYPOGRAPHY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TO_DISABLECUSTOMTEXTOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TO_SIMPLELINEBREAK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXES_ISDIALOG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_ADVANCEDINPUT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_ALLOWBEEP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_AUTOWORDSEL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_BACKSTYLECHANGE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_CHARFORMATCHANGE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_CLIENTRECTCHANGE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_D2DDWRITE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_D2DPIXELSNAPPED: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_D2DSIMPLETYPOGRAPHY: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_D2DSUBPIXELLINES: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_DISABLEDRAG: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_EXTENTCHANGE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_FLASHLASTPASSWORDCHAR: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_HIDESELECTION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_MAXLENGTHCHANGE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_MULTILINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_NOTHREADREFCOUNT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_PARAFORMATCHANGE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_READONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_RICHTEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_SAVESELECTION: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_SCROLLBARCHANGE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_SELBARCHANGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_SHOWACCELERATOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_SHOWPASSWORD: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_USECURRENTBKG: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_USEPASSWORD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_VERTICAL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_VIEWINSETCHANGE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_WORDWRAP: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const VM_NORMAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const VM_OUTLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const VM_PAGE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WBF_CUSTOM: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WBF_LEVEL1: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WBF_LEVEL2: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WBF_OVERFLOW: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WBF_WORDBREAK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WBF_WORDWRAP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WB_MOVEWORDNEXT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WB_MOVEWORDPREV: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WB_NEXTBREAK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WB_PREVBREAK: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const cchTextLimitDefault: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const lDefaultTab: u32 = 720u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const yHeightCharPtsMost: u32 = 1638u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type CARET_FLAGS = i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CARET_NONE: CARET_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CARET_CUSTOM: CARET_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CARET_RTL: CARET_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CARET_ITALIC: CARET_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CARET_NULL: CARET_FLAGS = 64i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CARET_ROTATE90: CARET_FLAGS = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type CFE_EFFECTS = u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_ALLCAPS: CFE_EFFECTS = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_AUTOBACKCOLOR: CFE_EFFECTS = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_DISABLED: CFE_EFFECTS = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_EMBOSS: CFE_EFFECTS = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_HIDDEN: CFE_EFFECTS = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_IMPRINT: CFE_EFFECTS = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_OUTLINE: CFE_EFFECTS = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_REVISED: CFE_EFFECTS = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_SHADOW: CFE_EFFECTS = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_SMALLCAPS: CFE_EFFECTS = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_AUTOCOLOR: CFE_EFFECTS = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_BOLD: CFE_EFFECTS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_ITALIC: CFE_EFFECTS = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_STRIKEOUT: CFE_EFFECTS = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_UNDERLINE: CFE_EFFECTS = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_PROTECTED: CFE_EFFECTS = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_LINK: CFE_EFFECTS = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_SUBSCRIPT: CFE_EFFECTS = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_SUPERSCRIPT: CFE_EFFECTS = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_FONTBOUND: CFE_EFFECTS = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_LINKPROTECTED: CFE_EFFECTS = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_EXTENDED: CFE_EFFECTS = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_MATHNOBUILDUP: CFE_EFFECTS = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_MATH: CFE_EFFECTS = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_MATHORDINARY: CFE_EFFECTS = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type CFM_MASK = u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_SUBSCRIPT: CFM_MASK = 196608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_SUPERSCRIPT: CFM_MASK = 196608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_EFFECTS: CFM_MASK = 1073741887u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_ALL: CFM_MASK = 4160749631u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_BOLD: CFM_MASK = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_CHARSET: CFM_MASK = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_COLOR: CFM_MASK = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_FACE: CFM_MASK = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_ITALIC: CFM_MASK = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_OFFSET: CFM_MASK = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_PROTECTED: CFM_MASK = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_SIZE: CFM_MASK = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_STRIKEOUT: CFM_MASK = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_UNDERLINE: CFM_MASK = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_LINK: CFM_MASK = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_SMALLCAPS: CFM_MASK = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_ALLCAPS: CFM_MASK = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_HIDDEN: CFM_MASK = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_OUTLINE: CFM_MASK = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_SHADOW: CFM_MASK = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_EMBOSS: CFM_MASK = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_IMPRINT: CFM_MASK = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_DISABLED: CFM_MASK = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_REVISED: CFM_MASK = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_REVAUTHOR: CFM_MASK = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_ANIMATION: CFM_MASK = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_STYLE: CFM_MASK = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_KERNING: CFM_MASK = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_SPACING: CFM_MASK = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_WEIGHT: CFM_MASK = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_UNDERLINETYPE: CFM_MASK = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_COOKIE: CFM_MASK = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_LCID: CFM_MASK = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_BACKCOLOR: CFM_MASK = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_EFFECTS2: CFM_MASK = 1141080063u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_ALL2: CFM_MASK = 4294967295u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_FONTBOUND: CFM_MASK = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_LINKPROTECTED: CFM_MASK = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_EXTENDED: CFM_MASK = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_MATHNOBUILDUP: CFM_MASK = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_MATH: CFM_MASK = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_MATHORDINARY: CFM_MASK = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_ALLEFFECTS: CFM_MASK = 2115207167u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type CHANGETYPE = i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CN_GENERIC: CHANGETYPE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CN_TEXTCHANGED: CHANGETYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CN_NEWUNDO: CHANGETYPE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CN_NEWREDO: CHANGETYPE = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type ENDCOMPOSITIONNOTIFY_CODE = u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECN_ENDCOMPOSITION: ENDCOMPOSITIONNOTIFY_CODE = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECN_NEWTEXT: ENDCOMPOSITIONNOTIFY_CODE = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type GETTEXTEX_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GT_DEFAULT: GETTEXTEX_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GT_NOHIDDENTEXT: GETTEXTEX_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GT_RAWTEXT: GETTEXTEX_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GT_SELECTION: GETTEXTEX_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GT_USECRLF: GETTEXTEX_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type GETTEXTLENGTHEX_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GTL_DEFAULT: GETTEXTLENGTHEX_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GTL_USECRLF: GETTEXTLENGTHEX_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GTL_PRECISE: GETTEXTLENGTHEX_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GTL_CLOSE: GETTEXTLENGTHEX_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GTL_NUMCHARS: GETTEXTLENGTHEX_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GTL_NUMBYTES: GETTEXTLENGTHEX_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type IMECOMPTEXT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICT_RESULTREADSTR: IMECOMPTEXT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type KHYPH = i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphNil: KHYPH = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphNormal: KHYPH = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphAddBefore: KHYPH = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphChangeBefore: KHYPH = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphDeleteBefore: KHYPH = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphChangeAfter: KHYPH = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphDelAndChange: KHYPH = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type MANCODE = i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MBOLD: MANCODE = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MITAL: MANCODE = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MGREEK: MANCODE = 64i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MROMN: MANCODE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MSCRP: MANCODE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MFRAK: MANCODE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MOPEN: MANCODE = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MSANS: MANCODE = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MMONO: MANCODE = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MMATH: MANCODE = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MISOL: MANCODE = 7i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MINIT: MANCODE = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MTAIL: MANCODE = 9i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MSTRCH: MANCODE = 10i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MLOOP: MANCODE = 11i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MOPENA: MANCODE = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type OBJECTTYPE = i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSimpleText: OBJECTTYPE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRuby: OBJECTTYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHorzVert: OBJECTTYPE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWarichu: OBJECTTYPE = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEq: OBJECTTYPE = 9i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMath: OBJECTTYPE = 10i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAccent: OBJECTTYPE = 10i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBox: OBJECTTYPE = 11i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxedFormula: OBJECTTYPE = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBrackets: OBJECTTYPE = 13i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBracketsWithSeps: OBJECTTYPE = 14i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEquationArray: OBJECTTYPE = 15i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFraction: OBJECTTYPE = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFunctionApply: OBJECTTYPE = 17i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLeftSubSup: OBJECTTYPE = 18i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLowerLimit: OBJECTTYPE = 19i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatrix: OBJECTTYPE = 20i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNary: OBJECTTYPE = 21i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOpChar: OBJECTTYPE = 22i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOverbar: OBJECTTYPE = 23i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantom: OBJECTTYPE = 24i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRadical: OBJECTTYPE = 25i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSlashedFraction: OBJECTTYPE = 26i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStack: OBJECTTYPE = 27i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStretchStack: OBJECTTYPE = 28i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSubscript: OBJECTTYPE = 29i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSubSup: OBJECTTYPE = 30i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSuperscript: OBJECTTYPE = 31i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnderbar: OBJECTTYPE = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUpperLimit: OBJECTTYPE = 33i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomObjectMax: OBJECTTYPE = 33i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type PARAFORMAT_ALIGNMENT = u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_CENTER: PARAFORMAT_ALIGNMENT = 3u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_LEFT: PARAFORMAT_ALIGNMENT = 1u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_RIGHT: PARAFORMAT_ALIGNMENT = 2u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type PARAFORMAT_BORDERS = u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_LEFT: PARAFORMAT_BORDERS = 1u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_RIGHT: PARAFORMAT_BORDERS = 2u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_TOP: PARAFORMAT_BORDERS = 4u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_BOTTOM: PARAFORMAT_BORDERS = 8u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_INSIDE: PARAFORMAT_BORDERS = 16u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_OUTSIDE: PARAFORMAT_BORDERS = 32u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_AUTOCOLOR: PARAFORMAT_BORDERS = 64u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type PARAFORMAT_MASK = u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_ALIGNMENT: PARAFORMAT_MASK = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_NUMBERING: PARAFORMAT_MASK = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_OFFSET: PARAFORMAT_MASK = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_OFFSETINDENT: PARAFORMAT_MASK = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_RIGHTINDENT: PARAFORMAT_MASK = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_RTLPARA: PARAFORMAT_MASK = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_STARTINDENT: PARAFORMAT_MASK = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_TABSTOPS: PARAFORMAT_MASK = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type PARAFORMAT_NUMBERING = u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFN_BULLET: PARAFORMAT_NUMBERING = 1u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFN_ARABIC: PARAFORMAT_NUMBERING = 2u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFN_LCLETTER: PARAFORMAT_NUMBERING = 3u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFN_UCLETTER: PARAFORMAT_NUMBERING = 4u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFN_LCROMAN: PARAFORMAT_NUMBERING = 5u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFN_UCROMAN: PARAFORMAT_NUMBERING = 6u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type PARAFORMAT_NUMBERING_STYLE = u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFNS_PAREN: PARAFORMAT_NUMBERING_STYLE = 0u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFNS_PARENS: PARAFORMAT_NUMBERING_STYLE = 256u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFNS_PERIOD: PARAFORMAT_NUMBERING_STYLE = 512u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFNS_PLAIN: PARAFORMAT_NUMBERING_STYLE = 768u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFNS_NONUMBER: PARAFORMAT_NUMBERING_STYLE = 1024u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFNS_NEWNUMBER: PARAFORMAT_NUMBERING_STYLE = 32768u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type PARAFORMAT_SHADING_STYLE = u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_NONE: PARAFORMAT_SHADING_STYLE = 0u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_HORIZ: PARAFORMAT_SHADING_STYLE = 1u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_VERT: PARAFORMAT_SHADING_STYLE = 2u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_DOWN_DIAG: PARAFORMAT_SHADING_STYLE = 3u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_UP_DIAG: PARAFORMAT_SHADING_STYLE = 4u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_GRID: PARAFORMAT_SHADING_STYLE = 5u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_TRELLIS: PARAFORMAT_SHADING_STYLE = 6u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_HORZ: PARAFORMAT_SHADING_STYLE = 7u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_VERT: PARAFORMAT_SHADING_STYLE = 8u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_DOWN_DIAG: PARAFORMAT_SHADING_STYLE = 9u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_UP_DIAG: PARAFORMAT_SHADING_STYLE = 10u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_GRID: PARAFORMAT_SHADING_STYLE = 11u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_TRELLIS: PARAFORMAT_SHADING_STYLE = 12u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type REOBJECT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_ALIGNTORIGHT: REOBJECT_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_BELOWBASELINE: REOBJECT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_BLANK: REOBJECT_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_CANROTATE: REOBJECT_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_DONTNEEDPALETTE: REOBJECT_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_DYNAMICSIZE: REOBJECT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_GETMETAFILE: REOBJECT_FLAGS = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_HILITED: REOBJECT_FLAGS = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_INPLACEACTIVE: REOBJECT_FLAGS = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_INVERTEDSELECT: REOBJECT_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_LINK: REOBJECT_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_LINKAVAILABLE: REOBJECT_FLAGS = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_OPEN: REOBJECT_FLAGS = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_OWNERDRAWSELECT: REOBJECT_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_RESIZABLE: REOBJECT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_SELECTED: REOBJECT_FLAGS = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_STATIC: REOBJECT_FLAGS = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_USEASBACKGROUND: REOBJECT_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_WRAPTEXTAROUND: REOBJECT_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SEL_EMPTY: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = 0u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SEL_TEXT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = 1u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SEL_OBJECT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = 2u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SEL_MULTICHAR: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = 4u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SEL_MULTIOBJECT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = 8u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCM_RIGHTMOUSEDROP: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = 32768u16;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type RICH_EDIT_GET_OBJECT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_GETOBJ_POLEOBJ: RICH_EDIT_GET_OBJECT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_GETOBJ_PSTG: RICH_EDIT_GET_OBJECT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_GETOBJ_POLESITE: RICH_EDIT_GET_OBJECT_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_GETOBJ_NO_INTERFACES: RICH_EDIT_GET_OBJECT_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_GETOBJ_ALL_INTERFACES: RICH_EDIT_GET_OBJECT_FLAGS = 7u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type TEXTMODE = i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TM_PLAINTEXT: TEXTMODE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TM_RICHTEXT: TEXTMODE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TM_SINGLELEVELUNDO: TEXTMODE = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TM_MULTILEVELUNDO: TEXTMODE = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TM_SINGLECODEPAGE: TEXTMODE = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TM_MULTICODEPAGE: TEXTMODE = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type TXTBACKSTYLE = i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBACK_TRANSPARENT: TXTBACKSTYLE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBACK_OPAQUE: TXTBACKSTYLE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type TXTHITRESULT = i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTHITRESULT_NOHIT: TXTHITRESULT = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTHITRESULT_TRANSPARENT: TXTHITRESULT = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTHITRESULT_CLOSE: TXTHITRESULT = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTHITRESULT_HIT: TXTHITRESULT = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type TXTNATURALSIZE = i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_FITTOCONTENT2: TXTNATURALSIZE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_FITTOCONTENT: TXTNATURALSIZE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_ROUNDTOLINE: TXTNATURALSIZE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_FITTOCONTENT3: TXTNATURALSIZE = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_FITTOCONTENTWSP: TXTNATURALSIZE = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_INCLUDELASTLINE: TXTNATURALSIZE = 1073741824i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_EMU: TXTNATURALSIZE = -2147483648i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type TXTVIEW = i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTVIEW_ACTIVE: TXTVIEW = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTVIEW_INACTIVE: TXTVIEW = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type UNDONAMEID = i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_UNKNOWN: UNDONAMEID = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_TYPING: UNDONAMEID = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_DELETE: UNDONAMEID = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_DRAGDROP: UNDONAMEID = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_CUT: UNDONAMEID = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_PASTE: UNDONAMEID = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_AUTOTABLE: UNDONAMEID = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type tomConstants = i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFalse: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTrue: tomConstants = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUndefined: tomConstants = -9999999i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomToggle: tomConstants = -9999998i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoColor: tomConstants = -9999997i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDefault: tomConstants = -9999996i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSuspend: tomConstants = -9999995i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomResume: tomConstants = -9999994i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomApplyNow: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomApplyLater: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTrackParms: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCacheParms: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomApplyTmp: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDisableSmartFont: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEnableSmartFont: tomConstants = 9i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUsePoints: tomConstants = 10i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUseTwips: tomConstants = 11i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBackward: tomConstants = -1073741823i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomForward: tomConstants = 1073741823i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMove: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomExtend: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoSelection: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionIP: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionNormal: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionFrame: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionColumn: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionRow: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionBlock: tomConstants = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionInlineShape: tomConstants = 7i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionShape: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelStartActive: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelAtEOL: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelOvertype: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelActive: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelReplace: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEnd: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStart: tomConstants = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCollapseEnd: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCollapseStart: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomClientCoord: tomConstants = 256i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAllowOffClient: tomConstants = 512i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTransform: tomConstants = 1024i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomObjectArg: tomConstants = 2048i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAtEnd: tomConstants = 4096i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNone: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSingle: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWords: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDouble: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDotted: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDash: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDashDot: tomConstants = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDashDotDot: tomConstants = 7i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWave: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThick: tomConstants = 9i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHair: tomConstants = 10i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDoubleWave: tomConstants = 11i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHeavyWave: tomConstants = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLongDash: tomConstants = 13i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThickDash: tomConstants = 14i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThickDashDot: tomConstants = 15i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThickDashDotDot: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThickDotted: tomConstants = 17i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThickLongDash: tomConstants = 18i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpaceSingle: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpace1pt5: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpaceDouble: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpaceAtLeast: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpaceExactly: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpaceMultiple: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpacePercent: tomConstants = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignLeft: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignCenter: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignRight: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignJustify: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignDecimal: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignBar: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDefaultTab: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignInterWord: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignNewspaper: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignInterLetter: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignScaled: tomConstants = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaces: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDots: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDashes: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLines: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThickLines: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEquals: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTabBack: tomConstants = -3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTabNext: tomConstants = -2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTabHere: tomConstants = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNone: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListBullet: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberAsArabic: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberAsLCLetter: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberAsUCLetter: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberAsLCRoman: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberAsUCRoman: tomConstants = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberAsSequence: tomConstants = 7i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedCircle: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedBlackCircleWingding: tomConstants = 9i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedWhiteCircleWingding: tomConstants = 10i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedArabicWide: tomConstants = 11i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedChS: tomConstants = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedChT: tomConstants = 13i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedJpnChS: tomConstants = 14i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedJpnKor: tomConstants = 15i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedArabic1: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedArabic2: tomConstants = 17i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedHebrew: tomConstants = 18i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedThaiAlpha: tomConstants = 19i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedThaiNum: tomConstants = 20i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedHindiAlpha: tomConstants = 21i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedHindiAlpha1: tomConstants = 22i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedHindiNum: tomConstants = 23i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListParentheses: tomConstants = 65536i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListPeriod: tomConstants = 131072i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListPlain: tomConstants = 196608i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNoNumber: tomConstants = 262144i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListMinus: tomConstants = 524288i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomIgnoreNumberStyle: tomConstants = 16777216i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleNormal: tomConstants = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading1: tomConstants = -2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading2: tomConstants = -3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading3: tomConstants = -4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading4: tomConstants = -5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading5: tomConstants = -6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading6: tomConstants = -7i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading7: tomConstants = -8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading8: tomConstants = -9i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading9: tomConstants = -10i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCharacter: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWord: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSentence: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParagraph: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLine: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStory: tomConstants = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomScreen: tomConstants = 7i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSection: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTableColumn: tomConstants = 9i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomColumn: tomConstants = 9i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRow: tomConstants = 10i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWindow: tomConstants = 11i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCell: tomConstants = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCharFormat: tomConstants = 13i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaFormat: tomConstants = 14i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTable: tomConstants = 15i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomObject: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPage: tomConstants = 17i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHardParagraph: tomConstants = 18i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCluster: tomConstants = 19i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomInlineObject: tomConstants = 20i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomInlineObjectArg: tomConstants = 21i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLeafLine: tomConstants = 22i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLayoutColumn: tomConstants = 23i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomProcessId: tomConstants = 1073741825i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchWord: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchCase: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchPattern: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnknownStory: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMainTextStory: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFootnotesStory: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEndnotesStory: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCommentsStory: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextFrameStory: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEvenPagesHeaderStory: tomConstants = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPrimaryHeaderStory: tomConstants = 7i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEvenPagesFooterStory: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPrimaryFooterStory: tomConstants = 9i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFirstPageHeaderStory: tomConstants = 10i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFirstPageFooterStory: tomConstants = 11i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomScratchStory: tomConstants = 127i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFindStory: tomConstants = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomReplaceStory: tomConstants = 129i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStoryInactive: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStoryActiveDisplay: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStoryActiveUI: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStoryActiveDisplayUI: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoAnimation: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLasVegasLights: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBlinkingBackground: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSparkleText: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMarchingBlackAnts: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMarchingRedAnts: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShimmer: tomConstants = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWipeDown: tomConstants = 7i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWipeRight: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAnimationMax: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLowerCase: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUpperCase: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTitleCase: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSentenceCase: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomToggleCase: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomReadOnly: tomConstants = 256i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShareDenyRead: tomConstants = 512i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShareDenyWrite: tomConstants = 1024i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPasteFile: tomConstants = 4096i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCreateNew: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCreateAlways: tomConstants = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOpenExisting: tomConstants = 48i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOpenAlways: tomConstants = 64i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTruncateExisting: tomConstants = 80i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRTF: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomText: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHTML: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWordDocument: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBold: tomConstants = -2147483647i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomItalic: tomConstants = -2147483646i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnderline: tomConstants = -2147483644i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStrikeout: tomConstants = -2147483640i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomProtected: tomConstants = -2147483632i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLink: tomConstants = -2147483616i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSmallCaps: tomConstants = -2147483584i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAllCaps: tomConstants = -2147483520i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHidden: tomConstants = -2147483392i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOutline: tomConstants = -2147483136i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShadow: tomConstants = -2147482624i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEmboss: tomConstants = -2147481600i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomImprint: tomConstants = -2147479552i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDisabled: tomConstants = -2147475456i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRevised: tomConstants = -2147467264i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSubscriptCF: tomConstants = -2147418112i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSuperscriptCF: tomConstants = -2147352576i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontBound: tomConstants = -2146435072i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLinkProtected: tomConstants = -2139095040i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomInlineObjectStart: tomConstants = -2130706432i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomExtendedChar: tomConstants = -2113929216i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoBackColor: tomConstants = -2080374784i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathZoneNoBuildUp: tomConstants = -2013265920i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathZone: tomConstants = -1879048192i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathZoneOrdinary: tomConstants = -1610612736i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoTextColor: tomConstants = -1073741824i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathZoneDisplay: tomConstants = 262144i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectRTL: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectKeep: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectKeepNext: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectPageBreakBefore: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectNoLineNumber: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectNoWidowControl: tomConstants = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectDoNotHyphen: tomConstants = 64i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectSideBySide: tomConstants = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectCollapsed: tomConstants = 256i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectOutlineLevel: tomConstants = 512i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectBox: tomConstants = 1024i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectTableRowDelimiter: tomConstants = 4096i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectTable: tomConstants = 16384i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomModWidthPairs: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomModWidthSpace: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoSpaceAlpha: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoSpaceNumeric: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoSpaceParens: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEmbeddedFont: tomConstants = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDoublestrike: tomConstants = 64i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOverlapping: tomConstants = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNormalCaret: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomKoreanBlockCaret: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNullCaret: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomIncludeInset: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnicodeBiDi: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathCFCheck: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnlink: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnhide: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCheckTextLimit: tomConstants = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomIgnoreCurrentFont: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchCharRep: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchFontSignature: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchAscii: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGetHeightOnly: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchMathFont: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCharset: tomConstants = -2147483648i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCharRepFromLcid: tomConstants = 1073741824i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAnsi: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEastEurope: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCyrillic: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGreek: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTurkish: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHebrew: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomArabic: tomConstants = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBaltic: tomConstants = 7i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomVietnamese: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDefaultCharRep: tomConstants = 9i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSymbol: tomConstants = 10i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThai: tomConstants = 11i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShiftJIS: tomConstants = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGB2312: tomConstants = 13i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHangul: tomConstants = 14i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBIG5: tomConstants = 15i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPC437: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOEM: tomConstants = 17i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMac: tomConstants = 18i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomArmenian: tomConstants = 19i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSyriac: tomConstants = 20i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThaana: tomConstants = 21i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDevanagari: tomConstants = 22i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBengali: tomConstants = 23i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGurmukhi: tomConstants = 24i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGujarati: tomConstants = 25i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOriya: tomConstants = 26i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTamil: tomConstants = 27i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTelugu: tomConstants = 28i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomKannada: tomConstants = 29i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMalayalam: tomConstants = 30i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSinhala: tomConstants = 31i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLao: tomConstants = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTibetan: tomConstants = 33i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMyanmar: tomConstants = 34i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGeorgian: tomConstants = 35i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomJamo: tomConstants = 36i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEthiopic: tomConstants = 37i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCherokee: tomConstants = 38i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAboriginal: tomConstants = 39i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOgham: tomConstants = 40i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRunic: tomConstants = 41i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomKhmer: tomConstants = 42i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMongolian: tomConstants = 43i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBraille: tomConstants = 44i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomYi: tomConstants = 45i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimbu: tomConstants = 46i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTaiLe: tomConstants = 47i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNewTaiLue: tomConstants = 48i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSylotiNagri: tomConstants = 49i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomKharoshthi: tomConstants = 50i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomKayahli: tomConstants = 51i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUsymbol: tomConstants = 52i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEmoji: tomConstants = 53i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGlagolitic: tomConstants = 54i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLisu: tomConstants = 55i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomVai: tomConstants = 56i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNKo: tomConstants = 57i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOsmanya: tomConstants = 58i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhagsPa: tomConstants = 59i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGothic: tomConstants = 60i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDeseret: tomConstants = 61i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTifinagh: tomConstants = 62i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCharRepMax: tomConstants = 63i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRE10Mode: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUseAtFont: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextFlowMask: tomConstants = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextFlowES: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextFlowSW: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextFlowWN: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextFlowNE: tomConstants = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoIME: tomConstants = 524288i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelfIME: tomConstants = 262144i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoUpScroll: tomConstants = 65536i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoVpScroll: tomConstants = 262144i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoLink: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomClientLink: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFriendlyLinkName: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFriendlyLinkAddress: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoLinkURL: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoLinkEmail: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoLinkPhone: tomConstants = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoLinkPath: tomConstants = 7i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCompressNone: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCompressPunctuation: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCompressPunctuationAndKana: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCompressMax: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnderlinePositionAuto: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnderlinePositionBelow: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnderlinePositionAbove: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnderlinePositionMax: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontAlignmentAuto: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontAlignmentTop: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontAlignmentBaseline: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontAlignmentBottom: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontAlignmentCenter: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontAlignmentMax: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRubyBelow: tomConstants = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRubyAlignCenter: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRubyAlign010: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRubyAlign121: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRubyAlignLeft: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRubyAlignRight: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitsDefault: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitsUnderOver: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitsSubSup: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUpperLimitAsSuperScript: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitsOpposite: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShowLLimPlaceHldr: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShowULimPlaceHldr: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDontGrowWithContent: tomConstants = 64i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGrowWithContent: tomConstants = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSubSupAlign: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitAlignMask: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitAlignCenter: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitAlignLeft: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitAlignRight: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShowDegPlaceHldr: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignDefault: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignMatchAscentDescent: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathVariant: tomConstants = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleDefault: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleScriptScriptCramped: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleScriptScript: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleScriptCramped: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleScript: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleTextCramped: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleText: tomConstants = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleDisplayCramped: tomConstants = 7i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleDisplay: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathRelSize: tomConstants = 64i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDecDecSize: tomConstants = 254i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDecSize: tomConstants = 255i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomIncSize: tomConstants = 65i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomIncIncSize: tomConstants = 66i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityUI: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityBack: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityFore: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityIn: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityOut: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityBackward: tomConstants = 536870912i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityForward: tomConstants = 1073741824i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAdjustCRLF: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUseCRLF: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextize: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAllowFinalEOP: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFoldMathAlpha: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoHidden: tomConstants = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomIncludeNumbering: tomConstants = 64i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTranslateTableCell: tomConstants = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoMathZoneBrackets: tomConstants = 256i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomConvertMathChar: tomConstants = 512i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoUCGreekItalic: tomConstants = 1024i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAllowMathBold: tomConstants = 2048i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLanguageTag: tomConstants = 4096i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomConvertRTF: tomConstants = 8192i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomApplyRtfDocProps: tomConstants = 16384i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomShow: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomZeroWidth: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomZeroAscent: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomZeroDescent: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomTransparent: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomASmash: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomDSmash: tomConstants = 9i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomHSmash: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomSmash: tomConstants = 13i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomHorz: tomConstants = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomVert: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxHideTop: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxHideBottom: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxHideLeft: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxHideRight: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxStrikeH: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxStrikeV: tomConstants = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxStrikeTLBR: tomConstants = 64i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxStrikeBLTR: tomConstants = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxAlignCenter: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceMask: tomConstants = 28i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceDefault: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceUnary: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceBinary: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceRelational: tomConstants = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceSkip: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceOrd: tomConstants = 20i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceDifferential: tomConstants = 24i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSizeText: tomConstants = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSizeScript: tomConstants = 64i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSizeScriptScript: tomConstants = 96i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoBreak: tomConstants = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTransparentForPositioning: tomConstants = 256i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTransparentForSpacing: tomConstants = 512i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStretchCharBelow: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStretchCharAbove: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStretchBaseBelow: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStretchBaseAbove: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatrixAlignMask: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatrixAlignCenter: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatrixAlignTopRow: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatrixAlignBottomRow: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShowMatPlaceHldr: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEqArrayLayoutWidth: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEqArrayAlignMask: tomConstants = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEqArrayAlignCenter: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEqArrayAlignTopRow: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEqArrayAlignBottomRow: tomConstants = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathManualBreakMask: tomConstants = 127i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBreakLeft: tomConstants = 125i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBreakCenter: tomConstants = 126i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBreakRight: tomConstants = 127i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathEqAlign: tomConstants = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathArgShadingStart: tomConstants = 593i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathArgShadingEnd: tomConstants = 594i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathObjShadingStart: tomConstants = 595i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathObjShadingEnd: tomConstants = 596i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFunctionTypeNone: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFunctionTypeTakesArg: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFunctionTypeTakesLim: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFunctionTypeTakesLim2: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFunctionTypeIsLim: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathParaAlignDefault: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathParaAlignCenterGroup: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathParaAlignCenter: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathParaAlignLeft: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathParaAlignRight: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispAlignMask: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispAlignCenterGroup: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispAlignCenter: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispAlignLeft: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispAlignRight: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispIntUnderOver: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispFracTeX: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispNaryGrow: tomConstants = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocEmptyArgMask: tomConstants = 96i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocEmptyArgAuto: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocEmptyArgAlways: tomConstants = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocEmptyArgNever: tomConstants = 64i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocSbSpOpUnchanged: tomConstants = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocDiffMask: tomConstants = 768i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocDiffDefault: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocDiffUpright: tomConstants = 256i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocDiffItalic: tomConstants = 512i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocDiffOpenItalic: tomConstants = 768i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispNarySubSup: tomConstants = 1024i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispDef: tomConstants = 2048i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathEnableRtl: tomConstants = 4096i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinMask: tomConstants = 196608i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinBefore: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinAfter: tomConstants = 65536i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinDup: tomConstants = 131072i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinSubMask: tomConstants = 786432i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinSubMM: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinSubPM: tomConstants = 262144i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinSubMP: tomConstants = 524288i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelRange: tomConstants = 597i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHstring: tomConstants = 596i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontPropTeXStyle: tomConstants = 828i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontPropAlign: tomConstants = 829i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretch: tomConstants = 830i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStyle: tomConstants = 831i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStyleUpright: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStyleOblique: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStyleItalic: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchDefault: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchUltraCondensed: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchExtraCondensed: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchCondensed: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchSemiCondensed: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchNormal: tomConstants = 5i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchSemiExpanded: tomConstants = 6i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchExpanded: tomConstants = 7i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchExtraExpanded: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchUltraExpanded: tomConstants = 9i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightDefault: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightThin: tomConstants = 100i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightExtraLight: tomConstants = 200i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightLight: tomConstants = 300i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightNormal: tomConstants = 400i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightRegular: tomConstants = 400i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightMedium: tomConstants = 500i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightSemiBold: tomConstants = 600i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightBold: tomConstants = 700i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightExtraBold: tomConstants = 800i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightBlack: tomConstants = 900i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightHeavy: tomConstants = 900i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightExtraBlack: tomConstants = 950i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaPropMathAlign: tomConstants = 1079i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDocMathBuild: tomConstants = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathLMargin: tomConstants = 129i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathRMargin: tomConstants = 130i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathWrapIndent: tomConstants = 131i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathWrapRight: tomConstants = 132i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathPostSpace: tomConstants = 134i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathPreSpace: tomConstants = 133i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathInterSpace: tomConstants = 135i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathIntraSpace: tomConstants = 136i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCanCopy: tomConstants = 137i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCanRedo: tomConstants = 138i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCanUndo: tomConstants = 139i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUndoLimit: tomConstants = 140i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDocAutoLink: tomConstants = 141i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEllipsisMode: tomConstants = 142i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEllipsisState: tomConstants = 143i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEllipsisNone: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEllipsisEnd: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEllipsisWord: tomConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEllipsisPresent: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomVTopCell: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomVLowCell: tomConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHStartCell: tomConstants = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHContCell: tomConstants = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRowUpdate: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRowApplyDefault: tomConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCellStructureChangeOnly: tomConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRowHeightActual: tomConstants = 2059i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct BIDIOPTIONS {
    pub cbSize: u32,
    pub wMask: u16,
    pub wEffects: u16,
}
impl ::core::marker::Copy for BIDIOPTIONS {}
impl ::core::clone::Clone for BIDIOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub union CARET_INFO {
    pub hbitmap: super::super::super::Graphics::Gdi::HBITMAP,
    pub caretFlags: CARET_FLAGS,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for CARET_INFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for CARET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct CHANGENOTIFY {
    pub dwChangeType: CHANGETYPE,
    pub pvCookieData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CHANGENOTIFY {}
impl ::core::clone::Clone for CHANGENOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHARFORMAT2A {
    pub Base: CHARFORMATA,
    pub wWeight: u16,
    pub sSpacing: i16,
    pub crBackColor: super::super::super::Foundation::COLORREF,
    pub lcid: u32,
    pub Anonymous: CHARFORMAT2A_0,
    pub sStyle: i16,
    pub wKerning: u16,
    pub bUnderlineType: u8,
    pub bAnimation: u8,
    pub bRevAuthor: u8,
    pub bUnderlineColor: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHARFORMAT2A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHARFORMAT2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union CHARFORMAT2A_0 {
    pub dwReserved: u32,
    pub dwCookie: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHARFORMAT2A_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHARFORMAT2A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHARFORMAT2W {
    pub Base: CHARFORMATW,
    pub wWeight: u16,
    pub sSpacing: i16,
    pub crBackColor: super::super::super::Foundation::COLORREF,
    pub lcid: u32,
    pub Anonymous: CHARFORMAT2W_0,
    pub sStyle: i16,
    pub wKerning: u16,
    pub bUnderlineType: u8,
    pub bAnimation: u8,
    pub bRevAuthor: u8,
    pub bUnderlineColor: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHARFORMAT2W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHARFORMAT2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union CHARFORMAT2W_0 {
    pub dwReserved: u32,
    pub dwCookie: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHARFORMAT2W_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHARFORMAT2W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHARFORMATA {
    pub cbSize: u32,
    pub dwMask: CFM_MASK,
    pub dwEffects: CFE_EFFECTS,
    pub yHeight: i32,
    pub yOffset: i32,
    pub crTextColor: super::super::super::Foundation::COLORREF,
    pub bCharSet: super::super::super::Graphics::Gdi::FONT_CHARSET,
    pub bPitchAndFamily: u8,
    pub szFaceName: [u8; 32],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHARFORMATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHARFORMATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHARFORMATW {
    pub cbSize: u32,
    pub dwMask: CFM_MASK,
    pub dwEffects: CFE_EFFECTS,
    pub yHeight: i32,
    pub yOffset: i32,
    pub crTextColor: super::super::super::Foundation::COLORREF,
    pub bCharSet: super::super::super::Graphics::Gdi::FONT_CHARSET,
    pub bPitchAndFamily: u8,
    pub szFaceName: [u16; 32],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHARFORMATW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHARFORMATW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct CHARRANGE {
    pub cpMin: i32,
    pub cpMax: i32,
}
impl ::core::marker::Copy for CHARRANGE {}
impl ::core::clone::Clone for CHARRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLIPBOARDFORMAT {
    pub nmhdr: super::NMHDR,
    pub cf: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLIPBOARDFORMAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLIPBOARDFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMPCOLOR {
    pub crText: super::super::super::Foundation::COLORREF,
    pub crBackground: super::super::super::Foundation::COLORREF,
    pub dwEffects: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMPCOLOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMPCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct EDITSTREAM {
    pub dwCookie: usize,
    pub dwError: u32,
    pub pfnCallback: EDITSTREAMCALLBACK,
}
impl ::core::marker::Copy for EDITSTREAM {}
impl ::core::clone::Clone for EDITSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENCORRECTTEXT {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENCORRECTTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENCORRECTTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENDCOMPOSITIONNOTIFY {
    pub nmhdr: super::NMHDR,
    pub dwCode: ENDCOMPOSITIONNOTIFY_CODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENDCOMPOSITIONNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENDCOMPOSITIONNOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENDROPFILES {
    pub nmhdr: super::NMHDR,
    pub hDrop: super::super::super::Foundation::HANDLE,
    pub cp: i32,
    pub fProtected: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENDROPFILES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENDROPFILES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENLINK {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENLINK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENLOWFIRTF {
    pub nmhdr: super::NMHDR,
    pub szControl: ::windows_sys::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENLOWFIRTF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENLOWFIRTF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENOLEOPFAILED {
    pub nmhdr: super::NMHDR,
    pub iob: i32,
    pub lOper: i32,
    pub hr: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENOLEOPFAILED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENOLEOPFAILED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENPROTECTED {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENPROTECTED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENPROTECTED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENSAVECLIPBOARD {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub cch: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENSAVECLIPBOARD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENSAVECLIPBOARD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct FINDTEXTA {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_sys::core::PCSTR,
}
impl ::core::marker::Copy for FINDTEXTA {}
impl ::core::clone::Clone for FINDTEXTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct FINDTEXTEXA {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_sys::core::PCSTR,
    pub chrgText: CHARRANGE,
}
impl ::core::marker::Copy for FINDTEXTEXA {}
impl ::core::clone::Clone for FINDTEXTEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct FINDTEXTEXW {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_sys::core::PCWSTR,
    pub chrgText: CHARRANGE,
}
impl ::core::marker::Copy for FINDTEXTEXW {}
impl ::core::clone::Clone for FINDTEXTEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct FINDTEXTW {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for FINDTEXTW {}
impl ::core::clone::Clone for FINDTEXTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct FORMATRANGE {
    pub hdc: super::super::super::Graphics::Gdi::HDC,
    pub hdcTarget: super::super::super::Graphics::Gdi::HDC,
    pub rc: super::super::super::Foundation::RECT,
    pub rcPage: super::super::super::Foundation::RECT,
    pub chrg: CHARRANGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for FORMATRANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for FORMATRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GETCONTEXTMENUEX {
    pub chrg: CHARRANGE,
    pub dwFlags: u32,
    pub pt: super::super::super::Foundation::POINT,
    pub pvReserved: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GETCONTEXTMENUEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GETCONTEXTMENUEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct GETTEXTEX {
    pub cb: u32,
    pub flags: GETTEXTEX_FLAGS,
    pub codepage: u32,
    pub lpDefaultChar: ::windows_sys::core::PCSTR,
    pub lpUsedDefChar: *mut i32,
}
impl ::core::marker::Copy for GETTEXTEX {}
impl ::core::clone::Clone for GETTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct GETTEXTLENGTHEX {
    pub flags: GETTEXTLENGTHEX_FLAGS,
    pub codepage: u32,
}
impl ::core::marker::Copy for GETTEXTLENGTHEX {}
impl ::core::clone::Clone for GETTEXTLENGTHEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUPTYPINGCHANGE {
    pub nmhdr: super::NMHDR,
    pub fGroupTyping: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUPTYPINGCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUPTYPINGCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct HYPHENATEINFO {
    pub cbSize: i16,
    pub dxHyphenateZone: i16,
    pub pfnHyphenate: isize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for HYPHENATEINFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for HYPHENATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(target_arch = "x86")]
pub struct HYPHENATEINFO {
    pub cbSize: i16,
    pub dxHyphenateZone: i16,
    pub pfnHyphenate: isize,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for HYPHENATEINFO {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for HYPHENATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct HYPHRESULT {
    pub khyph: KHYPH,
    pub ichHyph: i32,
    pub chHyph: u16,
}
impl ::core::marker::Copy for HYPHRESULT {}
impl ::core::clone::Clone for HYPHRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct IMECOMPTEXT {
    pub cb: i32,
    pub flags: IMECOMPTEXT_FLAGS,
}
impl ::core::marker::Copy for IMECOMPTEXT {}
impl ::core::clone::Clone for IMECOMPTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSGFILTER {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSGFILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSGFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OBJECTPOSITIONS {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub pcpPositions: *mut i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OBJECTPOSITIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OBJECTPOSITIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct PARAFORMAT {
    pub cbSize: u32,
    pub dwMask: PARAFORMAT_MASK,
    pub wNumbering: PARAFORMAT_NUMBERING,
    pub Anonymous: PARAFORMAT_0,
    pub dxStartIndent: i32,
    pub dxRightIndent: i32,
    pub dxOffset: i32,
    pub wAlignment: PARAFORMAT_ALIGNMENT,
    pub cTabCount: i16,
    pub rgxTabs: [u32; 32],
}
impl ::core::marker::Copy for PARAFORMAT {}
impl ::core::clone::Clone for PARAFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub union PARAFORMAT_0 {
    pub wReserved: u16,
    pub wEffects: u16,
}
impl ::core::marker::Copy for PARAFORMAT_0 {}
impl ::core::clone::Clone for PARAFORMAT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct PARAFORMAT2 {
    pub Base: PARAFORMAT,
    pub dySpaceBefore: i32,
    pub dySpaceAfter: i32,
    pub dyLineSpacing: i32,
    pub sStyle: i16,
    pub bLineSpacingRule: u8,
    pub bOutlineLevel: u8,
    pub wShadingWeight: u16,
    pub wShadingStyle: PARAFORMAT_SHADING_STYLE,
    pub wNumberingStart: u16,
    pub wNumberingStyle: PARAFORMAT_NUMBERING_STYLE,
    pub wNumberingTab: u16,
    pub wBorderSpace: u16,
    pub wBorderWidth: u16,
    pub wBorders: PARAFORMAT_BORDERS,
}
impl ::core::marker::Copy for PARAFORMAT2 {}
impl ::core::clone::Clone for PARAFORMAT2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct PUNCTUATION {
    pub iSize: u32,
    pub szPunctuation: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for PUNCTUATION {}
impl ::core::clone::Clone for PUNCTUATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub struct REOBJECT {
    pub cbStruct: u32,
    pub cp: i32,
    pub clsid: ::windows_sys::core::GUID,
    pub poleobj: super::super::super::System::Ole::IOleObject,
    pub pstg: super::super::super::System::Com::StructuredStorage::IStorage,
    pub polesite: super::super::super::System::Ole::IOleClientSite,
    pub sizel: super::super::super::Foundation::SIZE,
    pub dvaspect: u32,
    pub dwFlags: REOBJECT_FLAGS,
    pub dwUser: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for REOBJECT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for REOBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct REPASTESPECIAL {
    pub dwAspect: super::super::super::System::Com::DVASPECT,
    pub dwParam: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for REPASTESPECIAL {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for REPASTESPECIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REQRESIZE {
    pub nmhdr: super::NMHDR,
    pub rc: super::super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REQRESIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REQRESIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub struct RICHEDIT_IMAGE_PARAMETERS {
    pub xWidth: i32,
    pub yHeight: i32,
    pub Ascent: i32,
    pub Type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS,
    pub pwszAlternateText: ::windows_sys::core::PCWSTR,
    pub pIStream: super::super::super::System::Com::IStream,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for RICHEDIT_IMAGE_PARAMETERS {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for RICHEDIT_IMAGE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SELCHANGE {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SELCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SELCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct SETTEXTEX {
    pub flags: u32,
    pub codepage: u32,
}
impl ::core::marker::Copy for SETTEXTEX {}
impl ::core::clone::Clone for SETTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TABLECELLPARMS {
    pub dxWidth: i32,
    pub _bitfield: u16,
    pub wShading: u16,
    pub dxBrdrLeft: i16,
    pub dyBrdrTop: i16,
    pub dxBrdrRight: i16,
    pub dyBrdrBottom: i16,
    pub crBrdrLeft: super::super::super::Foundation::COLORREF,
    pub crBrdrTop: super::super::super::Foundation::COLORREF,
    pub crBrdrRight: super::super::super::Foundation::COLORREF,
    pub crBrdrBottom: super::super::super::Foundation::COLORREF,
    pub crBackPat: super::super::super::Foundation::COLORREF,
    pub crForePat: super::super::super::Foundation::COLORREF,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TABLECELLPARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TABLECELLPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct TABLEROWPARMS {
    pub cbRow: u8,
    pub cbCell: u8,
    pub cCell: u8,
    pub cRow: u8,
    pub dxCellMargin: i32,
    pub dxIndent: i32,
    pub dyHeight: i32,
    pub _bitfield: u32,
    pub cpStartRow: i32,
    pub bTableLevel: u8,
    pub iCell: u8,
}
impl ::core::marker::Copy for TABLEROWPARMS {}
impl ::core::clone::Clone for TABLEROWPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct TEXTRANGEA {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for TEXTRANGEA {}
impl ::core::clone::Clone for TEXTRANGEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct TEXTRANGEW {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for TEXTRANGEW {}
impl ::core::clone::Clone for TEXTRANGEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type AutoCorrectProc = ::core::option::Option<unsafe extern "system" fn(langid: u16, pszbefore: ::windows_sys::core::PCWSTR, pszafter: ::windows_sys::core::PCWSTR, cchafter: i32, pcchreplaced: *mut i32) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type EDITSTREAMCALLBACK = ::core::option::Option<unsafe extern "system" fn(dwcookie: usize, pbbuff: *mut u8, cb: i32, pcb: *mut i32) -> u32>;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type EDITWORDBREAKPROCEX = ::core::option::Option<unsafe extern "system" fn(pchtext: ::windows_sys::core::PCSTR, cchtext: i32, bcharset: u8, action: i32) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type PCreateTextServices = ::core::option::Option<unsafe extern "system" fn(punkouter: ::windows_sys::core::IUnknown, pitexthost: ITextHost, ppunk: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type PShutdownTextServices = ::core::option::Option<unsafe extern "system" fn(ptextservices: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT>;
