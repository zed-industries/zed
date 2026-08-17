use link_section::declarative::{section, in_section};
use link_section::TypedSection;




#[allow(non_camel_case_types)]
struct FOO;
impl FOO {
    /// Get a `const` reference to the underlying section. In
    /// non-const contexts, `deref` is sufficient.
    pub const fn const_deref(&self) -> &'static TypedSection<fn()> {
        static SECTION: TypedSection<fn()> =
            {
                let section =
                    {
                        static __LINK_SECTION_NAME: &'static str =
                            ".data.link_section.FOO";
                        #[export_name = ".data.link_section.FOO.bounds"]
                        #[used]
                        #[used]
                        static __LINK_SECTION_INFO:
                            ::link_section::__support::wasm::LinkSectionInfoLock<::link_section::__support::wasm::LinkSectionInfo>
                            =
                            ::link_section::__support::wasm::LinkSectionInfoLock::new(::link_section::__support::wasm::LinkSectionInfo::new::<(fn())>(__LINK_SECTION_NAME,
                                    false));
                        #[link_section = ".init_array.1"]
                        #[used]
                        #[allow(non_snake_case)]
                        static __LINK_SECTION_FLATTEN_FN_REF: extern "C" fn() =
                            {
                                extern "C" fn __LINK_SECTION_FLATTEN_FN() {
                                    unsafe {
                                        ::link_section::__support::wasm::flatten(&raw const __LINK_SECTION_INFO);
                                    }
                                }
                                __LINK_SECTION_FLATTEN_FN
                            };
                        unsafe {
                            <::link_section::__support::Bounds>::new(&raw const __LINK_SECTION_INFO)
                        }
                    };
                let name = ".data.link_section.FOO";
                ::link_section::__support::validate_section_name(name);
                unsafe { <TypedSection<fn()>>::new(name, section) }
            };
        &SECTION
    }
}
impl ::core::fmt::Debug for FOO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::ops::Deref::deref(self).fmt(f)
    }
}
impl ::core::ops::Deref for FOO {
    type Target = TypedSection<fn()>;
    fn deref(&self) -> &Self::Target { self.const_deref() }
}
impl ::link_section::__support::SectionItemType for FOO {
    type Item = (fn());
}
impl ::link_section::__support::SectionItemTyped<(fn())> for FOO {
    type Item = (fn());
}
impl FOO {
    /// Get the section as a slice.
    pub fn as_slice(&self) -> &[(fn())] { self.const_deref().as_slice() }
}
impl ::core::iter::IntoIterator for FOO {
    type Item = &'static (fn());
    type IntoIter = ::core::slice::Iter<'static, (fn())>;
    fn into_iter(self) -> Self::IntoIter {
        self.const_deref().as_slice().iter()
    }
}
fn foo() {
    const _: fn() =
        const {
                type __InSecStoredTy =
                    <::link_section::TypedSection<fn()> as
                    ::link_section::__support::SectionItemType>::Item;
                const __LINK_SECTION_CONST_ITEM_VALUE: __InSecStoredTy = foo;
                #[used]
                static __LINK_SECTION_CELL:
                    ::link_section::__support::wasm::LinkCell<__InSecStoredTy,
                    ::link_section::__support::wasm::LinkMeta> =
                    <::link_section::__support::wasm::LinkCell<__InSecStoredTy,
                            ::link_section::__support::wasm::LinkMeta>>::new(__LINK_SECTION_CONST_ITEM_VALUE);
                #[allow(missing_unsafe_on_extern)]
                extern "C" {
                    #[link_name = ".data.link_section.FOO.bounds"]
                    static __LINK_SECTION_INFO:
                        ::link_section::__support::wasm::LinkSectionInfoLock<::link_section::__support::wasm::LinkSectionInfo>;
                }
                #[link_section = ".init_array.0"]
                #[used]
                #[allow(non_snake_case)]
                static __LINK_SECTION_ITEM_FN_REF: extern "C" fn() =
                    {
                        extern "C" fn __LINK_SECTION_ITEM_FN() {
                            unsafe {
                                ::link_section::__support::wasm::register(&raw const __LINK_SECTION_INFO,
                                    __LINK_SECTION_CELL.as_cell_ptr());
                            }
                        }
                        __LINK_SECTION_ITEM_FN
                    };
                __LINK_SECTION_CONST_ITEM_VALUE
            };
}
fn main() {}
