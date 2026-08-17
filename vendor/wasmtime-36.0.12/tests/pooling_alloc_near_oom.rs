use libtest_mimic::Arguments;
use wasmtime::Result;

fn main() -> Result<()> {
    let mut trials = Vec::new();

    #[cfg(unix)]
    if !cfg!(miri) && !cfg!(asan) && std::env::var("WASMTIME_TEST_NO_HOG_MEMORY").is_err() {
        for (name, test) in unix::TESTS {
            trials.push(libtest_mimic::Trial::test(*name, || {
                test().map_err(|e| format!("{e:?}").into())
            }));
        }
    }
    let _ = &mut trials;

    let mut args = Arguments::from_args();
    // When testing memory exhaustion scenarios just test one at a time
    args.test_threads = Some(1);
    libtest_mimic::run(&args, trials).exit()
}

#[cfg(unix)]
mod unix {
    use rustix::mm::*;
    use std::ffi::c_void;
    use std::ptr;
    use wasmtime::*;

    pub const TESTS: &[(&str, fn() -> Result<()>)] = &[("exhausted_vmas", exhausted_vmas)];

    fn exhausted_vmas() -> Result<()> {
        // Non-Linux environments have different behavior around OOM and
        // overcommit it seems. For example macOS looks to wedge the CI runner
        // if it runs this tests. For now only run on Linux.
        if !cfg!(target_os = "linux") {
            return Ok(());
        }

        // Create a small-ish pooling allocator which can hold a number of
        // instances with memories, but we won't end up using all of them.
        let mut pooling = PoolingAllocationConfig::new();
        pooling.total_memories(100);
        pooling.total_tables(0);
        pooling.total_stacks(0);
        let mut config = Config::new();
        config.allocation_strategy(pooling);
        let engine = Engine::new(&config)?;

        // Create a module which has a CoW image to try to increase the number
        // of VMAs in use.
        let module = Module::new(
            &engine,
            r#"(module
                (memory 1)
                (data (i32.const 1) "\aa")
            )"#,
        )?;

        // Eat up all kernel VMAs we're allowed in this process. This makes a
        // lot of little tiny mmaps which force usage of a VMA by having a
        // readable region in the middle of an otherwise unmapped region.
        let mut vma_hogs = Vec::new();
        while let Ok(hog) = VmaHog::new() {
            vma_hogs.push(hog);
        }

        // Now that our process has reached it's VMA limit, or it's at least
        // close to it, start allocating some instances. Do this until failure
        // happens.
        let mut stores = Vec::new();
        loop {
            let mut store = Store::new(&engine, ());
            if Instance::new(&mut store, &module, &[]).is_ok() {
                stores.push(store);
            } else {
                break;
            }
        }

        // At this point if nothing has panicked then the test has passed.
        // Destructors will clean everything else up.

        Ok(())
    }

    struct VmaHog {
        addr: *mut c_void,
        size: usize,
    }

    impl VmaHog {
        fn new() -> Result<VmaHog> {
            unsafe {
                let page_size = rustix::param::page_size();
                let addr = mmap_anonymous(
                    ptr::null_mut(),
                    page_size * 3,
                    ProtFlags::empty(),
                    MapFlags::PRIVATE,
                )?;
                let ret = VmaHog {
                    addr,
                    size: page_size * 3,
                };
                mprotect(ret.addr.byte_add(page_size), page_size, MprotectFlags::READ)?;
                Ok(ret)
            }
        }
    }

    impl Drop for VmaHog {
        fn drop(&mut self) {
            unsafe {
                munmap(self.addr, self.size).unwrap();
            }
        }
    }
}
