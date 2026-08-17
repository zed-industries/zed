use crate::{AtlasAllocator, size2};
use std::os::raw::c_char;

/// 1 means OK, 0 means error.
pub type EtagereStatus = u32;
pub type EtagereAllocationId = u32;


#[repr(C)]
pub struct EtagereAllocation {
    pub rectangle: EtagereRectangle,
    pub id: EtagereAllocationId,
}

#[repr(C)]
pub struct EtagereRectangle {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,    
}


#[repr(C)]
pub struct EtagereAllocatorOptions {
    pub width_alignment: i32,
    pub height_alignment: i32,
    pub num_columns: i32,
    pub flags: u32,
}

pub const ETAGERE_FLAGS_VERTICAL_SHELVES: u32 = 1;

#[no_mangle]
pub unsafe extern "C" fn etagere_atlas_allocator_new(width: i32, height: i32) -> *mut AtlasAllocator {
    Box::into_raw(Box::new(AtlasAllocator::new(size2(width, height))))
}

#[no_mangle]
pub unsafe extern "C" fn etagere_atlas_allocator_with_options(
    width: i32,
    height: i32,
    options: &EtagereAllocatorOptions
) -> *mut AtlasAllocator {
    let options = crate::AllocatorOptions {
        alignment: size2(options.width_alignment, options.height_alignment),
        num_columns: options.num_columns,
        vertical_shelves: (options.flags & ETAGERE_FLAGS_VERTICAL_SHELVES) != 0,
    };
    Box::into_raw(Box::new(AtlasAllocator::with_options(size2(width, height), &options)))
}

#[no_mangle]
pub unsafe extern "C" fn etagere_atlas_allocator_delete(allocator: *mut AtlasAllocator)  {
    let _ = Box::from_raw(allocator);
}

#[no_mangle]
pub unsafe extern "C" fn etagere_atlas_allocator_allocate(allocator: &mut AtlasAllocator, width: i32, height: i32, allocation: &mut EtagereAllocation) -> EtagereStatus {
    match allocator.allocate(size2(width, height)) {
        Some(alloc) => {
            allocation.id = alloc.id.serialize();
            allocation.rectangle.min_x = alloc.rectangle.min.x;
            allocation.rectangle.min_y = alloc.rectangle.min.y;
            allocation.rectangle.max_x = alloc.rectangle.max.x;
            allocation.rectangle.max_y = alloc.rectangle.max.y;
            return 1;
        }
        None => {
            allocation.id = std::u32::MAX;
            allocation.rectangle.min_x = 0;
            allocation.rectangle.min_y = 0;
            allocation.rectangle.max_x = 0;
            allocation.rectangle.max_y = 0;
            return 0;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn etagere_atlas_allocator_deallocate(allocator: &mut AtlasAllocator, id: EtagereAllocationId) {
    allocator.deallocate(crate::AllocId::deserialize(id));
}

#[no_mangle]
pub unsafe extern "C" fn etagere_atlas_allocator_clear(allocator: &mut AtlasAllocator) {
    allocator.clear();
}

#[no_mangle]
pub unsafe extern "C" fn etagere_atlas_allocator_allocated_space(allocator: &AtlasAllocator) -> i32 {
    allocator.allocated_space()
}

#[no_mangle]
pub unsafe extern "C" fn etagere_atlas_allocator_free_space(allocator: &AtlasAllocator) -> i32 {
    allocator.free_space()
}

#[no_mangle]
pub unsafe extern "C" fn etagere_atlas_allocator_get(allocator: &AtlasAllocator, id: EtagereAllocationId) -> EtagereRectangle {
    let rect = allocator.get(crate::AllocId::deserialize(id));
    return EtagereRectangle {
        min_x: rect.min.x,
        min_y: rect.min.y,
        max_x: rect.max.x,
        max_y: rect.max.y,
    };
}

#[no_mangle]
pub unsafe extern "C" fn etagere_atlas_allocator_dump_svg(allocator: &AtlasAllocator, file_name: *const c_char) -> EtagereStatus {
    let cstr = std::ffi::CStr::from_ptr(file_name);
    let rstr = String::from_utf8_lossy(cstr.to_bytes());
    let mut file = match std::fs::File::create(rstr.as_ref()) {
        Ok(file) => file,
        Err(..) => {
            return 0;
        }
    };

    match allocator.dump_svg(&mut file) {
        Ok(..) => 1,
        Err(..) => 0, 
    }
}
