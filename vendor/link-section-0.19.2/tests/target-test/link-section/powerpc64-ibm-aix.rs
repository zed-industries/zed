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
                        ;
                        ::link_section::__support::PtrBounds::new({
                                #[allow(missing_unsafe_on_extern)]
                                extern "C" {
                                    #[link_name = "__start__data_link_section_FOO"]
                                    static __SYMBOL: u8;
                                }
                                unsafe { &raw const __SYMBOL as *const () }
                            },
                            {
                                #[allow(missing_unsafe_on_extern)]
                                extern "C" {
                                    #[link_name = "__stop__data_link_section_FOO"]
                                    static __SYMBOL: u8;
                                }
                                unsafe { &raw const __SYMBOL as *const () }
                            })
                    };
                let name = "_data_link_section_FOO";
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
                #[export_name =
                "__LINK_SECTION_CONST_ITEM_Pexpand_probe_Mexpand_probe_L11_C1"]
                #[link_section = "_data_link_section_FOO"]
                static __LINK_SECTION_CONST_ITEM: __InSecStoredTy =
                    __LINK_SECTION_CONST_ITEM_VALUE;
                __LINK_SECTION_CONST_ITEM_VALUE
            };
}
fn main() {}
