// Don't worry about unused imports if we're frobbing features, only worry about
// them with the default set of features enabled.
#![cfg_attr(not(feature = "cache"), allow(unused_imports))]

use crate::{handle_result, wasm_memorytype_t, wasmtime_error_t};
use std::os::raw::c_char;
use std::ptr;
use std::{ffi::CStr, sync::Arc};
use wasmtime::{
    Config, InstanceAllocationStrategy, LinearMemory, MemoryCreator, OptLevel, ProfilingStrategy,
    Result, Strategy,
};

#[cfg(feature = "pooling-allocator")]
use wasmtime::PoolingAllocationConfig;

#[repr(C)]
#[derive(Clone)]
pub struct wasm_config_t {
    pub(crate) config: Config,
}

wasmtime_c_api_macros::declare_own!(wasm_config_t);

#[repr(u8)]
#[derive(Clone)]
pub enum wasmtime_strategy_t {
    WASMTIME_STRATEGY_AUTO,
    WASMTIME_STRATEGY_CRANELIFT,
}

#[repr(u8)]
#[derive(Clone)]
pub enum wasmtime_opt_level_t {
    WASMTIME_OPT_LEVEL_NONE,
    WASMTIME_OPT_LEVEL_SPEED,
    WASMTIME_OPT_LEVEL_SPEED_AND_SIZE,
}

#[repr(u8)]
#[derive(Clone)]
pub enum wasmtime_profiling_strategy_t {
    WASMTIME_PROFILING_STRATEGY_NONE,
    WASMTIME_PROFILING_STRATEGY_JITDUMP,
    WASMTIME_PROFILING_STRATEGY_VTUNE,
    WASMTIME_PROFILING_STRATEGY_PERFMAP,
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_config_new() -> Box<wasm_config_t> {
    Box::new(wasm_config_t {
        config: Config::default(),
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_debug_info_set(c: &mut wasm_config_t, enable: bool) {
    c.config.debug_info(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_consume_fuel_set(c: &mut wasm_config_t, enable: bool) {
    c.config.consume_fuel(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_epoch_interruption_set(c: &mut wasm_config_t, enable: bool) {
    c.config.epoch_interruption(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_max_wasm_stack_set(c: &mut wasm_config_t, size: usize) {
    c.config.max_wasm_stack(size);
}

#[unsafe(no_mangle)]
#[cfg(feature = "threads")]
pub extern "C" fn wasmtime_config_wasm_threads_set(c: &mut wasm_config_t, enable: bool) {
    c.config.wasm_threads(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_tail_call_set(c: &mut wasm_config_t, enable: bool) {
    c.config.wasm_tail_call(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_reference_types_set(c: &mut wasm_config_t, enable: bool) {
    c.config.wasm_reference_types(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_function_references_set(
    c: &mut wasm_config_t,
    enable: bool,
) {
    c.config.wasm_function_references(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_gc_set(c: &mut wasm_config_t, enable: bool) {
    c.config.wasm_gc(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_simd_set(c: &mut wasm_config_t, enable: bool) {
    c.config.wasm_simd(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_relaxed_simd_set(c: &mut wasm_config_t, enable: bool) {
    c.config.wasm_relaxed_simd(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_relaxed_simd_deterministic_set(
    c: &mut wasm_config_t,
    enable: bool,
) {
    c.config.relaxed_simd_deterministic(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_bulk_memory_set(c: &mut wasm_config_t, enable: bool) {
    c.config.wasm_bulk_memory(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_multi_value_set(c: &mut wasm_config_t, enable: bool) {
    c.config.wasm_multi_value(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_multi_memory_set(c: &mut wasm_config_t, enable: bool) {
    c.config.wasm_multi_memory(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_memory64_set(c: &mut wasm_config_t, enable: bool) {
    c.config.wasm_memory64(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_stack_switching_set(c: &mut wasm_config_t, enable: bool) {
    c.config.wasm_stack_switching(enable);
}

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub extern "C" fn wasmtime_config_strategy_set(
    c: &mut wasm_config_t,
    strategy: wasmtime_strategy_t,
) {
    use wasmtime_strategy_t::*;
    c.config.strategy(match strategy {
        WASMTIME_STRATEGY_AUTO => Strategy::Auto,
        WASMTIME_STRATEGY_CRANELIFT => Strategy::Cranelift,
    });
}

#[unsafe(no_mangle)]
#[cfg(feature = "parallel-compilation")]
pub extern "C" fn wasmtime_config_parallel_compilation_set(c: &mut wasm_config_t, enable: bool) {
    c.config.parallel_compilation(enable);
}

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub extern "C" fn wasmtime_config_cranelift_debug_verifier_set(
    c: &mut wasm_config_t,
    enable: bool,
) {
    c.config.cranelift_debug_verifier(enable);
}

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub extern "C" fn wasmtime_config_cranelift_nan_canonicalization_set(
    c: &mut wasm_config_t,
    enable: bool,
) {
    c.config.cranelift_nan_canonicalization(enable);
}

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub extern "C" fn wasmtime_config_cranelift_opt_level_set(
    c: &mut wasm_config_t,
    opt_level: wasmtime_opt_level_t,
) {
    use wasmtime_opt_level_t::*;
    c.config.cranelift_opt_level(match opt_level {
        WASMTIME_OPT_LEVEL_NONE => OptLevel::None,
        WASMTIME_OPT_LEVEL_SPEED => OptLevel::Speed,
        WASMTIME_OPT_LEVEL_SPEED_AND_SIZE => OptLevel::SpeedAndSize,
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_profiler_set(
    c: &mut wasm_config_t,
    strategy: wasmtime_profiling_strategy_t,
) {
    use wasmtime_profiling_strategy_t::*;
    c.config.profiler(match strategy {
        WASMTIME_PROFILING_STRATEGY_NONE => ProfilingStrategy::None,
        WASMTIME_PROFILING_STRATEGY_JITDUMP => ProfilingStrategy::JitDump,
        WASMTIME_PROFILING_STRATEGY_VTUNE => ProfilingStrategy::VTune,
        WASMTIME_PROFILING_STRATEGY_PERFMAP => ProfilingStrategy::PerfMap,
    });
}

#[unsafe(no_mangle)]
#[cfg(feature = "cache")]
pub unsafe extern "C" fn wasmtime_config_cache_config_load(
    c: &mut wasm_config_t,
    filename: *const c_char,
) -> Option<Box<wasmtime_error_t>> {
    use std::path::Path;

    use wasmtime::Cache;

    handle_result(
        if filename.is_null() {
            Cache::from_file(None).map(|cache| c.config.cache(Some(cache)))
        } else {
            match CStr::from_ptr(filename).to_str() {
                Ok(s) => {
                    Cache::from_file(Some(&Path::new(s))).map(|cache| c.config.cache(Some(cache)))
                }
                Err(e) => Err(e.into()),
            }
        },
        |_cfg| {},
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_memory_may_move_set(c: &mut wasm_config_t, enable: bool) {
    c.config.memory_may_move(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_memory_reservation_set(c: &mut wasm_config_t, size: u64) {
    c.config.memory_reservation(size);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_memory_guard_size_set(c: &mut wasm_config_t, size: u64) {
    c.config.memory_guard_size(size);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_memory_reservation_for_growth_set(
    c: &mut wasm_config_t,
    size: u64,
) {
    c.config.memory_reservation_for_growth(size);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_native_unwind_info_set(c: &mut wasm_config_t, enabled: bool) {
    c.config.native_unwind_info(enabled);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_config_target_set(
    c: &mut wasm_config_t,
    target: *const c_char,
) -> Option<Box<wasmtime_error_t>> {
    let target = CStr::from_ptr(target).to_str().expect("not valid utf-8");
    handle_result(c.config.target(target), |_cfg| {})
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_macos_use_mach_ports_set(c: &mut wasm_config_t, enabled: bool) {
    c.config.macos_use_mach_ports(enabled);
}

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub unsafe extern "C" fn wasmtime_config_cranelift_flag_enable(
    c: &mut wasm_config_t,
    flag: *const c_char,
) {
    let flag = CStr::from_ptr(flag).to_str().expect("not valid utf-8");
    c.config.cranelift_flag_enable(flag);
}

#[unsafe(no_mangle)]
#[cfg(any(feature = "cranelift", feature = "winch"))]
pub unsafe extern "C" fn wasmtime_config_cranelift_flag_set(
    c: &mut wasm_config_t,
    flag: *const c_char,
    value: *const c_char,
) {
    let flag = CStr::from_ptr(flag).to_str().expect("not valid utf-8");
    let value = CStr::from_ptr(value).to_str().expect("not valid utf-8");
    c.config.cranelift_flag_set(flag, value);
}

pub type wasmtime_memory_get_callback_t = extern "C" fn(
    env: *mut std::ffi::c_void,
    byte_size: &mut usize,
    maximum_byte_size: &mut usize,
) -> *mut u8;

pub type wasmtime_memory_grow_callback_t =
    extern "C" fn(env: *mut std::ffi::c_void, new_size: usize) -> Option<Box<wasmtime_error_t>>;

#[repr(C)]
pub struct wasmtime_linear_memory_t {
    env: *mut std::ffi::c_void,
    get_memory: wasmtime_memory_get_callback_t,
    grow_memory: wasmtime_memory_grow_callback_t,
    finalizer: Option<extern "C" fn(arg1: *mut std::ffi::c_void)>,
}

pub type wasmtime_new_memory_callback_t = extern "C" fn(
    env: *mut std::ffi::c_void,
    ty: &wasm_memorytype_t,
    minimum: usize,
    maximum: usize,
    reserved_size_in_bytes: usize,
    guard_size_in_bytes: usize,
    memory_ret: *mut wasmtime_linear_memory_t,
) -> Option<Box<wasmtime_error_t>>;

struct CHostLinearMemory {
    foreign: crate::ForeignData,
    get_memory: wasmtime_memory_get_callback_t,
    grow_memory: wasmtime_memory_grow_callback_t,
}

unsafe impl LinearMemory for CHostLinearMemory {
    fn byte_size(&self) -> usize {
        let mut byte_size = 0;
        let mut byte_capacity = 0;
        let cb = self.get_memory;
        cb(self.foreign.data, &mut byte_size, &mut byte_capacity);
        return byte_size;
    }
    fn byte_capacity(&self) -> usize {
        let mut byte_size = 0;
        let mut byte_capacity = 0;
        let cb = self.get_memory;
        cb(self.foreign.data, &mut byte_size, &mut byte_capacity);
        byte_capacity
    }
    fn as_ptr(&self) -> *mut u8 {
        let mut byte_size = 0;
        let mut byte_capacity = 0;
        let cb = self.get_memory;
        cb(self.foreign.data, &mut byte_size, &mut byte_capacity)
    }
    fn grow_to(&mut self, new_size: usize) -> Result<()> {
        let cb = self.grow_memory;
        let error = cb(self.foreign.data, new_size);
        if let Some(err) = error {
            Err((*err).into())
        } else {
            Ok(())
        }
    }
}

#[repr(C)]
pub struct wasmtime_memory_creator_t {
    env: *mut std::ffi::c_void,
    new_memory: wasmtime_new_memory_callback_t,
    finalizer: Option<extern "C" fn(arg1: *mut std::ffi::c_void)>,
}

struct CHostMemoryCreator {
    foreign: crate::ForeignData,
    new_memory: wasmtime_new_memory_callback_t,
}
unsafe impl Send for CHostMemoryCreator {}
unsafe impl Sync for CHostMemoryCreator {}

unsafe impl MemoryCreator for CHostMemoryCreator {
    fn new_memory(
        &self,
        ty: wasmtime::MemoryType,
        minimum: usize,
        maximum: Option<usize>,
        reserved_size_in_bytes: Option<usize>,
        guard_size_in_bytes: usize,
    ) -> Result<Box<dyn wasmtime::LinearMemory>, String> {
        extern "C" fn panic_get_callback(
            _env: *mut std::ffi::c_void,
            _byte_size: &mut usize,
            _maximum_byte_size: &mut usize,
        ) -> *mut u8 {
            panic!("a callback must be set");
        }
        extern "C" fn panic_grow_callback(
            _env: *mut std::ffi::c_void,
            _size: usize,
        ) -> Option<Box<wasmtime_error_t>> {
            panic!("a callback must be set");
        }
        let mut memory = wasmtime_linear_memory_t {
            env: ptr::null_mut(),
            get_memory: panic_get_callback,
            grow_memory: panic_grow_callback,
            finalizer: None,
        };
        let cb = self.new_memory;
        let error = cb(
            self.foreign.data,
            &wasm_memorytype_t::new(ty),
            minimum,
            maximum.unwrap_or(usize::MAX),
            reserved_size_in_bytes.unwrap_or(0),
            guard_size_in_bytes,
            &mut memory,
        );
        match error {
            None => {
                let foreign = crate::ForeignData {
                    data: memory.env,
                    finalizer: memory.finalizer,
                };
                Ok(Box::new(CHostLinearMemory {
                    foreign,
                    get_memory: memory.get_memory,
                    grow_memory: memory.grow_memory,
                }))
            }
            Some(err) => {
                let err: anyhow::Error = (*err).into();
                Err(format!("{err}"))
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_config_host_memory_creator_set(
    c: &mut wasm_config_t,
    creator: &wasmtime_memory_creator_t,
) {
    c.config.with_host_memory(Arc::new(CHostMemoryCreator {
        foreign: crate::ForeignData {
            data: creator.env,
            finalizer: creator.finalizer,
        },
        new_memory: creator.new_memory,
    }));
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_memory_init_cow_set(c: &mut wasm_config_t, enable: bool) {
    c.config.memory_init_cow(enable);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_config_wasm_wide_arithmetic_set(c: &mut wasm_config_t, enable: bool) {
    c.config.wasm_wide_arithmetic(enable);
}

#[repr(C)]
#[derive(Clone)]
#[cfg(feature = "pooling-allocator")]
pub struct wasmtime_pooling_allocation_config_t {
    pub(crate) config: PoolingAllocationConfig,
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_new()
-> Box<wasmtime_pooling_allocation_config_t> {
    Box::new(wasmtime_pooling_allocation_config_t {
        config: PoolingAllocationConfig::default(),
    })
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_delete(
    _: Box<wasmtime_pooling_allocation_config_t>,
) {
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_max_unused_warm_slots_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    max: u32,
) {
    c.config.max_unused_warm_slots(max);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_decommit_batch_size_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    batch_size: usize,
) {
    c.config.decommit_batch_size(batch_size);
}

#[unsafe(no_mangle)]
#[cfg(all(feature = "pooling-allocator", feature = "async"))]
pub extern "C" fn wasmtime_pooling_allocation_config_async_stack_keep_resident_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    size: usize,
) {
    c.config.async_stack_keep_resident(size);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_linear_memory_keep_resident_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    size: usize,
) {
    c.config.linear_memory_keep_resident(size);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_table_keep_resident_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    size: usize,
) {
    c.config.table_keep_resident(size);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_total_component_instances_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    count: u32,
) {
    c.config.total_component_instances(count);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_max_component_instance_size_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    size: usize,
) {
    c.config.max_component_instance_size(size);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_max_core_instances_per_component_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    count: u32,
) {
    c.config.max_core_instances_per_component(count);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_max_memories_per_component_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    count: u32,
) {
    c.config.max_memories_per_component(count);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_max_tables_per_component_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    count: u32,
) {
    c.config.max_tables_per_component(count);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_total_memories_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    count: u32,
) {
    c.config.total_memories(count);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_total_tables_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    count: u32,
) {
    c.config.total_tables(count);
}

#[unsafe(no_mangle)]
#[cfg(all(feature = "pooling-allocator", feature = "async"))]
pub extern "C" fn wasmtime_pooling_allocation_config_total_stacks_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    count: u32,
) {
    c.config.total_stacks(count);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_total_core_instances_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    count: u32,
) {
    c.config.total_core_instances(count);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_max_core_instance_size_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    size: usize,
) {
    c.config.max_core_instance_size(size);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_max_tables_per_module_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    tables: u32,
) {
    c.config.max_tables_per_module(tables);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_table_elements_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    elements: usize,
) {
    c.config.table_elements(elements);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_max_memories_per_module_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    memories: u32,
) {
    c.config.max_memories_per_module(memories);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_config_max_memory_size_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    bytes: usize,
) {
    c.config.max_memory_size(bytes);
}

#[unsafe(no_mangle)]
#[cfg(all(feature = "pooling-allocator", feature = "gc"))]
pub extern "C" fn wasmtime_pooling_allocation_config_total_gc_heaps_set(
    c: &mut wasmtime_pooling_allocation_config_t,
    count: u32,
) {
    c.config.total_gc_heaps(count);
}

#[unsafe(no_mangle)]
#[cfg(feature = "pooling-allocator")]
pub extern "C" fn wasmtime_pooling_allocation_strategy_set(
    c: &mut wasm_config_t,
    pc: &wasmtime_pooling_allocation_config_t,
) {
    c.config
        .allocation_strategy(InstanceAllocationStrategy::Pooling(pc.config.clone()));
}
