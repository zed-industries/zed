use super::*;

#[doc(hidden)]
pub enum CallingConvention {
    Stdcall(usize),
    Cdecl,
}

// Returns the libraries and their function and stack sizes used by the gnu and msvc tools to build the umbrella libs.
#[doc(hidden)]
pub fn libraries() -> BTreeMap<String, BTreeMap<String, CallingConvention>> {
    let mut libraries = BTreeMap::new();

    let reader = Reader::new(expand_input(&["default"]));
    combine_libraries(&reader, &mut libraries);
    libraries
}

fn combine_libraries(
    reader: &Reader,
    libraries: &mut BTreeMap<String, BTreeMap<String, CallingConvention>>,
) {
    for types in reader.values() {
        for ty in types.values() {
            let Some(ty) = cpp_fn(ty) else {
                continue;
            };

            // Windows libs are always produced with lower case module names.
            let library = ty.method.module_name().to_lowercase();
            let impl_map = ty.method.impl_map().unwrap();
            let flags = impl_map.flags();
            let name = impl_map.import_name().to_string();

            if flags.contains(PInvokeAttributes::CallConvPlatformapi) {
                let arches = ty.method.arches();
                let params = if (arches == 0) || (arches & 1 == 1) {
                    ty.method.signature(ty.namespace, &[]).size()
                } else {
                    0
                };

                libraries
                    .entry(library)
                    .or_default()
                    .insert(name, CallingConvention::Stdcall(params));
            } else if flags.contains(PInvokeAttributes::CallConvCdecl) {
                libraries
                    .entry(library)
                    .or_default()
                    .insert(name, CallingConvention::Cdecl);
            } else {
                panic!();
            }
        }
    }
}

fn cpp_fn(types: &[Type]) -> Option<CppFn> {
    let mut functions = vec![];

    for ty in types {
        if let Type::CppFn(ty) = ty {
            functions.push(ty.clone());
        }
    }

    for ty in &functions {
        let arches = ty.method.arches();
        if (arches == 0) || (arches & 1 == 1) {
            return Some(ty.clone());
        }
    }

    functions.first().cloned()
}
