use std::{
    collections::HashMap, fs::File, future::Future, marker::PhantomData, path::Path, pin::Pin,
};

use anyhow::{anyhow, Error};
use serde::{de::DeserializeOwned, Serialize};

use wasi_common::{dir, file};
use wasmtime::{
    AsContext, AsContextMut, Caller, Config, Engine, Extern, Instance, Linker, Module, Store,
    StoreContext, StoreContextMut, Trap, TypedFunc,
};
use wasmtime::{IntoFunc, Memory};
use wasmtime_wasi::{Dir, WasiCtx, WasiCtxBuilder};

pub struct WasiResource(u32);

pub struct WasiFn<A: Serialize, R: DeserializeOwned> {
    function: TypedFunc<(u32, u32), u32>,
    _function_type: PhantomData<fn(A) -> R>,
}

impl<A: Serialize, R: DeserializeOwned> Copy for WasiFn<A, R> {}

impl<A: Serialize, R: DeserializeOwned> Clone for WasiFn<A, R> {
    fn clone(&self) -> Self {
        Self {
            function: self.function,
            _function_type: PhantomData,
        }
    }
}

// impl<A: Serialize, R: DeserializeOwned> WasiFn<A, R> {
//     #[inline(always)]
//     pub async fn call(&self, runtime: &mut Wasi, arg: A) -> Result<R, Error> {
//         runtime.call(self, arg).await
//     }
// }

// type signature derived from:
// https://docs.rs/wasmtime/latest/wasmtime/struct.Linker.html#method.func_wrap2_async
// macro_rules! dynHostFunction {
//     () => {
//         Box<
//             dyn for<'a> Fn(Caller<'a, WasiCtx>, u32, u32)
//                 -> Box<dyn Future<Output = u32> + Send + 'a>
//                     + Send
//                     + Sync
//                     + 'static
//         >
//     };
// }

// macro_rules! implHostFunction {
//     () => {
//         impl for<'a> Fn(Caller<'a, WasiCtx>, u32, u32)
//             -> Box<dyn Future<Output = u32> + Send + 'a>
//                 + Send
//                 + Sync
//                 + 'static
//     };
// }

pub struct WasiPluginBuilder {
    wasi_ctx: WasiCtx,
    engine: Engine,
    linker: Linker<WasiCtxAlloc>,
}

impl WasiPluginBuilder {
    pub fn new(wasi_ctx: WasiCtx) -> Result<Self, Error> {
        let mut config = Config::default();
        config.async_support(true);
        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);

        Ok(WasiPluginBuilder {
            // host_functions: HashMap::new(),
            wasi_ctx,
            engine,
            linker,
        })
    }

    pub fn new_with_default_ctx() -> Result<Self, Error> {
        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdin()
            .inherit_stderr()
            .build();
        Self::new(wasi_ctx)
    }

    pub fn host_function<A: DeserializeOwned, R: Serialize>(
        mut self,
        name: &str,
        function: impl Fn(A) -> R + Send + Sync + 'static,
    ) -> Result<Self, Error> {
        self.linker.func_wrap(
            "env",
            name,
            move |mut caller: Caller<'_, WasiCtxAlloc>, ptr: u32, len: u32| {
                let mut plugin_memory = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => return Err(Trap::new("Could not grab slice of plugin memory")),
                };
                let args = Wasi::deserialize_from_buffer(&mut plugin_memory, &caller, ptr, len)?;

                let result = function(args);
                let buffer = Wasi::serialize_to_buffer(
                    caller.data().alloc_buffer(),
                    &mut plugin_memory,
                    &mut caller,
                    result,
                )
                .await;

                Ok(7u32)
            },
        )?;
        Ok(self)
    }

    pub async fn init<T: AsRef<[u8]>>(self, module: T) -> Result<Wasi, Error> {
        Wasi::init(module.as_ref().to_vec(), self).await
    }
}

// // TODO: remove
// /// Represents a to-be-initialized plugin.
// /// Please use [`WasiPluginBuilder`], don't use this directly.
// pub struct WasiPlugin {
//     pub module: Vec<u8>,
//     pub wasi_ctx: WasiCtx,
//     pub host_functions:
//         HashMap<String, Box<dyn Fn(&str, &mut Linker<WasiCtx>) -> Result<(), Error>>>,
// }

#[derive(Copy, Clone)]
struct WasiAlloc {
    alloc_buffer: TypedFunc<u32, u32>,
    free_buffer: TypedFunc<u32, u32>,
}

struct WasiCtxAlloc {
    wasi_ctx: WasiCtx,
    alloc: Option<WasiAlloc>,
}

impl WasiCtxAlloc {
    fn alloc_buffer(&self) -> TypedFunc<u32, u32> {
        self.alloc
            .expect("allocator has been not initialized, cannot allocate buffer!")
            .alloc_buffer
    }

    fn free_buffer(&self) -> TypedFunc<u32, u32> {
        self.alloc
            .expect("allocator has been not initialized, cannot free buffer!")
            .free_buffer
    }

    fn init_alloc(&mut self, alloc: WasiAlloc) {
        self.alloc = Some(alloc)
    }
}

pub struct Wasi {
    engine: Engine,
    module: Module,
    store: Store<WasiCtxAlloc>,
    instance: Instance,
}

impl Wasi {
    pub fn dump_memory(data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            if i % 32 == 0 {
                println!();
            }
            if i % 4 == 0 {
                print!("|");
            }
            if *byte == 0 {
                print!("__")
            } else {
                print!("{:02x}", byte);
            }
        }
        println!();
    }
}

impl Wasi {
    async fn init(module: Vec<u8>, plugin: WasiPluginBuilder) -> Result<Self, Error> {
        // initialize the WebAssembly System Interface context
        let engine = plugin.engine;
        let mut linker = plugin.linker;
        wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi_ctx)?;

        // create a store, note that we can't initialize the allocator,
        // because we can't grab the functions until initialized.
        let mut store: Store<WasiCtxAlloc> = Store::new(
            &engine,
            WasiCtxAlloc {
                wasi_ctx: plugin.wasi_ctx,
                alloc: None,
            },
        );
        let module = Module::new(&engine, module)?;

        // load the provided module into the asynchronous runtime
        linker.module_async(&mut store, "", &module).await?;
        let instance = linker.instantiate_async(&mut store, &module).await?;

        // now that the module is initialized,
        // we can initialize the store's allocator
        let alloc_buffer = instance.get_typed_func(&mut store, "__alloc_buffer")?;
        let free_buffer = instance.get_typed_func(&mut store, "__free_buffer")?;
        store.data_mut().init_alloc(WasiAlloc {
            alloc_buffer,
            free_buffer,
        });

        Ok(Wasi {
            engine,
            module,
            store,
            instance,
        })
    }

    /// Attaches a file or directory the the given system path to the runtime.
    /// Note that the resource must be freed by calling `remove_resource` afterwards.
    pub fn attach_path<T: AsRef<Path>>(&mut self, path: T) -> Result<WasiResource, Error> {
        // grab the WASI context
        let ctx = self.store.data_mut();

        // open the file we want, and convert it into the right type
        // this is a footgun and a half
        let file = File::open(&path).unwrap();
        let dir = Dir::from_std_file(file);
        let dir = Box::new(wasmtime_wasi::dir::Dir::from_cap_std(dir));

        // grab an empty file descriptor, specify capabilities
        let fd = ctx.wasi_ctx.table().push(Box::new(()))?;
        let caps = dir::DirCaps::all();
        let file_caps = file::FileCaps::all();

        // insert the directory at the given fd,
        // return a handle to the resource
        ctx.wasi_ctx
            .insert_dir(fd, dir, caps, file_caps, path.as_ref().to_path_buf());
        Ok(WasiResource(fd))
    }

    /// Returns `true` if the resource existed and was removed.
    pub fn remove_resource(&mut self, resource: WasiResource) -> Result<(), Error> {
        self.store
            .data_mut()
            .wasi_ctx
            .table()
            .delete(resource.0)
            .ok_or_else(|| anyhow!("Resource did not exist, but a valid handle was passed in"))?;
        Ok(())
    }

    // pub fn with_resource<T>(
    //     &mut self,
    //     resource: WasiResource,
    //     callback: fn(&mut Self) -> Result<T, Error>,
    // ) -> Result<T, Error> {
    //     let result = callback(self);
    //     self.remove_resource(resource)?;
    //     return result;
    // }

    // So this call function is kinda a dance, I figured it'd be a good idea to document it.
    // the high level is we take a serde type, serialize it to a byte array,
    // (we're doing this using bincode for now)
    // then toss that byte array into webassembly.
    // webassembly grabs that byte array, does some magic,
    // and serializes the result into yet another byte array.
    // we then grab *that* result byte array and deserialize it into a result.
    //
    // phew...
    //
    // now the problem is, webassambly doesn't support buffers.
    // only really like i32s, that's it (yeah, it's sad. Not even unsigned!)
    // (ok, I'm exaggerating a bit).
    //
    // the Wasm function that this calls must have a very specific signature:
    //
    // fn(pointer to byte array: i32, length of byte array: i32)
    //     -> pointer to (
    //            pointer to byte_array: i32,
    //            length of byte array: i32,
    //     ): i32
    //
    // This pair `(pointer to byte array, length of byte array)` is called a `Buffer`
    // and can be found in the cargo_test plugin.
    //
    // so on the wasm side, we grab the two parameters to the function,
    // stuff them into a `Buffer`,
    // and then pray to the `unsafe` Rust gods above that a valid byte array pops out.
    //
    // On the flip side, when returning from a wasm function,
    // we convert whatever serialized result we get into byte array,
    // which we stuff into a Buffer and allocate on the heap,
    // which pointer to we then return.
    // Note the double indirection!
    //
    // So when returning from a function, we actually leak memory *twice*:
    //
    // 1) once when we leak the byte array
    // 2) again when we leak the allocated `Buffer`
    //
    // This isn't a problem because Wasm stops executing after the function returns,
    // so the heap is still valid for our inspection when we want to pull things out.

    /// Takes an item, allocates a buffer, serializes the argument to that buffer,
    /// and returns a (ptr, len) pair to that buffer.
    async fn serialize_to_buffer<A: Serialize>(
        alloc_buffer: TypedFunc<u32, u32>,
        plugin_memory: &mut Memory,
        mut store: impl AsContextMut<Data = WasiCtxAlloc>,
        item: A,
    ) -> Result<(u32, u32), Error> {
        // serialize the argument using bincode
        let item = bincode::serialize(&item)?;
        let buffer_len = item.len() as u32;

        // allocate a buffer and write the argument to that buffer
        let buffer_ptr = alloc_buffer.call_async(&mut store, buffer_len).await?;
        plugin_memory.write(&mut store, buffer_ptr as usize, &item)?;
        Ok((buffer_ptr, buffer_len))
    }

    /// Takes `ptr to a `(ptr, len)` pair, and returns `(ptr, len)`.
    fn deref_buffer(
        plugin_memory: &mut Memory,
        store: impl AsContext<Data = WasiCtxAlloc>,
        buffer: u32,
    ) -> Result<(u32, u32), Error> {
        // create a buffer to read the (ptr, length) pair into
        // this is a total of 4 + 4 = 8 bytes.
        let raw_buffer = &mut [0; 8];
        plugin_memory.read(store, buffer as usize, raw_buffer)?;

        // use these bytes (wasm stores things little-endian)
        // to get a pointer to the buffer and its length
        let b = raw_buffer;
        let buffer_ptr = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        let buffer_len = u32::from_le_bytes([b[4], b[5], b[6], b[7]]);

        return Ok((buffer_ptr, buffer_len));
    }

    /// Takes a `(ptr, len)` pair and returns the corresponding deserialized buffer.
    fn deserialize_from_buffer<R: DeserializeOwned>(
        plugin_memory: &mut Memory,
        store: impl AsContext<Data = WasiCtxAlloc>,
        buffer_ptr: u32,
        buffer_len: u32,
    ) -> Result<R, Error> {
        let buffer_ptr = buffer_ptr as usize;
        let buffer_len = buffer_len as usize;
        let buffer_end = buffer_ptr + buffer_len;

        // read the buffer at this point into a byte array
        // deserialize the byte array into the provided serde type
        let result = &plugin_memory.data(store.as_context())[buffer_ptr..buffer_end];
        let result = bincode::deserialize(result)?;

        // TODO: this is handled wasm-side
        // // deallocate the argument buffer
        // self.free_buffer.call(&mut self.store, arg_buffer);

        Ok(result)
    }

    pub fn function<A: Serialize, R: DeserializeOwned, T: AsRef<str>>(
        &mut self,
        name: T,
    ) -> Result<WasiFn<A, R>, Error> {
        let fun_name = format!("__{}", name.as_ref());
        let fun = self
            .instance
            .get_typed_func::<(u32, u32), u32, _>(&mut self.store, &fun_name)?;
        Ok(WasiFn {
            function: fun,
            _function_type: PhantomData,
        })
    }

    // TODO: dont' use as for conversions
    pub async fn call<A: Serialize, R: DeserializeOwned>(
        &mut self,
        handle: &WasiFn<A, R>,
        arg: A,
    ) -> Result<R, Error> {
        // dbg!(&handle.name);
        // dbg!(serde_json::to_string(&arg)).unwrap();

        let mut plugin_memory = self
            .instance
            .get_memory(&mut self.store, "memory")
            .ok_or_else(|| anyhow!("Could not grab slice of plugin memory"))?;

        // write the argument to linear memory
        // this returns a (ptr, lentgh) pair
        let arg_buffer = Self::serialize_to_buffer(
            self.store.data().alloc_buffer(),
            &mut plugin_memory,
            &mut self.store,
            arg,
        )
        .await?;

        // call the function, passing in the buffer and its length
        // this returns a ptr to a (ptr, lentgh) pair
        let result_buffer = handle
            .function
            .call_async(&mut self.store, arg_buffer)
            .await?;
        let (result_buffer_ptr, result_buffer_len) =
            Self::deref_buffer(&mut plugin_memory, &mut self.store, result_buffer)?;

        Self::deserialize_from_buffer(
            &mut plugin_memory,
            &mut self.store,
            result_buffer_ptr,
            result_buffer_len,
        )
    }
}
