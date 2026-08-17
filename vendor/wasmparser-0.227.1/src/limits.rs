/* Copyright 2017 Mozilla Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![cfg_attr(not(feature = "validate"), allow(dead_code))]

// The following limits are imposed by wasmparser on WebAssembly modules.
// The limits are agreed upon with other engines for consistency.
pub const MAX_WASM_TYPES: usize = 1_000_000;
pub const MAX_WASM_SUPERTYPES: usize = 1;
pub const MAX_WASM_FUNCTIONS: usize = 1_000_000;
pub const MAX_WASM_IMPORTS: usize = 1_000_000;
pub const MAX_WASM_EXPORTS: usize = 1_000_000;
pub const MAX_WASM_GLOBALS: usize = 1_000_000;
pub const MAX_WASM_ELEMENT_SEGMENTS: usize = 100_000;
pub const MAX_WASM_DATA_SEGMENTS: usize = 100_000;
pub const MAX_WASM_STRING_SIZE: usize = 100_000;
pub const MAX_WASM_FUNCTION_SIZE: usize = 128 * 1024;
pub const MAX_WASM_FUNCTION_LOCALS: usize = 50000;
pub const MAX_WASM_FUNCTION_PARAMS: usize = 1000;
pub const MAX_WASM_FUNCTION_RETURNS: usize = 1000;
pub const _MAX_WASM_TABLE_SIZE: usize = 10_000_000;
pub const MAX_WASM_TABLE_ENTRIES: usize = 10_000_000;
pub const MAX_WASM_TABLES: usize = 100;
pub const MAX_WASM_MEMORIES: usize = 100;
pub const MAX_WASM_TAGS: usize = 1_000_000;
pub const MAX_WASM_BR_TABLE_SIZE: usize = MAX_WASM_FUNCTION_SIZE;
pub const MAX_WASM_STRUCT_FIELDS: usize = 10_000;
pub const MAX_WASM_CATCHES: usize = 10_000;
pub const MAX_WASM_SUBTYPING_DEPTH: usize = 63;
pub const MAX_WASM_HANDLERS: usize = 10_000;
pub const MAX_WASM_TYPE_SIZE: u32 = 1_000_000;

pub const DEFAULT_WASM_PAGE_SIZE: u64 = 1 << 16;

pub fn max_wasm_memory32_pages(page_size: u64) -> u64 {
    assert!(page_size.is_power_of_two());
    assert!(page_size <= DEFAULT_WASM_PAGE_SIZE);
    (1 << 32) / page_size
}

pub fn max_wasm_memory64_pages(page_size: u64) -> u64 {
    assert!(page_size.is_power_of_two());
    assert!(page_size <= DEFAULT_WASM_PAGE_SIZE);
    u64::try_from((1_u128 << 64) / u128::from(page_size)).unwrap_or(u64::MAX)
}

// Component-related limits

#[cfg(feature = "component-model")]
pub use self::component_limits::*;
#[cfg(feature = "component-model")]
mod component_limits {
    pub const MAX_WASM_MODULE_SIZE: usize = 1024 * 1024 * 1024; //= 1 GiB
    pub const MAX_WASM_MODULE_TYPE_DECLS: usize = 100_000;
    pub const MAX_WASM_COMPONENT_TYPE_DECLS: usize = 100_000;
    pub const MAX_WASM_INSTANCE_TYPE_DECLS: usize = 100_000;
    pub const MAX_WASM_RECORD_FIELDS: usize = 10_000;
    pub const MAX_WASM_VARIANT_CASES: usize = 10_000;
    pub const MAX_WASM_TUPLE_TYPES: usize = 10_000;
    pub const MAX_WASM_FLAG_NAMES: usize = 1_000;
    pub const MAX_WASM_ENUM_CASES: usize = 10_000;
    pub const MAX_WASM_INSTANTIATION_EXPORTS: usize = 100_000;
    pub const MAX_WASM_CANONICAL_OPTIONS: usize = 10;
    pub const MAX_WASM_INSTANTIATION_ARGS: usize = 100_000;
    pub const MAX_WASM_START_ARGS: usize = 1000;
    pub const MAX_WASM_MODULES: usize = 1_000;
    pub const MAX_WASM_COMPONENTS: usize = 1_000;
    pub const MAX_WASM_INSTANCES: usize = 1_000;
    pub const MAX_WASM_VALUES: usize = 1_000;

    /// Core items in components such as globals/memories/tables don't actually
    /// create new definitions but are instead just aliases to preexisting items.
    /// This means they have a different limit than the core wasm based limits.
    pub const MAX_CORE_INDEX_SPACE_ITEMS: usize = 1_000_000;
}
