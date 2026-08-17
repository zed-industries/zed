use crate::{CExternType, wasm_externtype_t, wasm_valtype_t, wasm_valtype_vec_t};
use std::cell::OnceCell;
use std::{
    mem,
    sync::{Arc, Mutex},
};
use wasmtime::{Engine, FuncType, ValType};

#[repr(transparent)]
#[derive(Clone)]
pub struct wasm_functype_t {
    ext: wasm_externtype_t,
}

wasmtime_c_api_macros::declare_ty!(wasm_functype_t);

#[derive(Clone)]
enum LazyFuncType {
    Lazy {
        params: Vec<ValType>,
        results: Vec<ValType>,
    },
    FuncType(FuncType),
}

impl LazyFuncType {
    pub(crate) fn force(&mut self, engine: &Engine) -> FuncType {
        match self {
            LazyFuncType::FuncType(ty) => ty.clone(),
            LazyFuncType::Lazy { params, results } => {
                let params = mem::take(params);
                let results = mem::take(results);
                let ty = FuncType::new(engine, params, results);
                *self = LazyFuncType::FuncType(ty.clone());
                ty
            }
        }
    }

    fn params(&self) -> impl ExactSizeIterator<Item = ValType> + '_ {
        match self {
            LazyFuncType::Lazy { params, .. } => LazyFuncTypeIter::Lazy(params.iter()),
            LazyFuncType::FuncType(f) => LazyFuncTypeIter::FuncType(f.params()),
        }
    }

    fn results(&self) -> impl ExactSizeIterator<Item = ValType> + '_ {
        match self {
            LazyFuncType::Lazy { results, .. } => LazyFuncTypeIter::Lazy(results.iter()),
            LazyFuncType::FuncType(f) => LazyFuncTypeIter::FuncType(f.results()),
        }
    }
}

enum LazyFuncTypeIter<'a, T> {
    Lazy(std::slice::Iter<'a, ValType>),
    FuncType(T),
}

impl<'a, T> Iterator for LazyFuncTypeIter<'a, T>
where
    T: Iterator<Item = ValType>,
{
    type Item = ValType;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            LazyFuncTypeIter::FuncType(i) => i.next(),
            LazyFuncTypeIter::Lazy(i) => i.next().cloned(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            LazyFuncTypeIter::FuncType(i) => i.size_hint(),
            LazyFuncTypeIter::Lazy(i) => i.size_hint(),
        }
    }
}

impl<'a, T> ExactSizeIterator for LazyFuncTypeIter<'a, T> where T: ExactSizeIterator<Item = ValType> {}

#[derive(Clone)]
pub(crate) struct CFuncType {
    ty: Arc<Mutex<LazyFuncType>>,
    params_cache: OnceCell<wasm_valtype_vec_t>,
    returns_cache: OnceCell<wasm_valtype_vec_t>,
}

impl wasm_functype_t {
    pub(crate) fn new(ty: FuncType) -> wasm_functype_t {
        wasm_functype_t {
            ext: wasm_externtype_t::from_extern_type(ty.into()),
        }
    }

    pub(crate) fn lazy(params: Vec<ValType>, results: Vec<ValType>) -> wasm_functype_t {
        wasm_functype_t {
            ext: wasm_externtype_t::from_cextern_type(CExternType::Func(CFuncType::lazy(
                params, results,
            ))),
        }
    }

    pub(crate) fn try_from(e: &wasm_externtype_t) -> Option<&wasm_functype_t> {
        match &e.which {
            CExternType::Func(_) => Some(unsafe { &*(e as *const _ as *const _) }),
            _ => None,
        }
    }

    pub(crate) fn ty(&self) -> &CFuncType {
        match &self.ext.which {
            CExternType::Func(f) => &f,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

impl CFuncType {
    pub(crate) fn new(ty: FuncType) -> CFuncType {
        CFuncType {
            ty: Arc::new(Mutex::new(LazyFuncType::FuncType(ty))),
            params_cache: OnceCell::new(),
            returns_cache: OnceCell::new(),
        }
    }

    pub(crate) fn lazy(params: Vec<ValType>, results: Vec<ValType>) -> CFuncType {
        CFuncType {
            ty: Arc::new(Mutex::new(LazyFuncType::Lazy { params, results })),
            params_cache: OnceCell::new(),
            returns_cache: OnceCell::new(),
        }
    }

    pub(crate) fn ty(&self, engine: &Engine) -> FuncType {
        let mut ty = self.ty.lock().unwrap();
        ty.force(engine)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_functype_new(
    params: &mut wasm_valtype_vec_t,
    results: &mut wasm_valtype_vec_t,
) -> Box<wasm_functype_t> {
    let params = params
        .take()
        .into_iter()
        .map(|vt| vt.unwrap().ty.clone())
        .collect();
    let results = results
        .take()
        .into_iter()
        .map(|vt| vt.unwrap().ty.clone())
        .collect();
    Box::new(wasm_functype_t::lazy(params, results))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_functype_params(ft: &wasm_functype_t) -> &wasm_valtype_vec_t {
    let ft = ft.ty();
    ft.params_cache.get_or_init(|| {
        let ty = ft.ty.lock().unwrap();
        ty.params()
            .map(|p| Some(Box::new(wasm_valtype_t { ty: p.clone() })))
            .collect::<Vec<_>>()
            .into()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_functype_results(ft: &wasm_functype_t) -> &wasm_valtype_vec_t {
    let ft = ft.ty();
    ft.returns_cache.get_or_init(|| {
        let ty = ft.ty.lock().unwrap();
        ty.results()
            .map(|p| Some(Box::new(wasm_valtype_t { ty: p.clone() })))
            .collect::<Vec<_>>()
            .into()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_functype_as_externtype(ty: &wasm_functype_t) -> &wasm_externtype_t {
    &ty.ext
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_functype_as_externtype_const(ty: &wasm_functype_t) -> &wasm_externtype_t {
    &ty.ext
}
