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
pub struct StaticGenerator;

impl super::Generator for StaticGenerator {
    fn write<W>(&self, registry: &Registry, dest: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        try!(write_header(dest));
        try!(write_type_aliases(registry, dest));
        try!(write_enums(registry, dest));
        try!(write_fns(registry, dest));
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

/// io::Writes all functions corresponding to the GL bindings.
///
/// These are foreign functions, they don't have any content.
fn write_fns<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    try!(writeln!(
        dest,
        "
        #[allow(non_snake_case, unused_variables, dead_code)]
        extern \"system\" {{"
    ));

    for cmd in &registry.cmds {
        try!(writeln!(
            dest,
            "#[link_name=\"{symbol}\"]
            pub fn {name}({params}) -> {return_suffix};",
            symbol = super::gen_symbol_name(registry.api, &cmd.proto.ident),
            name = cmd.proto.ident,
            params = super::gen_parameters(cmd, true, true).join(", "),
            return_suffix = cmd.proto.ty,
        ));
    }

    writeln!(dest, "}}")
}
