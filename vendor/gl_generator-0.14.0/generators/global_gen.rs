// Copyright 2015 Brendan Zabarauskas and the gl-rs developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use registry::Registry;
use std::io;

#[allow(missing_copy_implementations)]
pub struct GlobalGenerator;

impl super::Generator for GlobalGenerator {
    fn write<W>(&self, registry: &Registry, dest: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        try!(write_header(dest));
        try!(write_metaloadfn(dest));
        try!(write_type_aliases(registry, dest));
        try!(write_enums(registry, dest));
        try!(write_fns(registry, dest));
        try!(write_fnptr_struct_def(dest));
        try!(write_ptrs(registry, dest));
        try!(write_fn_mods(registry, dest));
        try!(write_panicking_fns(registry, dest));
        try!(write_load_fn(registry, dest));
        Ok(())
    }
}

/// Creates a `__gl_imports` module which contains all the external symbols that we need for the
///  bindings.
fn write_header<W>(dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    writeln!(
        dest,
        r#"
        mod __gl_imports {{
            pub use std::mem;
            pub use std::os::raw;
        }}
    "#
    )
}

/// Creates the metaloadfn function for fallbacks
fn write_metaloadfn<W>(dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    writeln!(
        dest,
        r#"
        #[inline(never)]
        fn metaloadfn(loadfn: &mut dyn FnMut(&'static str) -> *const __gl_imports::raw::c_void,
                      symbol: &'static str,
                      fallbacks: &[&'static str]) -> *const __gl_imports::raw::c_void {{
            let mut ptr = loadfn(symbol);
            if ptr.is_null() {{
                for &sym in fallbacks {{
                    ptr = loadfn(sym);
                    if !ptr.is_null() {{ break; }}
                }}
            }}
            ptr
        }}
    "#
    )
}

/// Creates a `types` module which contains all the type aliases.
///
/// See also `generators::gen_types`.
fn write_type_aliases<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    try!(writeln!(
        dest,
        r#"
        pub mod types {{
            #![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]
    "#
    ));

    try!(super::gen_types(registry.api, dest));

    writeln!(
        dest,
        "
        }}
    "
    )
}

/// Creates all the `<enum>` elements at the root of the bindings.
fn write_enums<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    for enm in &registry.enums {
        try!(super::gen_enum_item(enm, "types::", dest));
    }

    Ok(())
}

/// Creates the functions corresponding to the GL commands.
///
/// The function calls the corresponding function pointer stored in the `storage` module created
///  by `write_ptrs`.
fn write_fns<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    for cmd in &registry.cmds {
        if let Some(v) = registry.aliases.get(&cmd.proto.ident) {
            try!(writeln!(dest, "/// Fallbacks: {}", v.join(", ")));
        }

        try!(writeln!(dest,
            "#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn {name}({params}) -> {return_suffix} {{ \
                __gl_imports::mem::transmute::<_, extern \"system\" fn({typed_params}) -> {return_suffix}>\
                    (storage::{name}.f)({idents}) \
            }}",
            name = cmd.proto.ident,
            params = super::gen_parameters(cmd, true, true).join(", "),
            typed_params = super::gen_parameters(cmd, false, true).join(", "),
            return_suffix = cmd.proto.ty,
            idents = super::gen_parameters(cmd, true, false).join(", "),
        ));
    }

    Ok(())
}

/// Creates a `FnPtr` structure which contains the store for a single binding.
fn write_fnptr_struct_def<W>(dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    writeln!(dest,
             "
        #[allow(missing_copy_implementations)]
        pub struct FnPtr {{
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::raw::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }}

        impl FnPtr {{
            /// Creates a `FnPtr` from a load attempt.
            pub fn new(ptr: *const __gl_imports::raw::c_void) -> FnPtr {{
                if ptr.is_null() {{
                    FnPtr {{ f: missing_fn_panic as *const __gl_imports::raw::c_void, is_loaded: false }}
                }} else {{
                    FnPtr {{ f: ptr, is_loaded: true }}
                }}
            }}
        }}
    ")
}

/// Creates a `storage` module which contains a static `FnPtr` per GL command in the registry.
fn write_ptrs<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    try!(writeln!(
        dest,
        "mod storage {{
            #![allow(non_snake_case)]
            #![allow(non_upper_case_globals)]
            use super::__gl_imports::raw;
            use super::FnPtr;"
    ));

    for c in &registry.cmds {
        try!(writeln!(
            dest,
            "pub static mut {name}: FnPtr = FnPtr {{
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            }};",
            name = c.proto.ident
        ));
    }

    writeln!(dest, "}}")
}

/// Creates one module for each GL command.
///
/// Each module contains `is_loaded` and `load_with` which interact with the `storage` module
///  created by `write_ptrs`.
fn write_fn_mods<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    for c in &registry.cmds {
        let fallbacks = match registry.aliases.get(&c.proto.ident) {
            Some(v) => {
                let names = v
                    .iter()
                    .map(|name| format!("\"{}\"", super::gen_symbol_name(registry.api, &name[..])))
                    .collect::<Vec<_>>();
                format!("&[{}]", names.join(", "))
            },
            None => "&[]".to_string(),
        };
        let fnname = &c.proto.ident[..];
        let symbol = super::gen_symbol_name(registry.api, &c.proto.ident[..]);
        let symbol = &symbol[..];

        try!(writeln!(dest, r##"
            #[allow(non_snake_case)]
            pub mod {fnname} {{
                use super::{{storage, metaloadfn}};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {{
                    unsafe {{ storage::{fnname}.is_loaded }}
                }}

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {{
                    unsafe {{
                        storage::{fnname} = FnPtr::new(metaloadfn(&mut loadfn, "{symbol}", {fallbacks}))
                    }}
                }}
            }}
        "##, fnname = fnname, fallbacks = fallbacks, symbol = symbol));
    }

    Ok(())
}

/// Creates a `missing_fn_panic` function.
///
/// This function is the mock that is called if the real function could not be called.
fn write_panicking_fns<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    writeln!(
        dest,
        "#[inline(never)]
        fn missing_fn_panic() -> ! {{
            panic!(\"{api} function was not loaded\")
        }}
        ",
        api = registry.api
    )
}

/// Creates the `load_with` function.
///
/// The function calls `load_with` in each module created by `write_fn_mods`.
fn write_load_fn<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    try!(writeln!(dest,
                  "
        /// Load each OpenGL symbol using a custom load function. This allows for the
        /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
        /// ~~~ignore
        /// gl::load_with(|s| glfw.get_proc_address(s));
        /// ~~~
        #[allow(dead_code)]
        pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const __gl_imports::raw::c_void {{
            #[inline(never)]
            fn inner(loadfn: &mut dyn FnMut(&'static str) -> *const __gl_imports::raw::c_void) {{
    "));

    for c in &registry.cmds {
        try!(writeln!(
            dest,
            "{cmd_name}::load_with(&mut *loadfn);",
            cmd_name = &c.proto.ident[..]
        ));
    }

    writeln!(
        dest,
        "
            }}

            inner(&mut loadfn)
        }}
    "
    )
}
