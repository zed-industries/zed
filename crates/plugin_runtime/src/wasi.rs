use std::{
    collections::HashMap, fs::File, future::Future, marker::PhantomData, path::Path, pin::Pin,
};

use anyhow::{anyhow, Error};
use serde::{de::DeserializeOwned, Serialize};

use wasi_common::{dir, file};
use wasmtime::IntoFunc;
use wasmtime::{Caller, Config, Engine, Instance, Linker, Module, Store, TypedFunc};
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

pub struct Wasi {
    engine: Engine,
    module: Module,
    store: Store<WasiCtx>,
    instance: Instance,
    alloc_buffer: TypedFunc<u32, u32>,
    // free_buffer: TypedFunc<(u32, u32), ()>,
}

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

// This type signature goodness gracious
pub type HostFunction = Box<dyn IntoFunc<WasiCtx, (u32, u32), u32>>;

pub struct WasiPluginBuilder {
    host_functions: HashMap<String, Box<dyn Fn(&str, &mut Linker<WasiCtx>) -> Result<(), Error>>>,
    wasi_ctx_builder: WasiCtxBuilder,
}

impl WasiPluginBuilder {
    pub fn new() -> Self {
        WasiPluginBuilder {
            host_functions: HashMap::new(),
            wasi_ctx_builder: WasiCtxBuilder::new(),
        }
    }

    pub fn new_with_default_ctx() -> WasiPluginBuilder {
        let mut this = Self::new();
        this.wasi_ctx_builder = this.wasi_ctx_builder.inherit_stdin().inherit_stderr();
        this
    }

    pub fn host_function<A: Serialize, R: DeserializeOwned>(
        mut self,
        name: &str,
        function: &dyn Fn(A) -> R + Send + Sync + 'static,
    ) -> Self {
        let name = name.to_string();
        self.host_functions.insert(
            name,
            Box::new(move |name: &str, linker: &mut Linker<WasiCtx>| {
                linker.func_wrap("env", name, |ptr: u32, len: u32| {
                    function(todo!());
                    7u32
                })?;
                Ok(())
            }),
        );
        self
    }

    pub fn wasi_ctx(mut self, config: impl FnOnce(WasiCtxBuilder) -> WasiCtxBuilder) -> Self {
        self.wasi_ctx_builder = config(self.wasi_ctx_builder);
        self
    }

    pub async fn init<T: AsRef<[u8]>>(self, module: T) -> Result<Wasi, Error> {
        let plugin = WasiPlugin {
            module: module.as_ref().to_vec(),
            wasi_ctx: self.wasi_ctx_builder.build(),
            host_functions: self.host_functions,
        };

        Wasi::init(plugin).await
    }
}

/// Represents a to-be-initialized plugin.
/// Please use [`WasiPluginBuilder`], don't use this directly.
pub struct WasiPlugin {
    pub module: Vec<u8>,
    pub wasi_ctx: WasiCtx,
    pub host_functions:
        HashMap<String, Box<dyn Fn(&str, &mut Linker<WasiCtx>) -> Result<(), Error>>>,
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
    async fn init(plugin: WasiPlugin) -> Result<Self, Error> {
        let mut config = Config::default();
        config.async_support(true);
        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);

        for (name, add_to_linker) in plugin.host_functions.into_iter() {
            add_to_linker(&name, &mut linker)?;
        }

        linker
            .func_wrap("env", "__command", |x: u32, y: u32| x + y)
            .unwrap();
        linker.func_wrap("env", "__hello", |x: u32| x * 2).unwrap();
        linker.func_wrap("env", "__bye", |x: u32| x / 2).unwrap();

        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        let mut store: Store<_> = Store::new(&engine, plugin.wasi_ctx);
        let module = Module::new(&engine, plugin.module)?;

        linker.module_async(&mut store, "", &module).await?;
        let instance = linker.instantiate_async(&mut store, &module).await?;

        let alloc_buffer = instance.get_typed_func(&mut store, "__alloc_buffer")?;
        // let free_buffer = instance.get_typed_func(&mut store, "__free_buffer")?;

        Ok(Wasi {
            engine,
            module,
            store,
            instance,
            alloc_buffer,
            // free_buffer,
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
        let fd = ctx.table().push(Box::new(()))?;
        let caps = dir::DirCaps::all();
        let file_caps = file::FileCaps::all();

        // insert the directory at the given fd,
        // return a handle to the resource
        ctx.insert_dir(fd, dir, caps, file_caps, path.as_ref().to_path_buf());
        Ok(WasiResource(fd))
    }

    /// Returns `true` if the resource existed and was removed.
    pub fn remove_resource(&mut self, resource: WasiResource) -> Result<(), Error> {
        self.store
            .data_mut()
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
    async fn serialize_to_buffer<T: Serialize>(&mut self, item: T) -> Result<(u32, u32), Error> {
        // serialize the argument using bincode
        let item = bincode::serialize(&item)?;
        let buffer_len = item.len() as u32;

        // allocate a buffer and write the argument to that buffer
        let buffer_ptr = self
            .alloc_buffer
            .call_async(&mut self.store, buffer_len)
            .await?;
        let plugin_memory = self
            .instance
            .get_memory(&mut self.store, "memory")
            .ok_or_else(|| anyhow!("Could not grab slice of plugin memory"))?;
        plugin_memory.write(&mut self.store, buffer_ptr as usize, &item)?;
        Ok((buffer_ptr, buffer_len))
    }

    /// Takes a ptr to a (ptr, len) pair and returns the corresponding deserialized buffer
    fn deserialize_from_buffer<R: DeserializeOwned>(&mut self, buffer: u32) -> Result<R, Error> {
        // create a buffer to read the (ptr, length) pair into
        // this is a total of 4 + 4 = 8 bytes.
        let raw_buffer = &mut [0; 8];
        let plugin_memory = self
            .instance
            .get_memory(&mut self.store, "memory")
            .ok_or_else(|| anyhow!("Could not grab slice of plugin memory"))?;
        plugin_memory.read(&mut self.store, buffer as usize, raw_buffer)?;

        // use these bytes (wasm stores things little-endian)
        // to get a pointer to the buffer and its length
        let b = raw_buffer;
        let buffer_ptr = u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize;
        let buffer_len = u32::from_le_bytes([b[4], b[5], b[6], b[7]]) as usize;
        let buffer_end = buffer_ptr + buffer_len;

        // read the buffer at this point into a byte array
        // deserialize the byte array into the provided serde type
        let result = &plugin_memory.data(&mut self.store)[buffer_ptr..buffer_end];
        let result = bincode::deserialize(result)?;

        // TODO: this is handled wasm-side, but I'd like to double-check
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

        // write the argument to linear memory
        // this returns a (ptr, lentgh) pair
        let arg_buffer = self.serialize_to_buffer(arg).await?;

        // get the webassembly function we want to actually call
        // TODO: precompute handle
        // let fun_name = format!("__{}", handle);
        // let fun = self
        //     .instance
        //     .get_typed_func::<(u32, u32), u32, _>(&mut self.store, &fun_name)?;
        let fun = handle.function;

        // call the function, passing in the buffer and its length
        // this returns a ptr to a (ptr, lentgh) pair
        let result_buffer = fun.call_async(&mut self.store, arg_buffer).await?;

        self.deserialize_from_buffer(result_buffer)
    }
}
