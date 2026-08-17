use wasmtime::component::Val;

use crate::wasm_name_t;

use std::mem;
use std::mem::MaybeUninit;
use std::ptr;
use std::slice;

crate::declare_vecs! {
    (
        name: wasmtime_component_vallist_t,
        ty: wasmtime_component_val_t,
        new: wasmtime_component_vallist_new,
        empty: wasmtime_component_vallist_new_empty,
        uninit: wasmtime_component_vallist_new_uninit,
        copy: wasmtime_component_vallist_copy,
        delete: wasmtime_component_vallist_delete,
    )
    (
        name: wasmtime_component_valrecord_t,
        ty: wasmtime_component_valrecord_entry_t,
        new: wasmtime_component_valrecord_new,
        empty: wasmtime_component_valrecord_new_empty,
        uninit: wasmtime_component_valrecord_new_uninit,
        copy: wasmtime_component_valrecord_copy,
        delete: wasmtime_component_valrecord_delete,
    )
    (
        name: wasmtime_component_valtuple_t,
        ty: wasmtime_component_val_t,
        new: wasmtime_component_valtuple_new,
        empty: wasmtime_component_valtuple_new_empty,
        uninit: wasmtime_component_valtuple_new_uninit,
        copy: wasmtime_component_valtuple_copy,
        delete: wasmtime_component_valtuple_delete,
    )
    (
        name: wasmtime_component_valflags_t,
        ty: wasm_name_t,
        new: wasmtime_component_valflags_new,
        empty: wasmtime_component_valflags_new_empty,
        uninit: wasmtime_component_valflags_new_uninit,
        copy: wasmtime_component_valflags_copy,
        delete: wasmtime_component_valflags_delete,
    )
}

impl From<&wasmtime_component_vallist_t> for Vec<Val> {
    fn from(value: &wasmtime_component_vallist_t) -> Self {
        value.as_slice().iter().map(Val::from).collect()
    }
}

impl From<&[Val]> for wasmtime_component_vallist_t {
    fn from(value: &[Val]) -> Self {
        value
            .iter()
            .map(wasmtime_component_val_t::from)
            .collect::<Vec<_>>()
            .into()
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct wasmtime_component_valrecord_entry_t {
    name: wasm_name_t,
    val: wasmtime_component_val_t,
}

impl Default for wasmtime_component_valrecord_entry_t {
    fn default() -> Self {
        Self {
            name: wasm_name_t::from_name(String::new()),
            val: Default::default(),
        }
    }
}

impl From<&wasmtime_component_valrecord_entry_t> for (String, Val) {
    fn from(value: &wasmtime_component_valrecord_entry_t) -> Self {
        (
            String::from_utf8(value.name.clone().take()).unwrap(),
            Val::from(&value.val),
        )
    }
}

impl From<&(String, Val)> for wasmtime_component_valrecord_entry_t {
    fn from((name, val): &(String, Val)) -> Self {
        Self {
            name: wasm_name_t::from_name(name.clone()),
            val: wasmtime_component_val_t::from(val),
        }
    }
}

impl From<&wasmtime_component_valrecord_t> for Vec<(String, Val)> {
    fn from(value: &wasmtime_component_valrecord_t) -> Self {
        value.as_slice().iter().map(Into::into).collect()
    }
}

impl From<&[(String, Val)]> for wasmtime_component_valrecord_t {
    fn from(value: &[(String, Val)]) -> Self {
        value
            .iter()
            .map(wasmtime_component_valrecord_entry_t::from)
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<&wasmtime_component_valtuple_t> for Vec<Val> {
    fn from(value: &wasmtime_component_valtuple_t) -> Self {
        value.as_slice().iter().map(Val::from).collect()
    }
}

impl From<&[Val]> for wasmtime_component_valtuple_t {
    fn from(value: &[Val]) -> Self {
        value
            .iter()
            .map(wasmtime_component_val_t::from)
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<&wasmtime_component_valflags_t> for Vec<String> {
    fn from(value: &wasmtime_component_valflags_t) -> Self {
        value
            .clone()
            .take()
            .into_iter()
            .map(|mut x| String::from_utf8(x.take()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }
}

impl From<&[String]> for wasmtime_component_valflags_t {
    fn from(value: &[String]) -> Self {
        value
            .iter()
            .map(|x| wasm_name_t::from_name(x.clone()))
            .collect::<Vec<_>>()
            .into()
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct wasmtime_component_valvariant_t {
    discriminant: wasm_name_t,
    val: Option<Box<wasmtime_component_val_t>>,
}

impl From<(&String, &Option<Box<Val>>)> for wasmtime_component_valvariant_t {
    fn from((discriminant, value): (&String, &Option<Box<Val>>)) -> Self {
        Self {
            discriminant: wasm_name_t::from_name(discriminant.clone()),
            val: value
                .as_ref()
                .map(|x| Box::new(wasmtime_component_val_t::from(x.as_ref()))),
        }
    }
}

impl From<&wasmtime_component_valvariant_t> for (String, Option<Box<Val>>) {
    fn from(value: &wasmtime_component_valvariant_t) -> Self {
        (
            String::from_utf8(value.discriminant.clone().take()).unwrap(),
            value.val.as_ref().map(|x| Box::new(Val::from(x.as_ref()))),
        )
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct wasmtime_component_valresult_t {
    is_ok: bool,
    val: Option<Box<wasmtime_component_val_t>>,
}

impl From<&wasmtime_component_valresult_t> for Result<Option<Box<Val>>, Option<Box<Val>>> {
    fn from(value: &wasmtime_component_valresult_t) -> Self {
        let val = value.val.as_ref().map(|x| Box::new(Val::from(x.as_ref())));

        match value.is_ok {
            true => Ok(val),
            false => Err(val),
        }
    }
}

impl From<&Result<Option<Box<Val>>, Option<Box<Val>>>> for wasmtime_component_valresult_t {
    fn from(value: &Result<Option<Box<Val>>, Option<Box<Val>>>) -> Self {
        let (Ok(x) | Err(x)) = value;

        Self {
            is_ok: value.is_ok(),
            val: x
                .as_ref()
                .map(|x| Box::new(wasmtime_component_val_t::from(x.as_ref()))),
        }
    }
}

#[repr(C, u8)]
#[derive(Clone)]
pub enum wasmtime_component_val_t {
    Bool(bool),
    S8(i8),
    U8(u8),
    S16(i16),
    U16(u16),
    S32(i32),
    U32(u32),
    S64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    Char(u32),
    String(wasm_name_t),
    List(wasmtime_component_vallist_t),
    Record(wasmtime_component_valrecord_t),
    Tuple(wasmtime_component_valtuple_t),
    Variant(wasmtime_component_valvariant_t),
    Enum(wasm_name_t),
    Option(Option<Box<Self>>),
    Result(wasmtime_component_valresult_t),
    Flags(wasmtime_component_valflags_t),
}

impl Default for wasmtime_component_val_t {
    fn default() -> Self {
        Self::Bool(false)
    }
}

impl From<&wasmtime_component_val_t> for Val {
    fn from(value: &wasmtime_component_val_t) -> Self {
        match value {
            wasmtime_component_val_t::Bool(x) => Val::Bool(*x),
            wasmtime_component_val_t::S8(x) => Val::S8(*x),
            wasmtime_component_val_t::U8(x) => Val::U8(*x),
            wasmtime_component_val_t::S16(x) => Val::S16(*x),
            wasmtime_component_val_t::U16(x) => Val::U16(*x),
            wasmtime_component_val_t::S32(x) => Val::S32(*x),
            wasmtime_component_val_t::U32(x) => Val::U32(*x),
            wasmtime_component_val_t::S64(x) => Val::S64(*x),
            wasmtime_component_val_t::U64(x) => Val::U64(*x),
            wasmtime_component_val_t::F32(x) => Val::Float32(*x),
            wasmtime_component_val_t::F64(x) => Val::Float64(*x),
            wasmtime_component_val_t::Char(x) => Val::Char(char::from_u32(*x).unwrap()),
            wasmtime_component_val_t::String(x) => {
                Val::String(String::from_utf8(x.clone().take()).unwrap())
            }
            wasmtime_component_val_t::List(x) => Val::List(x.into()),
            wasmtime_component_val_t::Record(x) => Val::Record(x.into()),
            wasmtime_component_val_t::Tuple(x) => Val::Tuple(x.into()),
            wasmtime_component_val_t::Variant(x) => {
                let (a, b) = x.into();
                Val::Variant(a, b)
            }
            wasmtime_component_val_t::Enum(x) => {
                Val::Enum(String::from_utf8(x.clone().take()).unwrap())
            }
            wasmtime_component_val_t::Option(x) => {
                Val::Option(x.as_ref().map(|x| Box::new(Val::from(x.as_ref()))))
            }
            wasmtime_component_val_t::Result(x) => Val::Result(x.into()),
            wasmtime_component_val_t::Flags(x) => Val::Flags(x.into()),
        }
    }
}

impl From<&Val> for wasmtime_component_val_t {
    fn from(value: &Val) -> Self {
        match value {
            Val::Bool(x) => wasmtime_component_val_t::Bool(*x),
            Val::S8(x) => wasmtime_component_val_t::S8(*x),
            Val::U8(x) => wasmtime_component_val_t::U8(*x),
            Val::S16(x) => wasmtime_component_val_t::S16(*x),
            Val::U16(x) => wasmtime_component_val_t::U16(*x),
            Val::S32(x) => wasmtime_component_val_t::S32(*x),
            Val::U32(x) => wasmtime_component_val_t::U32(*x),
            Val::S64(x) => wasmtime_component_val_t::S64(*x),
            Val::U64(x) => wasmtime_component_val_t::U64(*x),
            Val::Float32(x) => wasmtime_component_val_t::F32(*x),
            Val::Float64(x) => wasmtime_component_val_t::F64(*x),
            Val::Char(x) => wasmtime_component_val_t::Char(*x as _),
            Val::String(x) => wasmtime_component_val_t::String(wasm_name_t::from_name(x.clone())),
            Val::List(x) => wasmtime_component_val_t::List(x.as_slice().into()),
            Val::Record(x) => wasmtime_component_val_t::Record(x.as_slice().into()),
            Val::Tuple(x) => wasmtime_component_val_t::Tuple(x.as_slice().into()),
            Val::Variant(discriminant, val) => {
                wasmtime_component_val_t::Variant((discriminant, val).into())
            }
            Val::Enum(x) => wasmtime_component_val_t::Enum(wasm_name_t::from_name(x.clone())),
            Val::Option(x) => wasmtime_component_val_t::Option(
                x.as_ref()
                    .map(|x| Box::new(wasmtime_component_val_t::from(x.as_ref()))),
            ),
            Val::Result(x) => wasmtime_component_val_t::Result(x.into()),
            Val::Flags(x) => wasmtime_component_val_t::Flags(x.as_slice().into()),
            Val::Resource(_resource_any) => todo!(),
            Val::Future(_) => todo!(),
            Val::Stream(_) => todo!(),
            Val::ErrorContext(_) => todo!(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_val_new() -> Box<wasmtime_component_val_t> {
    Box::new(wasmtime_component_val_t::default())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_component_val_delete(value: *mut wasmtime_component_val_t) {
    unsafe {
        std::ptr::drop_in_place(value);
    }
}
