use crate::names;

use proc_macro2::TokenStream;
use quote::quote;
use witx::Layout;

pub(super) fn define_handle(name: &witx::Id, h: &witx::HandleDatatype) -> TokenStream {
    let ident = names::type_(name);
    let size = h.mem_size_align().size as u32;
    let align = h.mem_size_align().align;
    quote! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Debug, ::std::hash::Hash, Eq, PartialEq)]
        pub struct #ident(u32);

        impl #ident {
            #[inline]
            pub unsafe fn inner(&self) -> u32 {
                self.0
            }
        }

        impl From<#ident> for u32 {
            #[inline]
            fn from(e: #ident) -> u32 {
                e.0
            }
        }

        impl From<#ident> for i32 {
            #[inline]
            fn from(e: #ident) -> i32 {
                e.0 as i32
            }
        }

        impl From<u32> for #ident {
            #[inline]
            fn from(e: u32) -> #ident {
                #ident(e)
            }
        }
        impl From<i32> for #ident {
            #[inline]
            fn from(e: i32) -> #ident {
                #ident(e as u32)
            }
        }

        impl ::std::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}({})", stringify!(#ident), self.0)
            }
        }

        impl wiggle::GuestType for #ident {
            #[inline]
            fn guest_size() -> u32 {
                #size
            }

            #[inline]
            fn guest_align() -> usize {
                #align
            }

            #[inline]
            fn read(mem: &wiggle::GuestMemory, location: wiggle::GuestPtr<#ident>) -> Result<#ident, wiggle::GuestError> {
                Ok(#ident(u32::read(mem, location.cast())?))
            }

            #[inline]
            fn write(mem: &mut wiggle::GuestMemory, location: wiggle::GuestPtr<Self>, val: Self) -> Result<(), wiggle::GuestError> {
                u32::write(mem, location.cast(), val.0)
            }
        }
    }
}

impl super::WiggleType for witx::HandleDatatype {
    fn impls_display(&self) -> bool {
        true
    }
}
