/*!
Frontend for [WGSL][wgsl] (WebGPU Shading Language).

[wgsl]: https://gpuweb.github.io/gpuweb/wgsl.html
*/

mod error;
mod index;
mod lower;
mod parse;
#[cfg(test)]
mod tests;

pub use parse::directive::enable_extension::{
    EnableExtension, ImplementedEnableExtension, UnimplementedEnableExtension,
};

pub use crate::front::wgsl::error::ParseError;
pub use crate::front::wgsl::parse::directive::language_extension::{
    ImplementedLanguageExtension, LanguageExtension, UnimplementedLanguageExtension,
};
pub use crate::front::wgsl::parse::Options;

use alloc::boxed::Box;
use thiserror::Error;

use crate::front::wgsl::error::Error;
use crate::front::wgsl::lower::Lowerer;
use crate::front::wgsl::parse::Parser;
use crate::Scalar;

#[cfg(test)]
use std::println;

pub(crate) type Result<'a, T> = core::result::Result<T, Box<Error<'a>>>;

pub struct Frontend {
    parser: Parser,
    options: Options,
}

impl Frontend {
    pub const fn new() -> Self {
        Self {
            parser: Parser::new(),
            options: Options::new(),
        }
    }

    pub const fn new_with_options(options: Options) -> Self {
        Self {
            parser: Parser::new(),
            options,
        }
    }

    pub const fn set_options(&mut self, options: Options) {
        self.options = options;
    }

    pub fn parse(&mut self, source: &str) -> core::result::Result<crate::Module, ParseError> {
        self.inner(source).map_err(|x| x.as_parse_error(source))
    }

    fn inner<'a>(&mut self, source: &'a str) -> Result<'a, crate::Module> {
        let tu = self.parser.parse(source, &self.options)?;
        let index = index::Index::generate(&tu)?;
        let module = Lowerer::new(&index).lower(tu)?;

        Ok(module)
    }
}

/// <div class="warning">
// NOTE: Keep this in sync with `wgpu::Device::create_shader_module`!
// NOTE: Keep this in sync with `wgpu_core::Global::device_create_shader_module`!
///
/// This function may consume a lot of stack space. Compiler-enforced limits for parsing recursion
/// exist; if shader compilation runs into them, it will return an error gracefully. However, on
/// some build profiles and platforms, the default stack size for a thread may be exceeded before
/// this limit is reached during parsing. Callers should ensure that there is enough stack space
/// for this, particularly if calls to this method are exposed to user input.
///
/// </div>
pub fn parse_str(source: &str) -> core::result::Result<crate::Module, ParseError> {
    Frontend::new().parse(source)
}

#[cfg(test)]
#[track_caller]
pub fn assert_parse_err(input: &str, snapshot: &str) {
    let output = parse_str(input)
        .expect_err("expected parser error")
        .emit_to_string(input);
    if output != snapshot {
        for diff in diff::lines(snapshot, &output) {
            match diff {
                diff::Result::Left(l) => println!("-{l}"),
                diff::Result::Both(l, _) => println!(" {l}"),
                diff::Result::Right(r) => println!("+{r}"),
            }
        }
        panic!("Error snapshot failed");
    }
}
