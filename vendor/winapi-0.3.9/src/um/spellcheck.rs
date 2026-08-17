use shared::minwindef::{BOOL, BYTE, DWORD};
use shared::ntdef::{LPCWSTR, LPWSTR, ULONG};
use shared::winerror::HRESULT;
use um::objidlbase::IEnumString;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
ENUM!{enum WORDLIST_TYPE {
    WORDLIST_TYPE_IGNORE = 0,
    WORDLIST_TYPE_ADD = 1,
    WORDLIST_TYPE_EXCLUDE = 2,
    WORDLIST_TYPE_AUTOCORRECT = 3,
}}
ENUM!{enum CORRECTIVE_ACTION {
    CORRECTIVE_ACTION_NONE = 0,
    CORRECTIVE_ACTION_GET_SUGGESTIONS = 1,
    CORRECTIVE_ACTION_REPLACE = 2,
    CORRECTIVE_ACTION_DELETE = 3,
}}
RIDL!{#[uuid(0xb7c82d61, 0xfbe8, 0x4b47, 0x9b, 0x27, 0x6c, 0x0d, 0x2e, 0x0d, 0xe0, 0xa3)]
interface ISpellingError(ISpellingErrorVtbl): IUnknown(IUnknownVtbl) {
    fn get_StartIndex(
        value: *mut ULONG,
    ) -> HRESULT,
    fn get_Length(
        value: *mut ULONG,
    ) -> HRESULT,
    fn get_CorrectiveAction(
        value: *mut CORRECTIVE_ACTION,
    ) -> HRESULT,
    fn get_Replacement(
        value: *mut LPWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x803e3bd4, 0x2828, 0x4410, 0x82, 0x90, 0x41, 0x8d, 0x1d, 0x73, 0xc7, 0x62)]
interface IEnumSpellingError(IEnumSpellingErrorVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        value: *mut *mut ISpellingError,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x432e5f85, 0x35cf, 0x4606, 0xa8, 0x01, 0x6f, 0x70, 0x27, 0x7e, 0x1d, 0x7a)]
interface IOptionDescription(IOptionDescriptionVtbl): IUnknown(IUnknownVtbl) {
    fn Id(
        value: *mut LPWSTR,
    ) -> HRESULT,
    fn Heading(
        value: *mut LPWSTR,
    ) -> HRESULT,
    fn Description(
        value: *mut LPWSTR,
    ) -> HRESULT,
    fn Labels(
        value: *mut *mut IEnumString,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0b83a5b0, 0x792f, 0x4eab, 0x97, 0x99, 0xac, 0xf5, 0x2c, 0x5e, 0xd0, 0x8a)]
interface ISpellCheckerChangedEventHandler(ISpellCheckerChangedEventHandlerVtbl):
    IUnknown(IUnknownVtbl) {
    fn Invoke(
        sender: *const ISpellChecker,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb6fd0b71, 0xe2bc, 0x4653, 0x8d, 0x05, 0xf1, 0x97, 0xe4, 0x12, 0x77, 0x0b)]
interface ISpellChecker(ISpellCheckerVtbl): IUnknown(IUnknownVtbl) {
    fn get_LanguageTag(
        value: *mut LPWSTR,
    ) -> HRESULT,
    fn Check(
        text: LPCWSTR,
        value: *mut *mut IEnumSpellingError,
    ) -> HRESULT,
    fn Suggest(
        word: LPCWSTR,
        value: *mut *mut IEnumString,
    ) -> HRESULT,
    fn Add(
        word: LPCWSTR,
    ) -> HRESULT,
    fn Ignore(
        word: LPCWSTR,
    ) -> HRESULT,
    fn AutoCorrect(
        from: LPCWSTR,
        to: LPCWSTR,
    ) -> HRESULT,
    fn GetOptionValue(
        optionId: LPCWSTR,
        value: *mut BYTE,
    ) -> HRESULT,
    fn Get_OptionIds(
        value: *mut *mut IEnumString,
    ) -> HRESULT,
    fn Get_Id(
        value: *mut LPWSTR,
    ) -> HRESULT,
    fn Get_LocalizedName(
        value: *mut LPWSTR,
    ) -> HRESULT,
    fn add_SpellCheckerChanged(
        handler: *const ISpellCheckerChangedEventHandler,
        eventCookie: *mut DWORD,
    ) -> HRESULT,
    fn remove_SpellCheckerChanged(
        eventCookie: DWORD,
    ) -> HRESULT,
    fn GetOptionDescription(
        optionId: LPCWSTR,
        value: *mut *mut IOptionDescription,
    ) -> HRESULT,
    fn ComprehensiveCheck(
        text: LPCWSTR,
        value: *mut *mut IEnumSpellingError,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe7ed1c71, 0x87f7, 0x4378, 0xa8, 0x40, 0xc9, 0x20, 0x0d, 0xac, 0xee, 0x47)]
interface ISpellChecker2(ISpellChecker2Vtbl): ISpellChecker(ISpellCheckerVtbl) {
    fn Remove(
        word: LPCWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8e018a9d, 0x2415, 0x4677, 0xbf, 0x08, 0x79, 0x4e, 0xa6, 0x1f, 0x94, 0xbb)]
interface ISpellCheckerFactory(ISpellCheckerFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn SupportedLanguages(
       value: *mut *mut IEnumString,
    ) -> HRESULT,
    fn IsSupported(
        languageTag: LPCWSTR,
        value: *mut BOOL,
    ) -> HRESULT,
    fn CreateSpellChecker(
        languageTag: LPCWSTR,
        value: *mut *mut ISpellChecker,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xaa176b85, 0x0e12, 0x4844, 0x8e, 0x1a, 0xee, 0xf1, 0xda, 0x77, 0xf5, 0x86)]
interface IUserDictionariesRegistrar(IUserDictionariesRegistrarVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterUserDictionary(
        dictionaryPath: LPCWSTR,
        languageTag: LPCWSTR,
    ) -> HRESULT,
    fn UnregisterUserDictionary(
        dictionaryPath: LPCWSTR,
        languageTag: LPCWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7ab36653, 0x1796, 0x484b, 0xbd, 0xfa, 0xe7, 0x4f, 0x1d, 0xb7, 0xc1, 0xdc)]
class SpellCheckerFactory;
}
