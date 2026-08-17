use crate::{get_stack_limit, set_stack_limit};

pub struct StackRestoreGuard {
    mapping: *mut u8,
    size_with_guard: usize,
    page_size: usize,
    old_stack_limit: Option<usize>,
}

impl StackRestoreGuard {
    pub fn new(requested_size: usize) -> StackRestoreGuard {
        // For maximum portability we want to produce a stack that is aligned to a page and has
        // a size that’s a multiple of page size. It is natural to use mmap to allocate
        // these pages. Furthermore, we want to allocate two extras pages for the stack guard.
        // To achieve that we do our calculations in number of pages and convert to bytes last.
        let page_size = page_size();
        let requested_pages = requested_size
            .checked_add(page_size - 1)
            .expect("unreasonably large stack requested")
            / page_size;
        let page_count_with_guard = std::cmp::max(1, requested_pages) + 2;
        let size_with_guard = page_count_with_guard
            .checked_mul(page_size)
            .expect("unreasonably large stack requested");

        unsafe {
            let new_stack = libc::mmap(
                std::ptr::null_mut(),
                size_with_guard,
                libc::PROT_NONE,
                libc::MAP_PRIVATE | libc::MAP_ANON,
                -1, // Some implementations assert fd = -1 if MAP_ANON is specified
                0,
            );
            assert_ne!(
                new_stack,
                libc::MAP_FAILED,
                "mmap failed to allocate stack: {}",
                std::io::Error::last_os_error()
            );
            let guard = StackRestoreGuard {
                mapping: new_stack as *mut u8,
                page_size,
                size_with_guard,
                old_stack_limit: get_stack_limit(),
            };
            // We leave two guard pages without read/write access in our allocation.
            // There is one guard page below the stack and another above it.
            let above_guard_page = new_stack.add(page_size);
            #[cfg(not(target_os = "openbsd"))]
            let result = libc::mprotect(
                above_guard_page,
                size_with_guard - 2 * page_size,
                libc::PROT_READ | libc::PROT_WRITE,
            );
            #[cfg(target_os = "openbsd")]
            let result = if libc::mmap(
                above_guard_page,
                size_with_guard - 2 * page_size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_FIXED | libc::MAP_PRIVATE | libc::MAP_ANON | libc::MAP_STACK,
                -1,
                0,
            ) == above_guard_page
            {
                0
            } else {
                -1
            };
            assert_ne!(
                result,
                -1,
                "mprotect/mmap failed: {}",
                std::io::Error::last_os_error()
            );
            guard
        }
    }

    // TODO this should return a *mut [u8], but pointer slices only got proper support with Rust 1.79.
    pub fn stack_area(&self) -> (*mut u8, usize) {
        unsafe {
            (
                self.mapping.add(self.page_size),
                self.size_with_guard - 2 * self.page_size,
            )
        }
    }
}

impl Drop for StackRestoreGuard {
    fn drop(&mut self) {
        unsafe {
            // FIXME: check the error code and decide what to do with it.
            // Perhaps a debug_assertion?
            libc::munmap(self.mapping as *mut std::ffi::c_void, self.size_with_guard);
        }
        set_stack_limit(self.old_stack_limit);
    }
}

fn page_size() -> usize {
    // FIXME: consider caching the page size.
    unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) as usize }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_area() {
        for stack_size_kb in 1..64 {
            let size = stack_size_kb * 1024;
            let stack = StackRestoreGuard::new(size);
            let (mut ptr, actual_size) = stack.stack_area();
            for _ in 0..actual_size {
                unsafe {
                    core::ptr::write_volatile(ptr, 0b10101011);
                    ptr = ptr.add(1)
                }
            }
            assert_eq!(
                actual_size,
                size.div_ceil(page_size()) * page_size()
            )
        }
    }
}