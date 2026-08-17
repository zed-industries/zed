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

//! An OpenGL bindings generator. It defines a function named `generate_bindings` which can be
//! used to generate all constants and functions of a given OpenGL version.
//!
//! # Example
//!
//! In `build.rs`:
//!
//! ```no_run
//! extern crate gl_generator;
//!
//! use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
//! use std::env;
//! use std::fs::File;
//! use std::path::Path;
//!
//! fn main() {
//!     let dest = env::var("OUT_DIR").unwrap();
//!     let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();
//!
//!     Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [])
//!         .write_bindings(GlobalGenerator, &mut file)
//!         .unwrap();
//! }
//! ```
//!
//! In your project:
//!
//! ```ignore
//! include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
//! ```
//!
//! # About EGL
//!
//! When you generate bindings for EGL, the following platform-specific types must be declared
//!  *at the same level where you include the bindings*:
//!
//! - `khronos_utime_nanoseconds_t`
//! - `khronos_uint64_t`
//! - `khronos_ssize_t`
//! - `EGLNativeDisplayType`
//! - `EGLNativePixmapType`
//! - `EGLNativeWindowType`
//! - `EGLint`
//! - `NativeDisplayType`
//! - `NativePixmapType`
//! - `NativeWindowType`
//!

#[macro_use]
extern crate log;
extern crate xml;

#[cfg(feature = "unstable_generator_utils")]
pub mod generators;
#[cfg(not(feature = "unstable_generator_utils"))]
mod generators;

mod registry;

pub use generators::debug_struct_gen::DebugStructGenerator;
pub use generators::global_gen::GlobalGenerator;
pub use generators::static_gen::StaticGenerator;
pub use generators::static_struct_gen::StaticStructGenerator;
pub use generators::struct_gen::StructGenerator;
pub use generators::Generator;

pub use registry::*;
